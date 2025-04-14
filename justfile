# Lists all recipes.
default:
    @just --list

# Runs the frontend with hot reload.
dev:
    @npm run dev --prefix view

# Runs the project with the backend.
run:
    @cargo run
