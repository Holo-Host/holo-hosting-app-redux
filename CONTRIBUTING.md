## Build release DNA build

Run `nix-build -A holo-hosting-app`.

## Develop against a local `holochain-rust` version

Run `nix-shell -I holochain-rust=../holochain-rust`, where `../holochain-rust`
is a path to `holochain-rust` checkout. Then, develop as usual: you can run
`make build`, `make test` as usual.

## Develop against built-in `holochain-rust` version

See previous section, run `nix-shell` without arguments.

## Use Holo cache to speed up builds

Add these lines to your ~/.config/nix/nix.conf:

    substituters = https://cache.holo.host/ https://cache.nixos.org/
    trusted-public-keys = cache.holo.host-1:lNXIXtJgS9Iuw4Cu6X0HINLu9sTfcjEntnrgwMQIMcE= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY=
