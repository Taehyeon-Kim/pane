#!/usr/bin/env bats
# Test Suite: Prerequisite Validation (AC9: Error Handling)
# Priority: P0 - Critical infrastructure validation
# Test IDs: 5.2-UNIT-001, 5.2-UNIT-002, 5.2-UNIT-003, 5.2-UNIT-014, 5.2-UNIT-015

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/mock_git'
load 'helpers/mock_uname'
load 'helpers/assertions'

setup() {
    # Reset PATH to include mock bin directory first, but preserve system bins for basic commands
    export PATH="$MOCK_BIN_DIR:/usr/bin:/bin"

    # Create minimal Cargo.toml for testing
    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    cd "$BATS_TEST_TMPDIR"
}

# 5.2-UNIT-001: Script exists and is executable
@test "5.2-UNIT-001: Script exists and is executable" {
    assert_file_exists "$PROJECT_ROOT/scripts/build-release.sh"
    [ -x "$PROJECT_ROOT/scripts/build-release.sh" ]
}

# 5.2-UNIT-002: Script has proper shebang
@test "5.2-UNIT-002: Script has proper shebang (#!/usr/bin/env bash)" {
    assert_file_exists "$PROJECT_ROOT/scripts/build-release.sh"

    local first_line=$(head -n 1 $PROJECT_ROOT/scripts/build-release.sh)
    [[ "$first_line" == "#!/usr/bin/env bash" ]]
}

# 5.2-UNIT-003: Script sets strict error handling
@test "5.2-UNIT-003: Script sets strict error handling (set -e -u -o pipefail)" {
    assert_file_exists "$PROJECT_ROOT/scripts/build-release.sh"

    # Check for set -e
    grep -q "set -e" $PROJECT_ROOT/scripts/build-release.sh

    # Check for set -u
    grep -q "set -u" $PROJECT_ROOT/scripts/build-release.sh

    # Check for set -o pipefail
    grep -q "set -o pipefail" $PROJECT_ROOT/scripts/build-release.sh
}

# 5.2-UNIT-014: Error when Rust toolchain not found
@test "5.2-UNIT-014: Error when Rust toolchain not found" {
    # Remove cargo from PATH
    mock_cargo_missing
    mock_git_success

    # Run script (should fail)
    run $PROJECT_ROOT/scripts/build-release.sh

    # Should exit with code 1 (all errors use exit 1)
    assert_exit_status 1

    # Error message should be clear and actionable
    assert_output_contains "Rust toolchain not found"
    assert_output_contains "https://rustup.rs"
}

# 5.2-UNIT-015: Error when git not available
@test "5.2-UNIT-015: Error when git not available" {
    # Setup cargo and uname, but remove git
    mock_cargo_success
    mock_uname_success

    # Remove git from PATH completely (don't use system git)
    rm -f "$MOCK_BIN_DIR/git"

    # Temporarily use restricted PATH without /usr/bin
    PATH="$MOCK_BIN_DIR:/bin" run $PROJECT_ROOT/scripts/build-release.sh

    # Should exit with code 1 (all errors use exit 1)
    assert_exit_status 1

    # Error message should be clear
    assert_output_contains "Git is required"
    assert_output_contains "install git"
}

# Additional P0: Prerequisites check runs before build
@test "Prerequisites validated before attempting build" {
    # Setup: cargo missing
    mock_cargo_missing
    mock_git_success

    # Run script
    run $PROJECT_ROOT/scripts/build-release.sh

    # Should fail early before any build attempts
    [ "$status" -ne 0 ]

    # Should not create any dist/ artifacts
    [ ! -d "$DIST_DIR" ] || [ -z "$(ls -A "$DIST_DIR")" ]
}

# Additional P0: Error messages go to stderr
@test "Error messages output to stderr (>&2)" {
    mock_cargo_missing

    run $PROJECT_ROOT/scripts/build-release.sh

    # Capture stderr (BATS stores in $output)
    # Error messages should be in output
    [[ "$output" =~ "ERROR" ]] || [[ "$output" =~ "error" ]]
}
