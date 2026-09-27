# hello-rust

## setup

```sh
nix flake init --template templates#rust

devenv init
# edit devenv

devenv inputs add git-hooks github:cachix/git-hooks.nix
devenv inputs add treefmt-nix github:numtide/treefmt-nix

devenv allow
z - ; z -

cargo init
```
