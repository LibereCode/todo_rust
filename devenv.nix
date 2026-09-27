{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  indent_size = 2;
in
{

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
  ];

  files.".editorconfig" = {
    ini = {
      "*" = {
        inherit indent_size;
        indent_type = "space";
      };
    };
    copyMode = "copy";
  };

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    # channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "rustfmt"
      "rust-analyzer"

      "clippy"
      # "miri"
    ];
  };

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/basics/
  env.GREET = "devenv";
  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';
  # https://devenv.sh/basics/
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';
  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  scripts.clippy-pedantic.exec = "cargo clippy -- -W clippy::pedantic";
  # https://devenv.sh/processes/
  processes = {
    dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";
    clippy.exec = "cargo clippy";
    clippy-pedantic.exec = "${lib.getExe pkgs.watchexec} -n -- clippy-pedantic";
  };
  # https://devenv.sh/tasks/
  tasks = {
    # "devenv:enterShell".after = [ "myproj:setup" ];
    "checks:cargo".exec = "cargo check";
    "checks:clippy".exec = "clippy-pedantic";
  };

  /*
    devenv inputs add git-hooks github:cachix/git-hooks.nix
    devenv inputs add treefmt-nix github:numtide/treefmt-nix
  */
  treefmt = {
    enable = true;
    config = {
      programs = {
        nixfmt = {
          enable = true;
          indent = indent_size;
        };
        rustfmt.enable = true;
        # zizmore.enable = true; # Static Analyzer for GitHub-Actions
      };
    };
  };
  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    treefmt.enable = true;
    clippy.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
