{ pkgs }:
let
  name = "hf-install";

  script = pkgs.writeShellScriptBin name
  ''
  rm -f dist/dna-src.dna.json
  mkdir -p dist
  hc package --output dist/dna-src.dna.json --strip-meta
  '';
in
{
 buildInputs = [ script ];
}
