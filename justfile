# Lists all recipes.
default:
    @just --list

# Runs the frontend with hot reload.
dev:
    @npm run dev --prefix view

# Runs the backend with whatever frontend artifacts exist.
run:
    @cargo run

# Runs the backend after building a new frontend bundle.
full:
    @BUILD_FRONTEND=true cargo run
