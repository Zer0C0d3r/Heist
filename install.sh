#!/usr/bin/env bash
# Heist Interactive Installer/Uninstaller (Colorful & Animated)
set -e

APP_NAME="heist"
INSTALL_DIR="/usr/local/bin"
CARGO_BIN="${HOME}/.cargo/bin/${APP_NAME}"
VERSION="v0.1.7"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'
BOLD='\033[1m'

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
    clear
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
    else
        echo -e "${RED}Installation failed. Please check your PATH and try again.${NC}"
    fi
}

# Main menu
main_menu() {
    while true; do
        banner
        echo -e "${CYAN}This script will build and install Heist globally, or uninstall it.${NC}"
        echo -e "${YELLOW}You may need sudo privileges to copy to ${INSTALL_DIR}.${NC}\n"
        echo -e "${CYAN}Options:${NC}"
        echo -e "  [1] Install/Update Heist"
        echo -e "  [2] Uninstall Heist"
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
