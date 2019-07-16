{ pkgs }:
let
  name = "hf-install";

  script = pkgs.writeShellScriptBin name
  ''
  rm -f dist/DeepKey.dna.json
  mkdir -p dist
  hc package --output dist/DeepKey.dna.json --strip-meta
  '';
in
{
 buildInputs = [ script ];
}
