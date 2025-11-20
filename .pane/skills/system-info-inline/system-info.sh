#!/bin/bash
# System Information Script
# Displays OS, uptime, memory, and disk information
# Compatible with macOS and Linux

echo "=== System Information ==="
echo ""

# OS Information
echo "OS: $(uname -s -r)"
echo ""

# Uptime
echo "Uptime: $(uptime)"
echo ""

# Memory Information (platform-specific)
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    echo "Memory (macOS):"
    vm_stat | head -n 10
else
    # Linux
    echo "Memory (Linux):"
    free -h
fi
echo ""

# Disk Usage
echo "Disk Usage:"
df -h / | tail -n +2
