#!/usr/bin/env bash
# Interactive installer/uninstaller for Heist (Rust shell history analyzer)
set -e

APP_NAME="heist"
INSTALL_DIR="/usr/local/bin"
CARGO_BIN="${HOME}/.cargo/bin/${APP_NAME}"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

function prompt_continue() {
    read -rp "Press Enter to continue or Ctrl+C to abort..."
}

function uninstall() {
    echo -e "${YELLOW}Uninstalling Heist...${NC}"
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

clear
echo -e "${CYAN}"
cat <<'EOF'

██╗  ██╗███████╗██╗███████╗████████╗
██║  ██║██╔════╝██║██╔════╝╚══██╔══╝
███████║█████╗  ██║███████╗   ██║   
██╔══██║██╔══╝  ██║╚════██║   ██║   
██║  ██║███████╗██║███████║   ██║   
╚═╝  ╚═╝╚══════╝╚═╝╚══════╝   ╚═╝   

EOF
echo -e "${YELLOW}  Heist  ${CYAN}v0.1.0"
echo -e "Heist Interactive Installer${NC}"

echo -e "${CYAN}This script will build and install Heist globally, or uninstall it.${NC}"
echo -e "${YELLOW}You may need sudo privileges to copy to ${INSTALL_DIR}.${NC}"
echo -e ""
echo -e "${CYAN}Options:${NC}"
echo -e "  [1] Install/Update Heist"
echo -e "  [2] Uninstall Heist"
echo -e "  [q] Quit"
echo -n "Choose an option: "
read -r opt
case "$opt" in
    1)
        echo -e "${GREEN}Proceeding with installation...${NC}"
        ;;
    2)
        uninstall
        ;;
    q|Q)
        echo "Aborted."
        exit 0
        ;;
    *)
        echo "Invalid option. Exiting."
        exit 1
        ;;
esac
prompt_continue

# Check for Rust
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${RED}Rust (cargo) is not installed. Please install Rust first: https://rustup.rs/${NC}"
    exit 1
fi

# Check for sudo
if ! command -v sudo >/dev/null 2>&1; then
    echo -e "${RED}sudo is required for installation. Please install sudo or run as root.${NC}"
    exit 1
fi

echo -e "${GREEN}Building Heist in release mode...${NC}"
cargo build --release || { echo -e "${RED}Build failed. Aborting.${NC}"; exit 1; }

# Find built binary
if [ ! -f "target/release/${APP_NAME}" ]; then
    echo -e "${RED}Build failed: target/release/${APP_NAME} not found.${NC}"
    exit 1
fi

echo -e "${GREEN}Installing to ${INSTALL_DIR}...${NC}"
if sudo cp "target/release/${APP_NAME}" "${INSTALL_DIR}/"; then
    if command -v ${APP_NAME} >/dev/null 2>&1; then
        echo -e "${GREEN}Heist installed successfully! Run 'heist --help' to get started.${NC}"
    else
        echo -e "${RED}Installation failed. Please check your PATH and try again.${NC}"
    fi
else
    echo -e "${RED}Failed to copy binary to ${INSTALL_DIR}.${NC}"
    exit 1
fi
