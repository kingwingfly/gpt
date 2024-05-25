#! /bin/bash
set -e

export TERM=xterm-256color

# Statements waiting to be executed
statements=(
    "cargo fetch --locked"
    "cargo clippy --features cli -- -D warnings"
    "cargo clippy --features tui -- -D warnings"
    "cargo clippy --features cli,mock -- -D warnings"
    "cargo clippy --features tui,mock -- -D warnings"

    "cargo test --features mock -p gpt_core"

    "cargo doc --no-deps --all-features -p gpt_core"
)

# loop echo and executing statements
for statement in "${statements[@]}"; do
    echo "$(tput setaf 3)$statement$(tput sgr0)"
    eval $statement
    echo
done
