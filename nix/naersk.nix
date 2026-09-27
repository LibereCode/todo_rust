{
    self,
    inputs,
    ...
}:
{
    perSystem =
        {
            pkgs,
            # self',
            ...
        }:
        let
            naersk' = pkgs.callPackage inputs.naersk { };
        in
        {
            packages = {
                naersk = naersk'.buildPackage {
                    src = self;
                };

                # default = self'.packages.naersk; # NOTE See ./crane.nix
            };
        };
}
