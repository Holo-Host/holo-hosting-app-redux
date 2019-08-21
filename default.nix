{ pkgs ? import ./pkgs.nix {} }: with pkgs;

{
  example = buildZome {
    name = "example";
    src = gitignoreSource ./.;
  };
}
