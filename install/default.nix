{ pkgs }:
let
  name = "hc-install";

  script = pkgs.writeShellScriptBin name
  ''
  rm -f dist/Holo-Hosting-App.dna.json
  mkdir -p dist
  hc package --output dist/Holo-Hosting-App.dna.json --strip-meta
  '';
in
{
 buildInputs = [ script ];
}
