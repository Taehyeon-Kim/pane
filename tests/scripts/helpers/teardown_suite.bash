#!/usr/bin/env bash
# BATS Suite Teardown - Runs once after all tests in a file

# Clean up temporary test directory
if [ -n "$BATS_TEST_TMPDIR" ] && [ -d "$BATS_TEST_TMPDIR" ]; then
    rm -rf "$BATS_TEST_TMPDIR"
fi

# Restore original PATH
export PATH="$ORIGINAL_PATH"
