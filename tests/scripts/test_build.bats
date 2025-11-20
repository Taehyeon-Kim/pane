#!/usr/bin/env bats
# Test Suite: Build Execution (AC2, AC3: Workspace build with release profile)
# Priority: P0, P1 - Core build functionality
# Test IDs: 5.2-INT-001, 5.2-INT-002, 5.2-INT-005

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/mock_git'
load 'helpers/assertions'

setup() {
    export PATH="$MOCK_BIN_DIR:$ORIGINAL_PATH"

    # Create test environment
    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "0.1.0"

[workspace]
members = ["skills/claude-tips"]
EOF

    mkdir -p "$BATS_TEST_TMPDIR/skills/claude-tips"

    cd "$BATS_TEST_TMPDIR"
}

# 5.2-INT-001: Native build produces both binaries
@test "5.2-INT-001: Native build produces both binaries (pane, claude-tips)" {
    # Setup mocks
    mock_cargo_success
    mock_git_success

    # Run build script
    run $PROJECT_ROOT/scripts/build-release.sh

    # Should succeed
    assert_exit_status 0

    # Both binaries should exist
    assert_file_exists "$TARGET_DIR/release/pane"
    assert_file_exists "$TARGET_DIR/release/claude-tips"
}

# 5.2-INT-002: Build command includes --workspace flag
@test "5.2-INT-002: Build command includes --workspace flag" {
    # Create wrapper to capture cargo arguments
    cat > "$MOCK_BIN_DIR/cargo" <<'EOF'
#!/usr/bin/env bash
# Capture arguments to file
echo "$@" >> "$BATS_TEST_TMPDIR/cargo_calls.txt"

if [[ "$1" == "build" ]]; then
    mkdir -p "$TARGET_DIR/release"
    echo "mock" > "$TARGET_DIR/release/pane"
    echo "mock" > "$TARGET_DIR/release/claude-tips"
    exit 0
elif [[ "$1" == "metadata" ]]; then
    echo '{"packages":[{"name":"pane","version":"0.1.0"}]}'
    exit 0
fi
EOF
    chmod +x "$MOCK_BIN_DIR/cargo"
    mock_git_success

    # Run build
    run $PROJECT_ROOT/scripts/build-release.sh

    # Check cargo was called with --workspace
    assert_file_exists "$BATS_TEST_TMPDIR/cargo_calls.txt"
    assert_file_contains "$BATS_TEST_TMPDIR/cargo_calls.txt" "--workspace"
}

# 5.2-INT-005: Build uses --release flag
@test "5.2-INT-005: Build uses --release flag for optimizations" {
    # Create wrapper to capture arguments
    cat > "$MOCK_BIN_DIR/cargo" <<'EOF'
#!/usr/bin/env bash
echo "$@" >> "$BATS_TEST_TMPDIR/cargo_calls.txt"

if [[ "$1" == "build" ]]; then
    mkdir -p "$TARGET_DIR/release"
    echo "mock" > "$TARGET_DIR/release/pane"
    echo "mock" > "$TARGET_DIR/release/claude-tips"
    exit 0
elif [[ "$1" == "metadata" ]]; then
    echo '{"packages":[{"name":"pane","version":"0.1.0"}]}'
    exit 0
fi
EOF
    chmod +x "$MOCK_BIN_DIR/cargo"
    mock_git_success

    # Run build
    run $PROJECT_ROOT/scripts/build-release.sh

    # Check cargo was called with --release
    assert_file_contains "$BATS_TEST_TMPDIR/cargo_calls.txt" "--release"
}

# 5.2-INT-024: Error on cargo build failure with actionable message
@test "5.2-INT-024: Clear error message on cargo build failure" {
    # Setup: cargo fails
    mock_cargo_failure
    mock_git_success

    # Run build (should fail)
    run $PROJECT_ROOT/scripts/build-release.sh

    # Should exit with error
    [ "$status" -ne 0 ]

    # Error message should be actionable
    assert_output_contains "Build failed" || assert_output_contains "could not compile"
}

# Additional P0: Build creates target directory structure
@test "Build creates proper target directory structure" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0
    assert_dir_exists "$TARGET_DIR/release"
}

# Additional P0: Build displays progress messages
@test "Build displays progress messages during execution" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Should show progress indicators
    [[ "$output" =~ "Compiling" ]] || [[ "$output" =~ "Building" ]] || [[ "$output" =~ "STEP" ]]
}
