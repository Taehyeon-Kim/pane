#!/usr/bin/env bats
# Test Suite: Platform Detection & Cross-Compilation (AC4)
# Priority: P0, P1 - Target platform validation
# Test IDs: 5.2-UNIT-005, 5.2-UNIT-006, 5.2-UNIT-007, 5.2-UNIT-016

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/mock_git'
load 'helpers/assertions'

setup() {
    export PATH="$MOCK_BIN_DIR:$ORIGINAL_PATH"

    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    cd "$BATS_TEST_TMPDIR"
}

# 5.2-UNIT-006: Target validation accepts all supported platforms
@test "5.2-UNIT-006: Target validation accepts x86_64-apple-darwin" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh --target x86_64-apple-darwin

    # Should not fail on target validation
    [ "$status" -eq 0 ] || assert_output_contains "Building"
}

@test "5.2-UNIT-006: Target validation accepts aarch64-apple-darwin" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh --target aarch64-apple-darwin

    # Should not fail on target validation
    [ "$status" -eq 0 ] || assert_output_contains "Building"
}

@test "5.2-UNIT-006: Target validation accepts x86_64-unknown-linux-gnu" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh --target x86_64-unknown-linux-gnu

    # Should not fail on target validation
    [ "$status" -eq 0 ] || assert_output_contains "Building"
}

# 5.2-UNIT-007: Target validation rejects unsupported platforms
@test "5.2-UNIT-007: Target validation rejects invalid target triple" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh --target invalid-target-triple

    # Should exit with error
    assert_exit_status 1

    # Error should list supported targets
    assert_output_contains "Unsupported target"
    assert_output_contains "x86_64-apple-darwin"
    assert_output_contains "aarch64-apple-darwin"
    assert_output_contains "x86_64-unknown-linux-gnu"
}

# 5.2-UNIT-016: Error when unsupported target specified
@test "5.2-UNIT-016: Clear error for unsupported Windows target" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh --target x86_64-pc-windows-msvc

    assert_exit_status 1
    assert_output_contains "Unsupported target"
}

# Additional P0: Platform auto-detection works
@test "Script auto-detects host platform when no target specified" {
    mock_cargo_success
    mock_git_success

    run $PROJECT_ROOT/scripts/build-release.sh

    # Should succeed with auto-detected platform
    assert_exit_status 0

    # Output should indicate detected platform
    [[ "$output" =~ "darwin" ]] || [[ "$output" =~ "linux" ]] || [[ "$output" =~ "Building" ]]
}

# Additional P0: Multiple supported targets listed in error
@test "Unsupported target error lists all valid options" {
    run $PROJECT_ROOT/scripts/build-release.sh --target bad-target

    # Should mention all three supported platforms
    local supported_count=0
    [[ "$output" =~ "x86_64-apple-darwin" ]] && ((supported_count++))
    [[ "$output" =~ "aarch64-apple-darwin" ]] && ((supported_count++))
    [[ "$output" =~ "x86_64-unknown-linux-gnu" ]] && ((supported_count++))

    [ "$supported_count" -eq 3 ]
}
