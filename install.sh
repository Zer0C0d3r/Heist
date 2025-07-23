#!/usr/bin/env bash
# Heist Interactive Installer/Uninstaller (Colorful & Animated)
set -e

APP_NAME="heist"
INSTALL_DIR="/usr/local/bin"
CARGO_BIN="${HOME}/.cargo/bin/${APP_NAME}"
VERSION="v1.0.0"

# Colors
if [ ! -t 1 ]; then
    GREEN=''
    RED=''
    YELLOW=''
    CYAN=''
    MAGENTA=''
    NC=''
    BOLD=''
    CLEAR=''
    TPUT_CLEAR=':'
else
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    MAGENTA='\033[0;35m'
    NC='\033[0m'
    BOLD='\033[1m'
    CLEAR='clear'
    TPUT_CLEAR='tput clear'
fi

# Spinner animation
spinner() {
    local pid=$1
    local msg="$2"
    local spin='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    local i=0
    tput civis
    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) % 10 ))
        printf "\r${CYAN}%s${NC} %s" "${spin:$i:1}" "$msg"
        sleep 0.1
    done
    tput cnorm
    printf "\r"
}

# Banner
banner() {
    $CLEAR
    echo -e "${MAGENTA}${BOLD}"
    cat <<'EOF'
██╗  ██╗███████╗██╗███████╗████████╗
██║  ██║██╔════╝██║██╔════╝╚══██╔══╝
███████║█████╗  ██║███████╗   ██║   
██╔══██║██╔══╝  ██║╚════██║   ██║   
██║  ██║███████╗██║███████║   ██║   
╚═╝  ╚═╝╚══════╝╚═╝╚══════╝   ╚═╝   
EOF
    echo -e "${YELLOW}  Heist  ${CYAN}${VERSION}${NC}"
    echo -e "${CYAN}Heist Interactive Installer${NC}\n"
}

# Prompt
prompt_continue() {
    echo -en "${CYAN}Press Enter to continue...${NC}"
    read -r
}

# Dependency check
check_dep() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo -e "${RED}Missing dependency: $1${NC}"
        return 1
    fi
    return 0
}

# Uninstall
uninstall() {
    echo -e "${YELLOW}Uninstalling Heist...${NC}"
    sleep 0.3
    if [ -f "${INSTALL_DIR}/${APP_NAME}" ]; then
        if sudo rm -f "${INSTALL_DIR}/${APP_NAME}"; then
            echo -e "${GREEN}Heist removed from ${INSTALL_DIR}.${NC}"
        else
            echo -e "${RED}Failed to remove Heist from ${INSTALL_DIR}.${NC}"
        fi
    else
        echo -e "${RED}Heist is not installed in ${INSTALL_DIR}.${NC}"
    fi
    exit 0
}

# After install, offer to set up live tracking
setup_live_tracking() {
    echo -e "${CYAN}Heist can track commands in real time using PROMPT_COMMAND (Bash/Zsh).${NC}"
    echo -e "${YELLOW}This will append a snippet to your ~/.bashrc and/or ~/.zshrc to enable live tracking.${NC}"
    echo -en "Enable live tracking? [Y/n]: "
    read -r ans
    if [[ "$ans" =~ ^[Nn] ]]; then
        echo -e "${CYAN}Skipping live tracking setup.${NC}"
        return
    fi
    SNIPPET_PATH="$(pwd)/contrib/heist_live_tracking.sh"
    if [ ! -f "$SNIPPET_PATH" ]; then
        echo -e "${RED}Live tracking snippet not found: $SNIPPET_PATH${NC}"
        return
    fi
    for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
        if [ -f "$rc" ]; then
            if ! grep -q 'heist_live_tracking.sh' "$rc"; then
                echo -e "\n# Heist live tracking\nsource $SNIPPET_PATH" >> "$rc"
                echo -e "${GREEN}Appended live tracking snippet to $rc${NC}"
            else
                echo -e "${CYAN}Live tracking already set up in $rc${NC}"
            fi
        fi
    done
    echo -e "${GREEN}Live tracking enabled! Restart your shell to activate.${NC}"
}

# Install/Update
install() {
    echo -e "${GREEN}Building Heist in release mode...${NC}"
    (cargo build --release) &
    spinner $! "Compiling Rust project..."
    echo
    if [ ! -f "target/release/${APP_NAME}" ]; then
        echo -e "${RED}Build failed: target/release/${APP_NAME} not found.${NC}"
        exit 1
    fi
    echo -e "${GREEN}Installing to ${INSTALL_DIR}...${NC}"
    (sudo cp "target/release/${APP_NAME}" "${INSTALL_DIR}/") &
    spinner $! "Copying binary..."
    echo
    if command -v ${APP_NAME} >/dev/null 2>&1; then
        echo -e "${GREEN}Heist installed successfully! Run '${APP_NAME} --help' to get started.${NC}"
        setup_live_tracking
    else
        echo -e "${RED}Installation failed. Please check your PATH and try again.${NC}"
    fi
}

# Update Heist (like oh-my-zsh updater)
update() {
    echo -e "${CYAN}Checking for updates...${NC}"
    if [ ! -d .git ]; then
        echo -e "${RED}Not a git repository. Cannot update automatically.${NC}"
        return 1
    fi
    git fetch origin master &> /dev/null
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})
    BASE=$(git merge-base @ @{u})
    if [ "$LOCAL" = "$REMOTE" ]; then
        echo -e "${GREEN}Heist is already up to date!${NC}"
    elif [ "$LOCAL" = "$BASE" ]; then
        echo -e "${YELLOW}Updating Heist to latest version...${NC}"
        git pull --rebase --autostash
        install
        echo -e "${GREEN}Heist updated and reinstalled!${NC}"
    else
        echo -e "${RED}Local changes detected. Please commit or stash before updating.${NC}"
        return 1
    fi
}

# Main menu
main_menu() {
    while true; do
        banner
        echo -e "${CYAN}This script will build and install Heist globally, update, or uninstall it.${NC}"
        echo -e "${YELLOW}You may need sudo privileges to copy to ${INSTALL_DIR}.${NC}\n"
        echo -e "${CYAN}Options:${NC}"
        echo -e "  [1] Install Heist"
        echo -e "  [2] Uninstall Heist"
        echo -e "  [3] Update Heist"
        echo -e "  [q] Quit"
        echo -en "${BOLD}Choose an option:${NC} "
        read -r opt
        case "$opt" in
            1)
                echo -e "${GREEN}Proceeding with installation...${NC}"
                prompt_continue
                check_dep cargo || exit 1
                check_dep sudo || exit 1
                install
                prompt_continue
                ;;
            2)
                uninstall
                ;;
            3)
                update
                prompt_continue
                ;;
            q|Q)
                echo "Aborted."
                exit 0
                ;;
            *)
                echo -e "${RED}Invalid option. Try again.${NC}"
                sleep 1
                ;;
        esac
    done
}

main_menu
