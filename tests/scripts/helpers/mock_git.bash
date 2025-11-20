#!/usr/bin/env bash
# Mock git command for testing

# Mock successful git commands
mock_git_success() {
    cat > "$MOCK_BIN_DIR/git" <<'EOF'
#!/usr/bin/env bash
# Mock git - successful commands

if [[ "$1" == "rev-parse" ]] && [[ "$2" == "--short" ]]; then
    echo "a1b2c3d"
    exit 0
elif [[ "$1" == "--version" ]]; then
    echo "git version 2.39.0"
    exit 0
else
    echo "Unknown git command: $*" >&2
    exit 1
fi
EOF
    chmod +x "$MOCK_BIN_DIR/git"
}

# Remove git from PATH (simulate not installed)
mock_git_missing() {
    # Create a mock that simulates "command not found"
    cat > "$MOCK_BIN_DIR/git" <<'EOF'
#!/usr/bin/env bash
# Mock git - simulate not installed
echo "bash: git: command not found" >&2
exit 127
EOF
    chmod +x "$MOCK_BIN_DIR/git"
}
