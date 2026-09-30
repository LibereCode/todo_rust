{
    inputs,
    self,
    ...
}:
{
    perSystem =
        {
            self',
            pkgs,
            config,
            ...
        }:
        let
            craneLib = inputs.crane.mkLib pkgs;

            #: Common arguments can be set here to avoid repeating them later
            #: NOTE changes here will rebuild all dependency crates
            commonArgs = {
                src = craneLib.cleanCargoSource "${self}";
                strictDeps = true;

                buildInputs = [
                    # Add additional build inputs here
                ]
                ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
                    # Additional darwin specific inputs can be set here
                    pkgs.libiconv
                ];
            };

            my-crate = craneLib.buildPackage (
                commonArgs
                // {
                    cargoArtifacts = craneLib.buildDepsOnly commonArgs;

                    # Additional environment variables or build phases/hooks can be set
                    # here *without* rebuilding all dependency crates
                    # MY_CUSTOM_VAR = "some value";
                }
            );
        in
        {
            ## $ nix flake check
            checks = { inherit (config.packages) my-crate; };

            packages = {
                ## $ nix build .#my-crate
                inherit my-crate;
                ## $ nix build .
                crane = config.packages.my-crate;
            };

            ## XXX `apps` not needed. Fallback to running packages.
            ## $ nix run .
            ## $ nix run .#my-crate
            # apps.default = flake-utils.lib.mkApp { drv = my-crate; }; # legacy w flake-utils

            ## $ nix develop .
            devShells.default = craneLib.devShell {
                # Inherit inputs from checks.
                inherit (self') checks;

                ## Additional dev-shell environment variables can be set directly
                ## MY_CUSTOM_DEVELOPMENT_VAR = "something else";

                ## Extra inputs can be added here.
                ## Default: `with pkgs; [ cargo rustc ];`
                packages = [
                    # pkgs.ripgrep
                ];
            };

        };
}
