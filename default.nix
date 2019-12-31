{ pkgs ? import ./pkgs.nix {}, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  holo-hosting-app = buildDNA {
    inherit shell;

    name = "holo-hosting-app";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
