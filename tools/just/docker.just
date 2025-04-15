set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
[private]
default:
  just --list docker --no-aliases
  
# Upload converter image.
push-image:
   @echo "🐋 Uploading docker image"
   skopeo login ghcr.io
   skopeo copy \
    "docker-archive:{{root_dir}}/image/.output/package/catplus-converter-image" \
    "docker://ghcr.io/sdsc-ordes/catplus-converter:latest"

# Build the catplus-converter Nix package.
build-nix-package *args:
    @echo "Building package derivation"
    nix build {{root_dir}}/tools/nix#catplus-converter \
        --out-link {{root_dir}}/tools/package/.output/build/bin/catplus-converter

# Build the catplus-converter-image Nix Docker image.
build-nix-image *args:
    @echo "Building image derivation"
    nix build {{root_dir}}/tools/nix#catplus-converter-image \
        --out-link {{root_dir}}/tools/image/.output/package/catplus-converter-image