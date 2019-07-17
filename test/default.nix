{ pkgs }:
let
  name = "hc-test";

  script = pkgs.writeShellScriptBin name
  ''
  hc-test-unit
  hc-test-e2e
  '';
in
{
 buildInputs = [
  script
 ]
 ++ (pkgs.callPackage ./unit { }).buildInputs
 ++ (pkgs.callPackage ./e2e { }).buildInputs
 ;
}
