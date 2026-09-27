{
  self,
  ...
}:
{
  perSystem =
    {
      pkgs,
      lib,
      ...
    }:
    {
      packages = {
        ## NOTE Using default packageWrapping
        basic = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "todo_rust";
          version = "0.0.1";
          src = "${self}";
          cargoHash = "sha256-iUIfILIRCVB87SjvxLTikptBm2A0f25uitEKfZZ+C7k=";
          meta = with lib; {
            description = "Simple todo.txt parser/writer, written in rust.";
            license = licenses.eupl12;
            maintainers = [ ];
          };
        });

        #NOTE: Prefer ../naersk.nix or (better) ../crane.nix
      };
    };
}
