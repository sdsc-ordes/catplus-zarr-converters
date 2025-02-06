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

# Format all crates
fmt *args:
    cargo fmt

# Run the synth-converter.
run input_file output_file *args:
    cd "{{root_dir}}/synth-converter" && \
    cargo run --bin synth-converter {{root_dir}}/{{input_file}} {{root_dir}}/{{output_file}} {{args}}

# Enter a Nix development shell.
nix-develop *args:
    echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"
