#!/usr/bin/env bash
set -euo pipefail

# Windows not supported
if [[ ${OS:-} = Windows_NT ]]; then
    echo 'error: Please install bun using Windows Subsystem for Linux'
    exit 1
fi

# Setting global variables
INSTALL_DIR="$HOME/.tcli/bin"

# Fancy print
RESET=''
RED=''
GREEN=''
BLUE=''

if [[ -t 1 ]]; then
    RESET='\033[0m'
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    BLUE='\033[0;34m'
fi

error() {
    echo -e "${RED}error${RESET}:" "$@" >&2
    exit 1
}

info () {
    echo -e "${BLUE}info${RESET}:" "$@" >&1
}

success() {
    echo -e "${GREEN}success${RESET}:" "$@" >&1
}

platform=$(uname -ms)
case $platform in
'Darwin x86_64')
    info "Installing tcli for MacOS"
    target=darwin-x64
    ;;
'Darwin arm64')
    info "Installing tcli for MacOS (M1)"
    target=darwin-aarch64
    ;;
'Linux x86_64')
    info "Installing tcli for Linux"
    target=linux
    ;;
  *)
    error "$platform is not supported. You can open a feature request at https://github.com/Tectone23/tcli/issues"
    ;;
esac


github_repo="https://github.com/Tectone23/tcli"
github_latest="$github_repo/releases/latest/download/tcli-"

download_url="$github_latest$target"

if [[ ! -d $INSTALL_DIR ]]; then
  mkdir -p $INSTALL_DIR ||
    error "Failed to create $INSTALL_DIR"
fi

# Download the right version
info "Downloading tcli (tcli-$target)"
info "This may take a while, depending on your internet connenection"
curl -fLs $download_url -o "$INSTALL_DIR/tcli" ||
  error "Failed to download '$download_url'"

info "Downloaded tcli successfully at $INSTALL_DIR/tcli"

chmod +x "$INSTALL_DIR/tcli" ||
  error "Failed to set tcli as an executable"

success "Installation completed successfully"

# TODO
info "Add $INSTALL_DIR/tcli to your \$PATH"
