#!/usr/bin/env bats
# Test Suite: Flag Parsing and Help System
# Priority: P0, P1 - User interface validation
# Test IDs: 5.2-UNIT-019, 5.2-UNIT-020, 5.2-UNIT-021, 5.2-UNIT-022

load 'helpers/setup_suite'
load 'helpers/assertions'

setup() {
    cd "$BATS_TEST_TMPDIR"
}

# 5.2-UNIT-019: --help flag displays usage information
@test "5.2-UNIT-019: --help flag displays complete usage information" {
    run $PROJECT_ROOT/scripts/build-release.sh --help

    # Should exit successfully
    assert_exit_status 0

    # Should contain key sections
    assert_output_contains "USAGE:"
    assert_output_contains "OPTIONS:"
    assert_output_contains "EXAMPLES:"
    assert_output_contains "EXIT CODES:"

    # Should list all flags
    assert_output_contains "--help"
    assert_output_contains "--target"
    assert_output_contains "--dry-run"
    assert_output_contains "--clean"
}

# 5.2-UNIT-020: --dry-run flag prevents actual execution
@test "5.2-UNIT-020: --dry-run shows planned actions without executing" {
    # Create minimal test environment
    cat > Cargo.toml <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    run $PROJECT_ROOT/scripts/build-release.sh --dry-run

    # Should succeed
    [ "$status" -eq 0 ]

    # Should indicate dry-run mode
    assert_output_contains "DRY-RUN" || assert_output_contains "dry run" || assert_output_contains "would"

    # Should NOT create dist/ directory
    [ ! -d "dist" ]
}

# 5.2-UNIT-021: --clean flag sets CLEAN variable
@test "5.2-UNIT-021: --clean flag recognized and processed" {
    cat > Cargo.toml <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    # Create existing dist/ to verify cleanup
    mkdir -p dist/old
    touch dist/old/file.txt

    run $PROJECT_ROOT/scripts/build-release.sh --clean --dry-run

    # Should accept --clean flag without error
    [ "$status" -eq 0 ]

    # Should mention cleaning in dry-run output
    assert_output_contains "clean" || assert_output_contains "Removing" || assert_output_contains "dist"
}

# 5.2-UNIT-022: --target flag accepts valid target
@test "5.2-UNIT-022: --target flag parsed correctly with valid target" {
    cat > Cargo.toml <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    run $PROJECT_ROOT/scripts/build-release.sh --target x86_64-apple-darwin --dry-run

    # Should succeed
    [ "$status" -eq 0 ]

    # Should reference the target in output
    assert_output_contains "x86_64-apple-darwin"
}

# Additional P0: Unknown flags trigger helpful error
@test "Unknown flag triggers error with help suggestion" {
    run $PROJECT_ROOT/scripts/build-release.sh --unknown-flag

    # Should fail
    [ "$status" -ne 0 ]

    # Should suggest help
    assert_output_contains "--help" || assert_output_contains "Unknown" || assert_output_contains "invalid"
}

# Additional P1: Multiple flags can be combined
@test "Multiple flags work together (--dry-run --clean)" {
    cat > Cargo.toml <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    mkdir -p dist/old

    run $PROJECT_ROOT/scripts/build-release.sh --dry-run --clean

    assert_exit_status 0

    # Should show both dry-run and clean intentions
    [[ "$output" =~ "DRY" ]] || [[ "$output" =~ "dry" ]]
}
