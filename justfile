# Lists all recipes.
default:
    @just --list

# Runs the desktop application in debug mode.
run:
    @INITIATIVE_LOG=trace cargo run
