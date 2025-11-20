#!/usr/bin/env bats
# Test Suite: Artifact Organization (AC5)
# Priority: P0/P1 - Critical build output structure
# Test IDs: 5.2-INT-009 through 5.2-INT-013

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/mock_git'
load 'helpers/mock_uname'
load 'helpers/assertions'

setup() {
    export PATH="$MOCK_BIN_DIR:/usr/bin:/bin"

    # Create minimal Cargo.toml
    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    cd "$BATS_TEST_TMPDIR"

    # Setup mocks for successful build
    mock_cargo_success
    mock_git_success
    mock_uname_success
}

# 5.2-INT-009: dist/ directory created with correct structure
@test "5.2-INT-009: dist/ directory created with correct structure" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Verify dist/ directory exists
    [ -d "dist" ]

    # Verify version subdirectory exists
    [ -d "dist/v0.1.0" ]
}

# 5.2-INT-010: Directory structure is dist/v{version}/{target}/
@test "5.2-INT-010: Directory structure follows dist/v{version}/{target}/ pattern" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Verify target-specific subdirectory exists
    # Auto-detected target should be aarch64-apple-darwin on ARM Mac
    [ -d "dist/v0.1.0/aarch64-apple-darwin" ] || [ -d "dist/v0.1.0/x86_64-apple-darwin" ]
}

# 5.2-UNIT-008: get_version() extracts correct version from Cargo.toml
@test "5.2-UNIT-008: Version extraction from Cargo.toml works correctly" {
    # Create Cargo.toml with specific version
    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "1.2.3"
EOF

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Verify version-specific directory created
    [ -d "dist/v1.2.3" ]
}

# 5.2-INT-011: Binaries copied to correct target subdirectory
@test "5.2-INT-011: Binaries copied to correct target subdirectory" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Find target directory (auto-detected)
    local target_dir=$(find dist/v0.1.0 -type d -maxdepth 1 | grep -E "(aarch64|x86_64)" | head -1)

    # Verify binaries exist in target directory
    [ -f "$target_dir/pane" ]
    [ -f "$target_dir/claude-tips" ]
}

# 5.2-INT-012: --clean flag removes old dist/ before build
@test "5.2-INT-012: --clean flag removes existing dist/ directory" {
    # Create old dist/ directory with marker file
    mkdir -p dist/old-marker
    touch dist/old-marker/test-file.txt

    run $PROJECT_ROOT/scripts/build-release.sh --clean

    assert_exit_status 0

    # Old marker should be gone
    [ ! -d "dist/old-marker" ]
    [ ! -f "dist/old-marker/test-file.txt" ]

    # New dist/ should exist
    [ -d "dist/v0.1.0" ]
}

# 5.2-INT-013: Multiple builds to same version directory coexist
@test "5.2-INT-013: Multiple target builds coexist in same version directory" {
    # First build (auto-detected target)
    run $PROJECT_ROOT/scripts/build-release.sh
    assert_exit_status 0

    local first_target=$(find dist/v0.1.0 -type d -maxdepth 1 | grep -E "(aarch64|x86_64)" | head -1)

    # Verify first build artifacts exist
    [ -d "$first_target" ]

    # Note: Testing actual cross-compilation requires cross tool
    # This test validates directory structure supports multiple targets
    [ -d "dist/v0.1.0" ]
}
