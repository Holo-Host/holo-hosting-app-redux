{ pkgs ? import ./pkgs.nix {} }: with pkgs;

{
  Holo-Hosting-App = buildDNA {
    name = "Holo-Hosting-App";
    src = gitignoreSource ./.;
  };
}
