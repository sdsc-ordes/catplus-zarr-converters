#!/usr/bin/env bash
set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
default:
    just --list

# Build all crates
build *args:
    cargo build

# Test all crates
test *args:
    cargo test

alias fmt := format
# Format all crates
format *args:
    cargo fmt {{args}}

# Run the converter.
run input_type input_file output_file *args:
    cd "{{root_dir}}/src/converter" && \
    cargo run --bin converter \
        "{{input_type}}" \
        "{{root_dir}}/{{input_file}}" \
        "{{root_dir}}/{{output_file}}" {{args}}

# Enter a nix interpreter with loaded flake.
nix-repl:
    nix repl ./tools/nix

# Enter a Nix development shell.
nix-develop *args:
    echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"

# Manage docker.
mod docker './tools/just/docker.just'