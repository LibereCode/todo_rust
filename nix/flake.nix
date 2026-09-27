{ ... }:
{
  systems = [
    "x86_64-linux"
    "aarch64-linux"
    "x86_64-darwin"
    "aarch64-darwin"
  ];

  imports = [
    ./packages

    ./naersk.nix
    ./crane.nix # <- preferred
  ];

  ## XXX What? Why ???
  # perSystem = { system, ... }: {
  #   # You can use `extend' to extend the packages with an overlay (or use
  #   # `import inputs.nixpkgs { ... }`).
  #   _module.args.pkgs = inputs.nixpkgs.legacyPackages.${system};
  # };
}
