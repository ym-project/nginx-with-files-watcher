#!/bin/sh

set -eu

# Fail if the required env variable is missing or empty
: "${CERT_WATCHER_DIR:?CRITICAL ERROR: env variable CERT_WATCHER_DIR is required}"

# Fail if the directory does not exist
if [ ! -d "$CERT_WATCHER_DIR" ]; then
    echo "CRITICAL ERROR: CERT_WATCHER_DIR '$CERT_WATCHER_DIR' is not a directory" >&2
    exit 1
fi

# Start cert watcher in background
/cert-watcher &
