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

# Upload converter image.
upload-image:
   skopeo login ghcr.io
   skopeo copy \
    "docker-archive:.output/package/catplus-converter-image" \
    "docker://ghcr.io/sdsc-ordes/catplus-converter:latest"

# Build the catplus-converter Nix package.
nix-package *args:
    nix build ./tools/nix#catplus-converter \
        --out-link .output/build/bin/catplus-converter

# Build the catplus-converter-image Nix Docker image.
nix-image *args:
    nix build ./tools/nix#catplus-converter-image \
        --out-link .output/package/catplus-converter-image

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
