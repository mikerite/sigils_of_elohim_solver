#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 1 ]]; then
    echo "usage: <path>" 1>&2
    exit 1
fi

if [[ !( -d $1 ) ]]; then
    echo "<path> must be a directory" 1>&2
    exit 1
fi

for f in "$1"/*; do
    cargo run --release --bin sigil_reader --quiet -- "$f"
done
