# Lists all recipes.
default:
    @just --list

# Runs the project in debug mode.
dev:
    @cargo run

# Runs the project in release mode.
run:
    @cargo run --release
