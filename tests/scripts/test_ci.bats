#!/usr/bin/env bats
# Test Suite: CI Environment Compatibility (AC10)
# Priority: P0/P1 - Critical for GitHub Actions integration
# Test IDs: 5.2-E2E-003, 5.2-E2E-004, 5.2-INT-027, 5.2-INT-028

load 'helpers/setup_suite'
load 'helpers/mock_cargo'
load 'helpers/mock_git'
load 'helpers/mock_uname'
load 'helpers/assertions'

setup() {
    export PATH="$MOCK_BIN_DIR:/usr/bin:/bin"

    cat > "$BATS_TEST_TMPDIR/Cargo.toml" <<'EOF'
[package]
name = "pane"
version = "0.1.0"
EOF

    cd "$BATS_TEST_TMPDIR"

    mock_cargo_success
    mock_git_success
    mock_uname_success
}

# 5.2-E2E-003: Script runs non-interactively in CI environment
@test "5.2-E2E-003: Script completes without user interaction when CI=true" {
    export CI=true

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Should complete successfully
    assert_output_contains "✓" || assert_output_contains "Success" || assert_output_contains "Finished"
}

# 5.2-E2E-004: Script respects GITHUB_ACTIONS environment variable
@test "5.2-E2E-004: Script works correctly with GITHUB_ACTIONS=true" {
    export CI=true
    export GITHUB_ACTIONS=true
    export GITHUB_WORKSPACE="$BATS_TEST_TMPDIR"

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Build artifacts should be created
    [ -d "dist/v0.1.0" ]
}

# 5.2-INT-027: No prompts or user interaction during build
@test "5.2-INT-027: Build runs completely non-interactively" {
    export CI=true

    # Run with stdin closed to detect any interactive prompts
    run bash -c "$PROJECT_ROOT/scripts/build-release.sh </dev/null"

    assert_exit_status 0

    # Should not wait for user input
    # Successful completion verifies non-interactive operation
}

# 5.2-INT-028: Exit code 0 on successful CI build
@test "5.2-INT-028: Successful build returns exit code 0 for CI" {
    export CI=true
    export GITHUB_ACTIONS=true

    run $PROJECT_ROOT/scripts/build-release.sh

    # Must be exact exit code 0 for CI success detection
    assert_exit_status 0
}

# Additional: Artifacts uploadable in GitHub Actions workflow
@test "CI artifacts are in predictable dist/ location for upload" {
    export CI=true
    export GITHUB_ACTIONS=true

    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Artifacts should be in dist/ for GitHub Actions upload
    [ -d "dist/v0.1.0" ]

    # Archive should exist for artifact upload
    local archive_count=$(find dist/v0.1.0 -name "*.tar.gz" | wc -l)
    [ "$archive_count" -ge 1 ]
}
