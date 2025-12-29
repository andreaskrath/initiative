# Lists all recipes.
default:
    @just --list

# Runs the desktop application in debug mode.
run log_level='debug':
    #!/usr/bin/env bash
    set -euo pipefail
    # Extract workspace member crate names
    crates=$(sed -n '/^\[workspace\]/,/^\[/{ /members = \[/,/\]/p }' Cargo.toml | \
        grep -v 'members' | grep -v '^\[' | grep -v '\]' | tr -d ' ",' | grep -v '^$' | \
        while read member; do \
            [ -f "$member/Cargo.toml" ] && grep -m1 '^name = ' "$member/Cargo.toml" | sed 's/name = "\(.*\)"/\1/'; \
        done | paste -sd,)
    # Build RUST_LOG: default to error, then set each local crate to the specified level
    rust_log="error"
    for crate in ${crates//,/ }; do
        rust_log="${rust_log},${crate}={{log_level}}"
    done
    # echo "RUST_LOG=${rust_log}"
    RUST_LOG="${rust_log}" cargo run
