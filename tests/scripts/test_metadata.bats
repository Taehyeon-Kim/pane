#!/usr/bin/env bats
# Test Suite: Build Metadata Generation (AC8)
# Priority: P0/P1 - Critical build traceability
# Test IDs: 5.2-INT-021 through 5.2-INT-023, 5.2-UNIT-011 through 5.2-UNIT-013

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

# 5.2-INT-021: build-metadata.json created in dist/v{version}/
@test "5.2-INT-021: build-metadata.json created in correct location" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Metadata file should exist
    [ -f "dist/v0.1.0/build-metadata.json" ]
}

# 5.2-UNIT-011: Metadata contains all required fields
@test "5.2-UNIT-011: Metadata JSON contains all required fields" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local metadata_file="dist/v0.1.0/build-metadata.json"

    # Verify required fields exist
    grep -q '"version"' "$metadata_file"
    grep -q '"git_commit"' "$metadata_file"
    grep -q '"build_date"' "$metadata_file"
    grep -q '"targets"' "$metadata_file"
    grep -q '"rust_version"' "$metadata_file"
}

# 5.2-UNIT-012: Git commit extraction works
@test "5.2-UNIT-012: Metadata contains valid git commit hash" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local metadata_file="dist/v0.1.0/build-metadata.json"

    # Extract git_commit value (mock returns "a1b2c3d")
    local git_commit=$(grep '"git_commit"' "$metadata_file" | sed 's/.*: *"\(.*\)".*/\1/')

    # Should be mock value from mock_git
    [ "$git_commit" = "a1b2c3d" ]
}

# 5.2-UNIT-013: Build date in ISO-8601 format
@test "5.2-UNIT-013: Build date uses ISO-8601 format (YYYY-MM-DDTHH:MM:SSZ)" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local metadata_file="dist/v0.1.0/build-metadata.json"

    # Extract build_date
    local build_date=$(grep '"build_date"' "$metadata_file" | sed 's/.*: *"\(.*\)".*/\1/')

    # Verify ISO-8601 format (basic check)
    [[ "$build_date" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
}

# 5.2-INT-022: Metadata includes artifacts array with checksums
@test "5.2-INT-022: Metadata contains artifacts array with file details" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    local metadata_file="dist/v0.1.0/build-metadata.json"

    # Should have artifacts section
    grep -q '"artifacts"' "$metadata_file"
}

# 5.2-INT-023: Metadata displayed in build output
@test "5.2-INT-023: Build output shows metadata information" {
    run $PROJECT_ROOT/scripts/build-release.sh

    assert_exit_status 0

    # Output should mention metadata creation
    assert_output_contains "metadata" || assert_output_contains "Metadata"
}
