{
  pkgs,
  catplus-converter,
}:
pkgs.dockerTools.buildLayeredImage {
  name = "ghcr.io/sdsc-ordes/catplus-converter";
  tag = catplus-converter.version;

  contents = [ catplus-converter ];

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
      "org.opencontainers.image.source" = "https://github.com/catplus-converter";
      "org.opencontainers.image.description" = catplus-converter.meta.description;
      "org.opencontainers.image.license" = catplus-converter.meta.license.spdxId;
    };
    User = "non-root";
  };
}
