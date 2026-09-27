{ ... }:
{
    imports = [ ./basic.nix ];

    perSystem =
        {
            config,
            ...
        }:
        {
            packages.default = config.packages.crane; # aka `config.packages.my-crate;`
        };
}
