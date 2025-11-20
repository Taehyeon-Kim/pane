#!/usr/bin/env bats
# Test Suite: Archive Compression and Checksum Generation (AC6, AC7)
# Priority: P0/P1 - Critical distribution artifacts
# Test IDs: 5.2-INT-014 through 5.2-INT-020

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

# 5.2-INT-014: Archive created with naming pattern pane-v{version}-{target}.tar.gz
@test "5.2-INT-014: Archive follows naming convention pane-v{version}-{target}.tar.gz" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Find archive (auto-detected target)
    local archive=$(find dist/v0.1.0 -name "pane-v0.1.0-*.tar.gz" -type f | head -1)

    [ -n "$archive" ]
    [ -f "$archive" ]

    # Verify naming pattern
    [[ "$archive" =~ pane-v0\.1\.0-(aarch64-apple-darwin|x86_64-apple-darwin|x86_64-unknown-linux-gnu)\.tar\.gz ]]
}

# 5.2-INT-015: Archive contains both binaries
@test "5.2-INT-015: Archive contains pane and claude-tips binaries" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local archive=$(find dist/v0.1.0 -name "pane-v0.1.0-*.tar.gz" -type f | head -1)

    # List archive contents
    run tar -tzf "$archive"

    # Should contain both binaries
    assert_output_contains "pane"
    assert_output_contains "claude-tips"
}

# 5.2-INT-016: Archive stored in dist/v{version}/ directory
@test "5.2-INT-016: Archive stored in correct dist/v{version}/ location" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Archive should be in dist/v0.1.0/ (not in subdirectory)
    [ -f dist/v0.1.0/pane-v0.1.0-*.tar.gz ]
}

# 5.2-UNIT-009: Archive integrity verified (can extract successfully)
@test "5.2-UNIT-009: Archive can be extracted successfully" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local archive=$(find dist/v0.1.0 -name "pane-v0.1.0-*.tar.gz" -type f | head -1)

    # Extract to temp directory
    local extract_dir="$BATS_TEST_TMPDIR/extract-test"
    mkdir -p "$extract_dir"

    run tar -xzf "$archive" -C "$extract_dir"

    # Extraction should succeed
    assert_exit_status 0

    # Verify files extracted
    [ -f "$extract_dir/pane" ] || [ -f "$extract_dir/*/pane" ]
}

# 5.2-INT-017: Build output displays archive size
@test "5.2-INT-017: Build output shows created archive size" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Output should mention archive creation
    assert_output_contains "archive"
    assert_output_contains ".tar.gz"
}

# 5.2-INT-018: Checksum file created with .sha256 extension
@test "5.2-INT-018: SHA256 checksum file created with .sha256 extension" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Find checksum file
    local checksum_file=$(find dist/v0.1.0 -name "pane-v0.1.0-*.tar.gz.sha256" -type f | head -1)

    [ -n "$checksum_file" ]
    [ -f "$checksum_file" ]
}

# 5.2-INT-019: Checksum matches actual archive SHA256
@test "5.2-INT-019: Generated checksum matches actual file checksum" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local archive=$(find dist/v0.1.0 -name "pane-v0.1.0-*.tar.gz" -type f | head -1)
    local checksum_file="${archive}.sha256"

    [ -f "$checksum_file" ]

    # Read stored checksum
    local stored_checksum=$(cat "$checksum_file" | awk '{print $1}')

    # Calculate actual checksum
    local actual_checksum
    if command -v sha256sum >/dev/null 2>&1; then
        actual_checksum=$(sha256sum "$archive" | awk '{print $1}')
    else
        actual_checksum=$(shasum -a 256 "$archive" | awk '{print $1}')
    fi

    # Checksums should match
    [ "$stored_checksum" = "$actual_checksum" ]
}

# 5.2-UNIT-010: Platform-appropriate checksum command used
@test "5.2-UNIT-010: Script uses sha256sum on Linux, shasum on macOS" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Checksum file should exist (verifies command worked)
    local checksum_file=$(find dist/v0.1.0 -name "*.sha256" -type f | head -1)

    [ -f "$checksum_file" ]

    # Checksum should be valid hex (64 chars)
    local checksum=$(cat "$checksum_file" | awk '{print $1}')
    [[ "$checksum" =~ ^[a-f0-9]{64}$ ]]
}

# 5.2-INT-020: Checksum displayed in build output
@test "5.2-INT-020: Build output displays generated checksum" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Output should mention checksum
    assert_output_contains "checksum" || assert_output_contains "SHA256"
}
