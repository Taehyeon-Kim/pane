#!/usr/bin/env bash
# Pane Build & Release Automation Script
# Builds optimized release binaries with cross-compilation support

set -e  # Exit on error
set -u  # Error on undefined variables
set -o pipefail  # Pipe failures propagate

# Configuration
TARGET=""
DRY_RUN=false
CLEAN=false
SUPPORTED_TARGETS=("x86_64-apple-darwin" "aarch64-apple-darwin" "x86_64-unknown-linux-gnu")

# Color codes for output (fallback to empty if not supported)
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    GREEN=''
    RED=''
    YELLOW=''
    BLUE=''
    NC=''
fi

# Usage information
usage() {
    cat <<EOF
Pane Build & Release Automation Script

USAGE:
    ./scripts/build-release.sh [OPTIONS]

OPTIONS:
    --help              Show this help message
    --target TARGET     Build for specific target platform (auto-detect if not specified)
                        Supported: x86_64-apple-darwin, aarch64-apple-darwin, x86_64-unknown-linux-gnu
    --dry-run           Show what would be built without actually building
    --clean             Remove existing dist/ directory before building

EXAMPLES:
    # Build for host platform
    ./scripts/build-release.sh

    # Build for specific target
    ./scripts/build-release.sh --target x86_64-unknown-linux-gnu

    # Clean build from scratch
    ./scripts/build-release.sh --clean

    # Preview build actions
    ./scripts/build-release.sh --dry-run

EXIT CODES:
    0    Success
    1    Build failed, invalid arguments, or missing prerequisites

CROSS-COMPILATION:
    For cross-compilation to Linux from macOS, install 'cross':
        cargo install cross

    Native cross-compilation (e.g., x86_64 to ARM64 on macOS):
        rustup target add aarch64-apple-darwin

EOF
    exit 0
}

# Print info message
info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

# Print step message
step() {
    printf "${BLUE}[STEP]${NC} %s\n" "$1"
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

# Detect host platform
detect_host_target() {
    local os
    local arch

    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Darwin)
            case "$arch" in
                x86_64)
                    echo "x86_64-apple-darwin"
                    ;;
                arm64)
                    echo "aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported macOS architecture: $arch"
                    ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64)
                    echo "x86_64-unknown-linux-gnu"
                    ;;
                *)
                    error "Unsupported Linux architecture: $arch"
                    ;;
            esac
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
}

# Validate target platform
validate_target() {
    local target=$1
    local supported=false

    for supported_target in "${SUPPORTED_TARGETS[@]}"; do
        if [ "$target" = "$supported_target" ]; then
            supported=true
            break
        fi
    done

    if [ "$supported" = false ]; then
        error "Unsupported target: $target. Supported targets: ${SUPPORTED_TARGETS[*]}"
    fi
}

# Check prerequisites
check_prerequisites() {
    step "Checking prerequisites..."

    # Check for cargo
    if ! command -v cargo >/dev/null 2>&1; then
        error "Rust toolchain not found. Install from https://rustup.rs/"
    fi
    info "✓ Rust toolchain found: $(rustc --version)"

    # Check for git
    if ! command -v git >/dev/null 2>&1; then
        error "Git is required for build metadata. Install git first."
    fi
    info "✓ Git found: $(git --version | head -n1)"

    # Check for tar
    if ! command -v tar >/dev/null 2>&1; then
        error "tar is required for creating archives. Install tar first."
    fi
    info "✓ tar found"

    # Check for sha256sum or shasum
    if command -v sha256sum >/dev/null 2>&1; then
        info "✓ sha256sum found"
    elif command -v shasum >/dev/null 2>&1; then
        info "✓ shasum found"
    else
        error "sha256sum or shasum is required for checksums. Install coreutils or shasum."
    fi
}

# Check cross-compilation tooling
check_cross_compilation() {
    local target=$1
    local host_target
    host_target=$(detect_host_target)

    # If building for host platform, no cross-compilation needed
    if [ "$target" = "$host_target" ]; then
        return 0
    fi

    # Check if target is installed
    if rustup target list --installed | grep -q "$target"; then
        info "✓ Target $target is installed"
        return 0
    fi

    # Check for cross tool (recommended for Linux targets from macOS)
    if command -v cross >/dev/null 2>&1; then
        info "✓ cross tool found for cross-compilation"
        return 0
    fi

    # Suggest installation
    warn "Cross-compilation toolchain not found for $target"
    warn "Install with: rustup target add $target"
    warn "Or for Linux targets from macOS: cargo install cross"
    error "Cross-compilation requires either rustup target or cross tool"
}

# Build workspace binaries
build_binaries() {
    local target=$1
    local host_target
    host_target=$(detect_host_target)

    step "Building workspace binaries for $target..."

    if [ "$target" = "$host_target" ]; then
        # Native build
        info "Building natively for $target..."
        if [ "$DRY_RUN" = true ]; then
            execute cargo build --release --workspace
        else
            if ! cargo build --release --workspace; then
                error "Build failed. See cargo output above for details."
            fi
        fi
    else
        # Cross-compilation build
        info "Cross-compiling for $target..."

        # Try using cross if available
        if command -v cross >/dev/null 2>&1; then
            if [ "$DRY_RUN" = true ]; then
                execute cross build --release --workspace --target "$target"
            else
                if ! cross build --release --workspace --target "$target"; then
                    error "Cross-compilation build failed. See output above for details."
                fi
            fi
        else
            # Fall back to cargo with target
            if [ "$DRY_RUN" = true ]; then
                execute cargo build --release --workspace --target "$target"
            else
                if ! cargo build --release --workspace --target "$target"; then
                    error "Cross-compilation build failed. See output above for details."
                fi
            fi
        fi
    fi

    info "✓ Build completed successfully"
}

# Verify binaries exist
verify_binaries() {
    local target=$1
    local host_target
    local binary_dir

    host_target=$(detect_host_target)

    if [ "$target" = "$host_target" ]; then
        binary_dir="target/release"
    else
        binary_dir="target/$target/release"
    fi

    step "Verifying binaries in $binary_dir..."

    if [ "$DRY_RUN" = true ]; then
        info "Would verify: $binary_dir/pane"
        info "Would verify: $binary_dir/claude-tips"
        return 0
    fi

    if [ ! -f "$binary_dir/pane" ]; then
        error "Binary not found: $binary_dir/pane"
    fi
    info "✓ Found pane binary"

    if [ ! -f "$binary_dir/claude-tips" ]; then
        error "Binary not found: $binary_dir/claude-tips"
    fi
    info "✓ Found claude-tips binary"
}

# Get version from Cargo.toml
get_version() {
    # Try cargo metadata first
    if command -v jq >/dev/null 2>&1; then
        cargo metadata --format-version 1 --no-deps 2>/dev/null | \
            jq -r '.packages[] | select(.name == "pane") | .version' || \
            grep '^version' Cargo.toml | head -n1 | sed 's/version = "\(.*\)"/\1/'
    else
        # Fallback to grep
        grep '^version' Cargo.toml | head -n1 | sed 's/version = "\(.*\)"/\1/'
    fi
}

# Organize build artifacts in dist/ directory
organize_artifacts() {
    local target=$1
    local version=$2
    local host_target
    local binary_dir
    local dist_dir

    host_target=$(detect_host_target)

    if [ "$target" = "$host_target" ]; then
        binary_dir="target/release"
    else
        binary_dir="target/$target/release"
    fi

    dist_dir="dist/v$version/$target"

    step "Organizing artifacts in $dist_dir..."

    if [ "$DRY_RUN" = true ]; then
        execute mkdir -p "$dist_dir/bin"
        execute mkdir -p "$dist_dir/share/pane/skills/claude-tips/data"
        execute cp "$binary_dir/pane" "$dist_dir/bin/"
        execute cp "$binary_dir/claude-tips" "$dist_dir/bin/"
        execute cp "skills/claude-tips/pane-skill.yaml" "$dist_dir/share/pane/skills/claude-tips/"
        execute cp "skills/claude-tips/data/claude-tips.yaml" "$dist_dir/share/pane/skills/claude-tips/data/"
        return 0
    fi

    # Create dist directory structure
    if ! mkdir -p "$dist_dir/bin"; then
        error "Failed to create directory: $dist_dir/bin"
    fi

    if ! mkdir -p "$dist_dir/share/pane/skills/claude-tips/data"; then
        error "Failed to create directory: $dist_dir/share/pane/skills/claude-tips/data"
    fi

    # Copy binaries
    if ! cp "$binary_dir/pane" "$dist_dir/bin/"; then
        error "Failed to copy pane binary to $dist_dir/bin"
    fi
    info "✓ Copied pane to $dist_dir/bin"

    if ! cp "$binary_dir/claude-tips" "$dist_dir/bin/"; then
        error "Failed to copy claude-tips binary to $dist_dir/bin"
    fi
    info "✓ Copied claude-tips to $dist_dir/bin"

    # Copy skill files
    if ! cp "skills/claude-tips/pane-skill.yaml" "$dist_dir/share/pane/skills/claude-tips/"; then
        error "Failed to copy pane-skill.yaml to $dist_dir/share/pane/skills/claude-tips/"
    fi
    info "✓ Copied pane-skill.yaml to $dist_dir/share/pane/skills/claude-tips"

    if ! cp "skills/claude-tips/data/claude-tips.yaml" "$dist_dir/share/pane/skills/claude-tips/data/"; then
        error "Failed to copy claude-tips.yaml to $dist_dir/share/pane/skills/claude-tips/data/"
    fi
    info "✓ Copied claude-tips.yaml to $dist_dir/share/pane/skills/claude-tips/data"
}

# Create tar.gz archive
create_archive() {
    local target=$1
    local version=$2
    local dist_dir="dist/v$version/$target"
    local archive_name="pane-v$version-$target.tar.gz"
    local archive_path="dist/v$version/$archive_name"

    step "Creating archive: $archive_name..."

    if [ "$DRY_RUN" = true ]; then
        execute tar -czf "$archive_path" -C "$dist_dir" bin share
        return 0
    fi

    # Create archive with bin/ and share/ directory structure
    if ! tar -czf "$archive_path" -C "$dist_dir" bin share; then
        error "Failed to create archive. Ensure tar is installed and writable dist/ directory."
    fi

    # Verify archive
    if [ ! -f "$archive_path" ]; then
        error "Archive not created: $archive_path"
    fi

    # Get archive size
    local size
    size=$(ls -lh "$archive_path" | awk '{print $5}')
    info "✓ Created archive: $archive_name ($size)"
}

# Generate SHA256 checksum
generate_checksum() {
    local target=$1
    local version=$2
    local archive_name="pane-v$version-$target.tar.gz"
    local archive_path="dist/v$version/$archive_name"
    local checksum_path="$archive_path.sha256"

    step "Generating SHA256 checksum..."

    if [ "$DRY_RUN" = true ]; then
        if command -v sha256sum >/dev/null 2>&1; then
            execute sha256sum "$archive_path"
        else
            execute shasum -a 256 "$archive_path"
        fi
        return 0
    fi

    # Generate checksum
    if command -v sha256sum >/dev/null 2>&1; then
        if ! sha256sum "$archive_path" > "$checksum_path"; then
            error "Failed to generate checksum"
        fi
    else
        if ! shasum -a 256 "$archive_path" > "$checksum_path"; then
            error "Failed to generate checksum"
        fi
    fi

    # Display checksum
    local checksum
    checksum=$(cat "$checksum_path" | awk '{print $1}')
    info "✓ Generated checksum: ${checksum:0:16}..."
}

# Create build metadata
create_metadata() {
    local target=$1
    local version=$2
    local metadata_file="dist/v$version/build-metadata.json"

    step "Creating build metadata..."

    if [ "$DRY_RUN" = true ]; then
        info "Would create: $metadata_file"
        return 0
    fi

    # Get metadata values
    local git_commit
    local build_date
    local rust_version
    local archive_name
    local archive_path
    local archive_size
    local checksum

    git_commit=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    build_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    rust_version=$(rustc --version | awk '{print $2}')
    archive_name="pane-v$version-$target.tar.gz"
    archive_path="dist/v$version/$archive_name"

    if [ -f "$archive_path" ]; then
        archive_size=$(stat -f%z "$archive_path" 2>/dev/null || stat -c%s "$archive_path" 2>/dev/null)
        if [ -f "$archive_path.sha256" ]; then
            checksum=$(cat "$archive_path.sha256" | awk '{print $1}')
        else
            checksum="unknown"
        fi
    else
        archive_size=0
        checksum="unknown"
    fi

    # Create JSON metadata
    cat > "$metadata_file" <<EOF
{
  "version": "$version",
  "git_commit": "$git_commit",
  "build_date": "$build_date",
  "targets": [
    "$target"
  ],
  "rust_version": "$rust_version",
  "artifacts": [
    {
      "name": "$archive_name",
      "size_bytes": $archive_size,
      "sha256": "$checksum"
    }
  ]
}
EOF

    if [ ! -f "$metadata_file" ]; then
        error "Failed to create metadata file: $metadata_file"
    fi

    info "✓ Created build metadata"
    info "  Version: $version"
    info "  Git commit: $git_commit"
    info "  Build date: $build_date"
    info "  Rust version: $rust_version"
}

# Parse command-line arguments
parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            --help)
                usage
                ;;
            --target)
                if [ $# -lt 2 ]; then
                    error "Missing argument for --target"
                fi
                TARGET="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=true
                info "Dry-run mode enabled"
                shift
                ;;
            --clean)
                CLEAN=true
                info "Clean mode enabled"
                shift
                ;;
            *)
                error "Unknown option: $1. Use --help for usage information."
                ;;
        esac
    done
}

# Clean dist directory
clean_dist() {
    if [ "$CLEAN" = true ]; then
        step "Cleaning dist/ directory..."
        execute rm -rf dist/
        info "✓ Cleaned dist/ directory"
    fi
}

# Main execution
main() {
    # Parse arguments
    parse_args "$@"

    # Detect or validate target
    if [ -z "$TARGET" ]; then
        TARGET=$(detect_host_target)
        info "Auto-detected target: $TARGET"
    else
        validate_target "$TARGET"
        info "Building for target: $TARGET"
    fi

    # Get version
    local version
    version=$(get_version)
    info "Version: $version"

    # Clean if requested
    clean_dist

    # Check prerequisites
    check_prerequisites

    # Check cross-compilation tooling if needed
    check_cross_compilation "$TARGET"

    # Build binaries
    build_binaries "$TARGET"

    # Verify binaries exist
    verify_binaries "$TARGET"

    # Organize artifacts
    organize_artifacts "$TARGET" "$version"

    # Create archive
    create_archive "$TARGET" "$version"

    # Generate checksum
    generate_checksum "$TARGET" "$version"

    # Create metadata
    create_metadata "$TARGET" "$version"

    # Success
    info ""
    info "Build completed successfully!"
    info "Target: $TARGET"
    info "Version: $version"
    info "Artifacts: dist/v$version/"
    info ""
}

# Run main function
main "$@"
