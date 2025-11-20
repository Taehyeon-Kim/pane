#!/usr/bin/env bash
# Custom assertions for BATS tests

# Assert file exists
assert_file_exists() {
    local file="$1"
    if [ ! -f "$file" ]; then
        echo "Expected file to exist: $file" >&2
        return 1
    fi
}

# Assert directory exists
assert_dir_exists() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        echo "Expected directory to exist: $dir" >&2
        return 1
    fi
}

# Assert output contains string
assert_output_contains() {
    local expected="$1"
    if [[ ! "$output" =~ $expected ]]; then
        echo "Expected output to contain: $expected" >&2
        echo "Actual output: $output" >&2
        return 1
    fi
}

# Assert file contains string
assert_file_contains() {
    local file="$1"
    local expected="$2"
    if ! grep -q "$expected" "$file"; then
        echo "Expected file $file to contain: $expected" >&2
        return 1
    fi
}

# Assert exit status
assert_exit_status() {
    local expected="$1"
    if [ "$status" -ne "$expected" ]; then
        echo "Expected exit status $expected, got $status" >&2
        return 1
    fi
}
