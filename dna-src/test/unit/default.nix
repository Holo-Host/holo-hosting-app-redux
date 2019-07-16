{ pkgs }:
let
  name = "hc-test-unit";

  script = pkgs.writeShellScriptBin name
  ''
  RUST_BACKTRACE=1 cargo test \
      --manifest-path zomes/host/code/Cargo.toml zomes/provider/code/Cargo.toml zomes/whoami/code/Cargo.toml \
      -- --nocapture
  '';
in
{
 buildInputs = [ script ];
}
