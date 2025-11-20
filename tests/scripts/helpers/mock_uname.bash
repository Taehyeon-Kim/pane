#!/usr/bin/env bash
# Mock uname command for testing

# Mock uname for current platform
mock_uname_success() {
    cat > "$MOCK_BIN_DIR/uname" <<'UNAME_EOF'
#!/usr/bin/env bash
# Mock uname
if [[ "$1" == "-s" ]]; then
    echo "Darwin"
elif [[ "$1" == "-m" ]]; then
    echo "arm64"
else
    uname "$@"
fi
UNAME_EOF
    chmod +x "$MOCK_BIN_DIR/uname"
}
