#!/usr/bin/env bash

set -euo pipefail

# --------------------------------------------------
# Args
# --------------------------------------------------

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 day01"
    echo "         $0 day01::example"
    exit 1
fi

cargo-watch -x "test $1"