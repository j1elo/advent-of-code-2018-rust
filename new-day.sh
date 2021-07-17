#!/usr/bin/env bash

#/ Prepare an empty project dir for a new Advent of Code day.
#/
#/ Arguments
#/ ---------
#/
#/ <DayNumber> - The day number, used for the sub-project's name.

# Bash options for strict error checking
set -o errexit -o errtrace -o pipefail -o nounset

# Check arguments
if [[ $# -lt 1 ]]; then
    echo "ERROR: Required <DayNumber> is missing" >&2
    exit 1
fi

# Create project
NAME="$(printf "day-%02d" "$1")"
if [[ -d "$NAME" ]]; then
    echo "ERROR: '$NAME' already exists"
    exit 1
fi
cargo new --bin --vcs none "$NAME"

# Create empty file for problem input
touch "$NAME/input.txt"

# Final instructions (too lazy to script)
echo "Now add '$NAME' to the 'workspace.members' array in Cargo.toml"
echo "and copy your problem input from the web into '$NAME/input.txt'"
