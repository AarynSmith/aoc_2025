#!/usr/bin/env bash

set -euo pipefail

# --------------------------------------------------
# Args
# --------------------------------------------------

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 1"
    echo "         $0 12"
    exit 1
fi

DAY_RAW="$1"

# zero-pad to 2 digits: 1 -> 01, 9 -> 09, 12 -> 12
printf -v DAY "%02d" "$DAY_RAW"

# Files & templates
BIN_FILE="src/bin/day${DAY}.rs"
DAY_FILE="src/day/day${DAY}.rs"
INPUT_FILE="input/day${DAY}.txt"

BIN_TEMPLATE="src/bin/dayNN.rs.tmpl"
DAY_TEMPLATE="src/day/dayNN.rs.tmpl"
INPUT_TEMPLATE="input/dayNN.txt.tmpl"

MOD_FILE="src/day/mod.rs"

# --------------------------------------------------
# Safety checks
# --------------------------------------------------

if [[ ! -f "$BIN_TEMPLATE" ]]; then
    echo "Error: binary template not found at $BIN_TEMPLATE"
    exit 1
fi

if [[ ! -f "$DAY_TEMPLATE" ]]; then
    echo "Error: day module template not found at $DAY_TEMPLATE"
    exit 1
fi

if [[ -e "$BIN_FILE" || -e "$DAY_FILE" ]]; then
    echo "Error: One or more target files already exist:"
    [[ -e "$BIN_FILE" ]] && echo "  $BIN_FILE"
    [[ -e "$DAY_FILE" ]] && echo "  $DAY_FILE"
    exit 1
fi

# --------------------------------------------------
# Create src/bin/dayNN.rs from template
# --------------------------------------------------

echo "Creating $BIN_FILE..."
sed "s/NN/${DAY}/g" "$BIN_TEMPLATE" > "$BIN_FILE"

# --------------------------------------------------
# Create src/day/dayNN.rs from template
# --------------------------------------------------

echo "Creating $DAY_FILE..."
sed "s/NN/${DAY}/g" "$DAY_TEMPLATE" > "$DAY_FILE"

# --------------------------------------------------
# Create input/dayNN.txt (optional template support)
# --------------------------------------------------

if [[ -f "$INPUT_TEMPLATE" ]]; then
    echo "Creating $INPUT_FILE from template..."
    sed "s/NN/${DAY}/g" "$INPUT_TEMPLATE" > "$INPUT_FILE"
else
    # No template? just create an empty file if it doesn't exist
    if [[ ! -e "$INPUT_FILE" ]]; then
        echo "Creating empty $INPUT_FILE..."
        touch "$INPUT_FILE"
    else
        echo "Note: $INPUT_FILE already exists; leaving it as-is."
    fi
fi

# --------------------------------------------------
# Update src/day/mod.rs
# --------------------------------------------------

MOD_LINE="pub mod day${DAY};"
USE_LINE="pub use day${DAY}::solve_from_file as day${DAY};"

if [[ ! -f "$MOD_FILE" ]]; then
    echo "Error: mod file not found at $MOD_FILE"
    exit 1
fi

# Add `pub mod dayNN;` if missing
if ! grep -q "$MOD_LINE" "$MOD_FILE"; then
    echo "Updating $MOD_FILE (mod line)…"
    echo "$MOD_LINE" >> "$MOD_FILE"
fi

# Add `pub use dayNN::solve_from_file as dayNN;` if missing
if ! grep -q "$USE_LINE" "$MOD_FILE"; then
    echo "Updating $MOD_FILE (pub use)…"
    echo "$USE_LINE" >> "$MOD_FILE"
fi

echo "Done!"
echo "Created:"
echo "  $BIN_FILE"
echo "  $DAY_FILE"
echo "  $INPUT_FILE"
echo "Updated $MOD_FILE"
