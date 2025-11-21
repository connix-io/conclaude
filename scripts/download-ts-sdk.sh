#!/usr/bin/env bash

# Script to download and maintain the latest version of the Anthropic TypeScript SDK
# for reference and comparison purposes during Go SDK development.

set -euo pipefail

# Color output for better UX
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
TARGET_DIR=".claude/contexts/claude-agent-sdk-ts"
PACKAGE_NAME="@anthropic-ai/claude-agent-sdk"
TEMP_DIR="/tmp/ts-sdk-download-$$"

# Helper functions
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Cleanup function
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}

# Register cleanup on exit
trap cleanup EXIT

# Check for npm availability
check_npm() {
    info "Checking for npm..."
    if ! command -v npm &> /dev/null; then
        error "npm is not installed or not in PATH"
        error "Please install Node.js and npm to use this script"
        error "Visit: https://nodejs.org/ for installation instructions"
        exit 1
    fi
    info "npm found: $(npm --version)"
}

# Check for tar availability
check_tar() {
    if ! command -v tar &> /dev/null; then
        error "tar is not installed or not in PATH"
        error "Please install tar to use this script"
        exit 1
    fi
}

# Create target directory if it doesn't exist
create_target_directory() {
    info "Creating target directory..."
    if ! mkdir -p "$TARGET_DIR" 2>/dev/null; then
        error "Failed to create directory: $TARGET_DIR"
        error "Check that you have write permissions in this location"
        exit 1
    fi
}

# Check if SDK is already installed
check_existing_installation() {
    if [ -f "$TARGET_DIR/package.json" ]; then
        return 0  # Installation exists
    else
        return 1  # No installation
    fi
}

# Download and extract the TypeScript SDK
download_and_extract_sdk() {
    # Save current directory
    ORIGINAL_DIR=$(pwd)

    # Create temp directory
    mkdir -p "$TEMP_DIR"

    info "Downloading $PACKAGE_NAME using npm pack..."
    cd "$TEMP_DIR"

    if ! npm pack "$PACKAGE_NAME" 2>&1; then
        cd "$ORIGINAL_DIR"
        error "Failed to download $PACKAGE_NAME"
        error "This could be due to:"
        error "  - Network connectivity issues"
        error "  - npm registry unavailability"
        error "Try running the script again or check your internet connection"
        exit 1
    fi

    # Find the downloaded tarball (npm pack creates a .tgz file)
    TARBALL=$(ls anthropic-ai-claude-agent-sdk-*.tgz 2>/dev/null | head -n 1)

    if [ -z "$TARBALL" ]; then
        cd "$ORIGINAL_DIR"
        error "Failed to find downloaded package tarball"
        exit 1
    fi

    info "Extracting TypeScript SDK source files..."

    # Extract to target directory
    # npm pack creates a 'package' directory inside the tarball
    if ! tar -xzf "$TARBALL" -C "$TEMP_DIR" 2>&1; then
        cd "$ORIGINAL_DIR"
        error "Failed to extract package tarball"
        exit 1
    fi

    # Return to original directory
    cd "$ORIGINAL_DIR"

    # Remove old installation if it exists
    if check_existing_installation; then
        info "Removing old installation..."
        rm -rf "$TARGET_DIR"
    fi

    # Ensure target directory exists
    mkdir -p "$TARGET_DIR"

    # Move extracted files to target directory
    info "Installing to $TARGET_DIR..."
    cp -r "$TEMP_DIR/package"/* "$TARGET_DIR/"

    info "TypeScript SDK successfully installed!"
}

# Display success message with location
show_success() {
    echo ""
    info "✓ TypeScript SDK source files are available at: $TARGET_DIR"

    if [ -d "$TARGET_DIR/src" ]; then
        info "✓ Source code location: $TARGET_DIR/src"
    fi

    if [ -f "$TARGET_DIR/package.json" ]; then
        VERSION=$(grep '"version"' "$TARGET_DIR/package.json" | head -1 | sed 's/.*: "\(.*\)".*/\1/')
        info "✓ Version: $VERSION"
    fi

    echo ""
}

# Main execution
main() {
    echo ""
    info "TypeScript SDK Download Script"
    echo ""

    check_npm
    check_tar
    create_target_directory
    download_and_extract_sdk
    show_success
}

# Run main function
main
