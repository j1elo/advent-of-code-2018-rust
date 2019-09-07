#!/usr/bin/env bash

#/ Run a specified Advent Of Code day.
#/
#/
#/ Arguments
#/ ---------
#/
#/ <DayNumber>
#/
#/   The day number, that will be used to specify the target to run.



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

[[ -d "$NAME" ]] || {
    echo "ERROR: '$NAME' doesn't exist"
    exit 1
}

cargo run --bin "$NAME" < "$NAME/input.txt"
