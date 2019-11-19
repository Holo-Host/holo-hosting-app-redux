{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  holo-hosting-app = buildDNA {
    name = "holo-hosting-app";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
