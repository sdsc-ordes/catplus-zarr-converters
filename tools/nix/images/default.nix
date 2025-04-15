{
  pkgs,
  catplus-converters,
}:
pkgs.dockerTools.buildLayeredImage {
  name = "ghcr.io/sdsc-ordes/catplus-converters";
  tag = catplus-converters.version;

  contents = [ 
    catplus-converters
    pkgs.just
    pkgs.bashInteractive
    ];

  fakeRootCommands = ''
    ${pkgs.dockerTools.shadowSetup}
    groupadd -r non-root
    useradd -r -g non-root non-root
    mkdir -p /workspace
    chown non-root:non-root /workspace
  '';
  enableFakechroot = true;

  config = {
    Entrypoint = [ "converter" ];
    WorkingDir = "/workspace";
    Labels = {
      "org.opencontainers.image.source" = "https://github.com/catplus-converters";
      "org.opencontainers.image.description" = catplus-converters.meta.description;
      "org.opencontainers.image.license" = catplus-converters.meta.license.spdxId;
    };
    User = "non-root";
  };
}
