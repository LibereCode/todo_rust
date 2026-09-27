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
                    cargoHash = "sha256-abIvnGu0Vr+VzPVnc9EPjuaoGaWDcyY43a3MS0oj2m0=";
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
