#!/bin/sh

set -eu

# Fail if the required env variable is missing or empty
: "${WATCHER_DIRECTORIES:?CRITICAL ERROR: env variable WATCHER_DIRECTORIES is required}"

# Fail if any comma-separated directory does not exist.
# `xargs` trims whitespace, matching the binary's `.trim()`.
old_ifs=$IFS
IFS=','
for dir in $WATCHER_DIRECTORIES; do
    dir=$(printf '%s' "$dir" | xargs)
    if [ -z "$dir" ] || [ ! -d "$dir" ]; then
        echo "CRITICAL ERROR: WATCHER_DIRECTORIES contains a missing directory: '$dir'" >&2
        exit 1
    fi
done
IFS=$old_ifs

# Start files watcher in background
/files-watcher &
