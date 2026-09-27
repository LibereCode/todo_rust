# SPDX-License-Identifier: EUPL
{
    inputs = {
        ## Foundation
        # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
        flake-parts = {
            url = "github:hercules-ci/flake-parts";
            inputs.nixpkgs.follows = "nixpkgs";
        };

        ## Rust
        ## NOTE prefer crane (over naersk)
        naersk = {
            url = "github:nix-community/naersk";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        crane = {
            url = "github:ipetkov/crane";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs =
        inputs:
        inputs.flake-parts.lib.mkFlake
            {
                inherit inputs;
            }
            {
                imports = [ ./nix/parts.nix ];
            };
}
