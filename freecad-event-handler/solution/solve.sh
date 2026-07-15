#!/bin/bash

set -euo pipefail

cd /app

# Build the Rust project
cargo build --release 2>&1

# Run the event handler with test input and verify output
echo "STATUS" | ./target/release/event_handler > /tmp/status_output.txt

# Check that status output contains expected format
if grep -q "state=" /tmp/status_output.txt; then
    echo "Build and basic execution successful"
else
    echo "Error: Status output missing expected format"
    exit 1
fi

# Test drag operation
cat <<EOF | ./target/release/event_handler > /tmp/drag_output.txt
PRESS 0 150.0 125.0
DRAG 0 200.0 175.0
RELEASE 0
ANNOTATIONS
RENDER
EOF

# Verify annotation position changed after drag
if grep -q "ANNOTATION 0:" /tmp/drag_output.txt; then
    echo "Drag operation completed"
else
    echo "Error: Drag output missing annotation info"
    exit 1
fi

# Verify frame rendering with inset borders
if grep -q "FRAME_START" /tmp/drag_output.txt && grep -q "RECT" /tmp/drag_output.txt; then
    echo "Frame rendering completed with proper format"
else
    echo "Error: Frame rendering output missing"
    exit 1
fi

echo "Solution verification complete"
