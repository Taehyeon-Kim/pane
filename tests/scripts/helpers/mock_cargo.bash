#!/usr/bin/env bash
# Mock cargo command for testing

# Mock successful cargo build
mock_cargo_success() {
    # Mock cargo command
    cat > "$MOCK_BIN_DIR/cargo" <<'EOF'
#!/usr/bin/env bash
# Mock cargo - successful build

if [[ "$1" == "build" ]]; then
    # Simulate successful build
    echo "   Compiling pane v0.1.0"
    echo "   Compiling claude-tips v0.1.0"
    echo "    Finished release [optimized] target(s) in 1.23s"

    # Create mock binaries
    mkdir -p "$TARGET_DIR/release"
    echo "mock binary" > "$TARGET_DIR/release/pane"
    echo "mock binary" > "$TARGET_DIR/release/claude-tips"
    chmod +x "$TARGET_DIR/release/pane"
    chmod +x "$TARGET_DIR/release/claude-tips"
    exit 0
elif [[ "$1" == "metadata" ]]; then
    # Return mock metadata
    cat <<'METADATA'
{
  "packages": [
    {
      "name": "pane",
      "version": "0.1.0"
    }
  ]
}
METADATA
    exit 0
else
    echo "Unknown cargo command: $*" >&2
    exit 1
fi
EOF
    chmod +x "$MOCK_BIN_DIR/cargo"

    # Mock rustc command
    cat > "$MOCK_BIN_DIR/rustc" <<'EOF'
#!/usr/bin/env bash
# Mock rustc
if [[ "$1" == "--version" ]]; then
    echo "rustc 1.75.0 (mock)"
    exit 0
else
    exit 0
fi
EOF
    chmod +x "$MOCK_BIN_DIR/rustc"
}

# Mock cargo build failure
mock_cargo_failure() {
    cat > "$MOCK_BIN_DIR/cargo" <<'EOF'
#!/usr/bin/env bash
# Mock cargo - build failure

if [[ "$1" == "build" ]]; then
    echo "error: could not compile \`pane\`" >&2
    echo "error: aborting due to previous error" >&2
    exit 101
else
    echo "Unknown cargo command: $*" >&2
    exit 1
fi
EOF
    chmod +x "$MOCK_BIN_DIR/cargo"
}

# Remove cargo from PATH (simulate not installed)
mock_cargo_missing() {
    rm -f "$MOCK_BIN_DIR/cargo"
    rm -f "$MOCK_BIN_DIR/rustc"
}
