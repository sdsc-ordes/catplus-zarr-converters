#!/usr/bin/env bash
set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`
shapes_url := "https://github.com/sdsc-ordes/catplus-ontology/releases/download/v0.1.0/catplus_ontology.ttl"

# Default recipe to list all recipes.
default:
    just --list --no-aliases

# Build all crates
build *args:
    cargo build

# Test all crates
test *args:
    cargo test {{args}}

alias fmt := format
# Format all crates
format *args:
    cargo fmt {{args}}

# Lint all code
lint *args:
  cargo clippy \
    --no-deps \
    -- -D warnings -A clippy::needless_return {{args}}

alias dev := nix-develop
# Enter a Nix development shell.
nix-develop *args:
    @echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"

# Run the converter.
convert *args:
  cargo run --bin converter -- {{args}}

# Run the validation.
[group('validation')]
validate +args:
  cargo run \
    --bin validation \
    -- \
      --endpoint http://localhost:8001 \
      {{args}}

# Start validation server.
[group('validation')]
shacl-start:
  docker run \
    -d -it --rm \
    --name catplus-shacl-api \
    -p 8001:8000 \
    -e SHAPES_URL={{shapes_url}} \
    ghcr.io/sdsc-ordes/shacl-api:develop

# Stop validation server.
[group('validation')]
shacl-stop:
  docker stop catplus-shacl-api &

