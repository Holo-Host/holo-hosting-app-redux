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

    nativeBuildInputs = [
      cmake # required by wabt
      binaryen
      wasm-gc
      wabt
    ]
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
