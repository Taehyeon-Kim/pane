#!/usr/bin/env bash
# Pane Uninstallation Script
# POSIX-compatible uninstallation script for Pane and bundled skills

set -e  # Exit on error
set -u  # Error on undefined variables

# Configuration
PREFIX="${PREFIX:-/usr/local}"
DRY_RUN=false
SKIP_CONFIRMATION=false

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
Pane Uninstallation Script

USAGE:
    ./scripts/uninstall.sh [OPTIONS]

OPTIONS:
    --help              Show this help message
    --dry-run           Show what would be removed without actually removing
    --yes               Skip confirmation prompt (automatic yes)
    --prefix PATH       Set installation prefix (default: /usr/local)

ENVIRONMENT VARIABLES:
    PREFIX              Installation prefix (default: /usr/local)

EXAMPLES:
    # Uninstall from default location (/usr/local)
    sudo ./scripts/uninstall.sh

    # Uninstall from custom location
    PREFIX=\$HOME/.local ./scripts/uninstall.sh

    # Preview uninstallation without making changes
    ./scripts/uninstall.sh --dry-run

    # Uninstall without confirmation prompt
    sudo ./scripts/uninstall.sh --yes

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
    local test_path="$1"

    # Skip permission check for dry-run
    if [ "$DRY_RUN" = true ]; then
        return 0
    fi

    # Check if path exists and is writable
    if [ -e "$test_path" ]; then
        local parent_dir
        parent_dir=$(dirname "$test_path")
        if [ ! -w "$parent_dir" ]; then
            error "Permission denied: Cannot write to $parent_dir. Run with sudo or use custom prefix: PREFIX=\$HOME/.local $0"
        fi
    fi
}

# Confirm uninstallation
confirm_uninstallation() {
    if [ "$SKIP_CONFIRMATION" = true ]; then
        info "Skipping confirmation (--yes flag provided)"
        return 0
    fi

    if [ "$DRY_RUN" = true ]; then
        warn "Would prompt for confirmation: 'This will remove Pane and all bundled skills. Continue? (y/N)'"
        return 0
    fi

    warn "This will remove Pane and all bundled skills."
    printf "Continue? (y/N): "
    read -r response
    case "$response" in
        [yY][eE][sS]|[yY])
            info "Proceeding with uninstallation..."
            ;;
        *)
            info "Uninstallation cancelled by user"
            exit 0
            ;;
    esac
}

# Remove binaries
remove_binaries() {
    info "Removing Pane binaries..."

    local removed_any=false

    # Remove pane binary
    if [ -f "$PREFIX/bin/pane" ]; then
        execute rm -f "$PREFIX/bin/pane"
        removed_any=true
    elif [ "$DRY_RUN" = false ]; then
        warn "$PREFIX/bin/pane not found (already removed or never installed)"
    fi

    # Remove claude-tips binary
    if [ -f "$PREFIX/bin/claude-tips" ]; then
        execute rm -f "$PREFIX/bin/claude-tips"
        removed_any=true
    elif [ "$DRY_RUN" = false ]; then
        warn "$PREFIX/bin/claude-tips not found (already removed or never installed)"
    fi

    if [ "$removed_any" = true ] || [ "$DRY_RUN" = true ]; then
        info "Binaries removed successfully"
    fi
}

# Remove bundled skills
remove_skills() {
    info "Removing bundled skills..."

    # Remove entire skills directory
    if [ -d "$PREFIX/share/pane" ]; then
        execute rm -rf "$PREFIX/share/pane"
        info "Skills directory removed successfully"
    elif [ "$DRY_RUN" = false ]; then
        warn "$PREFIX/share/pane not found (already removed or never installed)"
    fi
}

# Main uninstallation function
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
            --yes)
                SKIP_CONFIRMATION=true
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

    info "Pane Uninstallation Script"
    info "Installation prefix: $PREFIX"

    if [ "$DRY_RUN" = true ]; then
        warn "DRY-RUN MODE: No files will be modified"
    fi

    # Pre-uninstallation checks
    check_permissions "$PREFIX/bin"
    confirm_uninstallation

    # Remove components
    remove_binaries
    remove_skills

    # Success message
    printf "\n"
    info "Uninstallation complete."
}

# Run main function
main "$@"
