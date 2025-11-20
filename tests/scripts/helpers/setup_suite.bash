#!/usr/bin/env bash
# BATS Suite Setup - Runs once before all tests in a file

# Project root is 3 levels up from helpers/ directory
export PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"

# Create temporary test directory
export BATS_TEST_TMPDIR="${BATS_TEST_TMPDIR:-$(mktemp -d)}"
export DIST_DIR="$BATS_TEST_TMPDIR/dist"
export TARGET_DIR="$BATS_TEST_TMPDIR/target"
export MOCK_BIN_DIR="$BATS_TEST_TMPDIR/bin"

# Save original PATH
export ORIGINAL_PATH="$PATH"

# Create mock directories
mkdir -p "$MOCK_BIN_DIR"
mkdir -p "$TARGET_DIR/release"
mkdir -p "$DIST_DIR"

# Helper functions available to all tests
export HELPERS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
