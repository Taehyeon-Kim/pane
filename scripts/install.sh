#!/usr/bin/env bash
# Pane Installation Script
# POSIX-compatible installation script for Pane and bundled skills

set -e  # Exit on error
set -u  # Error on undefined variables

# Configuration
PREFIX="${PREFIX:-/usr/local}"
DRY_RUN=false

# Color codes for output (fallback to empty if not supported)
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    NC='\033[0m' # No Color
else
    GREEN=''
    RED=''
    YELLOW=''
    NC=''
fi

# Usage information
usage() {
    cat <<EOF
Pane Installation Script

USAGE:
    ./scripts/install.sh [OPTIONS]

OPTIONS:
    --help              Show this help message
    --dry-run           Show what would be installed without actually installing
    --prefix PATH       Set installation prefix (default: /usr/local)

ENVIRONMENT VARIABLES:
    PREFIX              Installation prefix (default: /usr/local)

EXAMPLES:
    # Install to default location (/usr/local)
    sudo ./scripts/install.sh

    # Install to custom location
    PREFIX=\$HOME/.local ./scripts/install.sh

    # Preview installation without making changes
    ./scripts/install.sh --dry-run

EOF
    exit 0
}

# Print info message
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

# Print warning message
warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1" >&2
}

# Print error message and exit
error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1" >&2
    exit 1
}

# Execute command or print for dry-run
execute() {
    if [ "$DRY_RUN" = true ]; then
        echo "[DRY-RUN] $*"
    else
        "$@"
    fi
}

# Check if running with sufficient permissions
check_permissions() {
    local test_dir="$1"

    # Skip permission check for dry-run
    if [ "$DRY_RUN" = true ]; then
        return 0
    fi

    # Try to create parent directory if it doesn't exist
    if [ ! -d "$test_dir" ]; then
        if ! mkdir -p "$test_dir" 2>/dev/null; then
            error "Permission denied: Cannot write to $test_dir. Run with sudo or use custom prefix: PREFIX=\$HOME/.local $0"
        fi
    elif [ ! -w "$test_dir" ]; then
        error "Permission denied: Cannot write to $test_dir. Run with sudo or use custom prefix: PREFIX=\$HOME/.local $0"
    fi
}

# Check if release binaries exist
check_binaries() {
    info "Checking for release binaries..."

    if [ ! -f "target/release/pane" ]; then
        error "Binary not found: target/release/pane. Run 'cargo build --release' first."
    fi

    if [ ! -f "target/release/claude-tips" ]; then
        error "Binary not found: target/release/claude-tips. Run 'cargo build --release' first."
    fi

    info "Release binaries found"
}

# Check for existing installation
check_existing_installation() {
    if [ -f "$PREFIX/bin/pane" ]; then
        if [ "$DRY_RUN" = true ]; then
            warn "Existing installation found at $PREFIX/bin/pane (would prompt for upgrade)"
            return 0
        fi

        warn "Existing installation found at $PREFIX/bin/pane"
        printf "Upgrade existing installation? (y/N): "
        read -r response
        case "$response" in
            [yY][eE][sS]|[yY])
                info "Upgrading existing installation..."
                ;;
            *)
                info "Installation cancelled by user"
                exit 0
                ;;
        esac
    fi
}

# Rollback function for error recovery
rollback() {
    warn "Installation failed. Rolling back changes..."

    # Remove partially installed files
    [ -f "$PREFIX/bin/pane" ] && rm -f "$PREFIX/bin/pane" && info "Removed $PREFIX/bin/pane"
    [ -f "$PREFIX/bin/claude-tips" ] && rm -f "$PREFIX/bin/claude-tips" && info "Removed $PREFIX/bin/claude-tips"
    [ -d "$PREFIX/share/pane" ] && rm -rf "$PREFIX/share/pane" && info "Removed $PREFIX/share/pane"

    error "Installation rolled back due to failure"
}

# Install binaries
install_binaries() {
    info "Installing Pane binaries to $PREFIX/bin..."

    # Create bin directory if needed
    execute mkdir -p "$PREFIX/bin"

    # Copy binaries
    execute cp -f target/release/pane "$PREFIX/bin/pane" || rollback
    execute cp -f target/release/claude-tips "$PREFIX/bin/claude-tips" || rollback

    # Set executable permissions
    execute chmod +x "$PREFIX/bin/pane" || rollback
    execute chmod +x "$PREFIX/bin/claude-tips" || rollback

    info "Binaries installed successfully"
}

# Install bundled skills
install_skills() {
    info "Installing Claude Tips Viewer skill to $PREFIX/share/pane/skills..."

    # Create skill directories
    execute mkdir -p "$PREFIX/share/pane/skills/claude-tips/data" || rollback

    # Copy skill manifest
    if [ ! -f "skills/claude-tips/pane-skill.yaml" ]; then
        error "Skill manifest not found: skills/claude-tips/pane-skill.yaml"
    fi
    execute cp -f skills/claude-tips/pane-skill.yaml "$PREFIX/share/pane/skills/claude-tips/pane-skill.yaml" || rollback

    # Copy skill data
    if [ ! -f "skills/claude-tips/data/claude-tips.yaml" ]; then
        error "Skill data not found: skills/claude-tips/data/claude-tips.yaml"
    fi
    execute cp -f skills/claude-tips/data/claude-tips.yaml "$PREFIX/share/pane/skills/claude-tips/data/claude-tips.yaml" || rollback

    info "Skills installed successfully"
}

# Main installation function
main() {
    # Parse command line arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --help)
                usage
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --prefix)
                PREFIX="$2"
                shift 2
                ;;
            *)
                error "Unknown option: $1. Use --help for usage information."
                ;;
        esac
    done

    info "Pane Installation Script"
    info "Installation prefix: $PREFIX"

    if [ "$DRY_RUN" = true ]; then
        warn "DRY-RUN MODE: No files will be modified"
    fi

    # Pre-installation checks
    check_binaries
    check_permissions "$PREFIX/bin"
    check_existing_installation

    # Install components
    install_binaries
    install_skills

    # Success message
    printf "\n"
    info "Installation complete! Run 'pane' to launch."

    # Check if PREFIX/bin is in PATH
    if [ "$DRY_RUN" = false ]; then
        case ":$PATH:" in
            *":$PREFIX/bin:"*)
                ;;
            *)
                warn "$PREFIX/bin is not in your PATH. Add it to your shell profile:"
                echo "    export PATH=\"$PREFIX/bin:\$PATH\""
                ;;
        esac
    fi
}

# Run main function
main "$@"
