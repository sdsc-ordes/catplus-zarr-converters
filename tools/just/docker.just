set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
[private]
default:
  just --list docker --no-aliases
  
# Upload converters image.
push-image:
   @echo "🐋 Uploading docker image"
   skopeo copy \
    "docker-archive:{{root_dir}}/tools/image/.output/package/catplus-converters-image" \
    "docker://ghcr.io/sdsc-ordes/catplus-converters:latest"

# Build the catplus-converters Nix package.
build-nix-package *args:
    @echo "Building package derivation"
    nix build {{root_dir}}/tools/nix#catplus-converters \
        --out-link {{root_dir}}/tools/package/.output/build/bin/catplus-converters

# Build the catplus-converters-image Nix Docker image.
build-nix-image *args:
    @echo "Building image derivation"
    nix build {{root_dir}}/tools/nix#catplus-converters-image \
        --out-link {{root_dir}}/tools/image/.output/package/catplus-converters-image