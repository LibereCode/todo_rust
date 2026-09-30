{
    ...
}:
{
    # imports = [
    #   # inputs.flake-parts.flakeModules.partitions
    #   inputs.treefmt-nix.flakeModule
    # ];
    #
    # perSystem =
    #   { pkgs, ... }:
    #   {
    #     treefmt = {
    #       projectRootFile = "flake.nix";
    #       programs = {
    #         nixfmt.enable = true;
    #         zizmor.enable = true;
    #         rustfmt.enable = true;
    #       };
    #     };
    #   };
    #: NOTE instead use formatter from **devenv**
}
