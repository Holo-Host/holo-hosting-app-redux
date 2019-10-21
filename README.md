# Holo Hosting App

![GitHub last commit](https://img.shields.io/github/last-commit/holo-host/Holo-Hosting-App.svg) ![GitHub](https://img.shields.io/github/license/holo-host/Holo-Hosting-App.svg)
[![CircleCI](https://circleci.com/gh/Holo-Host/holo-hosting-app.svg?style=svg)](https://circleci.com/gh/Holo-Host/holo-hosting-app)

**Status:** Closed-Alpha. Early development and testing.

The hosting app is a space for Hosts and App Providers to interact

**App Providers** list their apps along with their hosting preferences. This makes the app available for hosting. It registers a domain name for the app and other stuff.

**Hosts** declare their intention to host an app and record their private signed service logs here. They can also compile invoices for their app Providers.

## How to run test?
```
nix-test
```
> Note since this repo is in dev mode, you would have to pull the [holochain-rust](https://github.com/holochain/holochain-rust) repo in the same folder you pull this repo into. This is required to run the hc test with the latest nodejs_conductor in the holochain-rust.

## How to build DNA?
After running the cmd bellow you will find the `.dna.json` in the `dist/` folder

```
nix-build
```
# How to quickly spin up an agent
> This could be used to test the [UI](https://github.com/Holo-Host/holo-hosting-app_GUI)
### Steps to run two DNAs:
  1. Open two terminals at @Holo-Hosting-App

  2. Update the `start-dna-agent1` script in the `package.json`
      - Find the `start-dna-agent1` script inside the `package.json`
      - Update the **HC_N3H_PATH** to path of the n3h repo on your local device.(eg: HC_N3H_PATH=/home/lisa/n3h)

  3. In the first terminal, run agent1's DNA
        ```
        npm run start-dna-agent1
        ````

  4. Update the `start-dna-agent2` script in the `package.json` BEFORE running agent2
        - Find the `start-dna-agent2` script inside the `package.json`
        - Update the **HC_N3H_PATH** to path of the n3h repo on your local device.
        - Add the **HC_N3H_BOOTSTRAP_NODE** as provided in the networking details within the terminal when running agent1's DNA. (As shown on line #9 in the terminal snippet below.)
        - Add the **HC_N3H_IPC_URI** as provided in the networking details within the terminal when running agent1's DNA. (As shown on line #3 in the terminal snippet below.)

```=
(wss-connection) [i] listening at wss://127.0.0.1:41249/
Network spawned with bindings:
	 - ipc: wss://127.0.0.1:41249/
	 - p2p: []
(p2p-hackmode) [i] node-id hkaQGtTemslrK79wHSwqQONetfVxUenB-ElgD1-RnnmxguJO_VCPdK2ZPKADdIjpu0xvI1yF6HTjD132jLA3rOMWTZKVR605
(wss-server-utils) [i] loaded rsa fingerprint faqnfO4LeJSOWCvVLLjXSN+7TPQ=
(wss-connection) [i] listening at wss://192.168.0.7:42179/
(@hackmode@) [i] p2p bound wss://192.168.0.7:42179/?a=hkaQGtTemslrK79wHSwqQONetfVxUenB-ElgD1-RnnmxguJO_VCPdK2ZPKADdIjpu0xvI1yF6HTjD132jLA3rOMWTZKVR605

```

  5. In the second terminal, run agent2's DNA
       ```
       npm run start-dna-agent2
       ```


## Documentaion:

**[KV Store API](https://hackmd.io/_zUswSixRRK0NpnvoK1dLA?both):** Doc required by the KV Store to communicate with the HHA


## Built With
* [Holochain-rust  v0.0.32-alpha2](https://github.com/holochain/holochain-rust)
* [HDK](https://developer.holochain.org/api/latest/hdk/)

## Authors
* **Joel Ulahanna** - [Zo-El](https://github.com/zo-el)
* **Lisa Jetton** - [JettTech](https://github.com/JettTech)

## License

Copyright (C) 2017, The MetaCurrency Project (Eric Harris-Braun, Arthur Brock, et. al.)

This program is free software: you can redistribute it and/or modify it under the terms of the license provided in the LICENSE file (GPLv3.0). This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

Note: We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.
