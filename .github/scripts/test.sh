#! /bin/bash
set -e

export TERM=xterm-256color

# Statements waiting to be executed
statements=(
    "cargo fetch --locked"
    "cargo clippy --all-features --all-targets -- -D warnings"
    "cargo test -p gpt_core --features mock"
    "cargo doc --no-deps -p gpt_core"
)

# loop echo and executing statements
for statement in "${statements[@]}"; do
    echo "$(tput setaf 3)$statement$(tput sgr0)"
    eval $statement
    echo
done
