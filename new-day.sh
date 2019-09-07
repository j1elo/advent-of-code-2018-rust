#!/usr/bin/env bash

#/ Prepare an empty project dir for a new Advent Of Code day.
#/
#/
#/ Arguments
#/ ---------
#/
#/ <DayNumber>
#/
#/   The day number, that will be used for the project's name.



# Shell setup
# -----------

# Bash options for strict error checking
set -o errexit -o errtrace -o pipefail -o nounset



# Check arguments
# ---------------

if [[ $# -lt 1 ]]; then
    echo "ERROR: Required <DayNumber> is missing" >&2
    exit 1
fi



# Create project
# --------------

NAME="$(printf "day-%02d" "$1")"

[[ -d "$NAME" ]] && {
    echo "ERROR: '$NAME' already exists"
    exit 1
}

cargo new --bin --vcs none "$NAME"

touch "$NAME/input.txt"

echo "Now add the '$NAME' to the 'workspace.members' array in Cargo.toml"
