# GLDT SWAP MONOREPO
This repository contains the source code for the GLDNFT to GLDT swapping mechanisms, as well as configuration and deployment files for GLDT ledger and indexer canisters.

See the [official website](https://gldt.org/) for more informations about the project.

> :warning: You are on the `master` branch, that currently doesn't contain any code for canisters. For now, checkout the `develop` branch to see the current source code being actively developed.  
> The `master` branch will become the default once the project will be launched in production.

## Project Structure:

- [`client/swap_app`](client/swap_app/): Contains the NextJS frontend for the [swap application](https://app.gldt.org).
- [`client/landing_page`](client/landing_page/): Contains the NextJS frontend for the [gldt.org landing page](https://gldt.org).
- [`client/explorer`](client/explorer): Contains the NextJS frontend for the [GLDT explorer](https://explorer.gldt.org).
- [`canister`](canister/): Contains the source code for the GLDT canisters (`gldt_core`, `gldt_fee_compensation`, and `gldt_ledger`, as well as other dependencies wasm files)

## Local development instructions
1. Clone this repository:
```sh
git clone https://gitlab.bochslerfinance.com/gldt/gldt-swap
```

2. Install the dependencies.
```sh
cd gldt-swap
npm install
```
Then you can launch a **front-end only** development server, with [HMR](https://webpack.js.org/concepts/hot-module-replacement/) with
```sh
npm run frontdev
```
or you can deploy the canister(s) and frontend to test in a [local replica](https://internetcomputer.org/docs/current/references/cli-reference/dfx-start#local-server-configuration) with
```sh
npm start
```
> **⚠️ Some resources (fonts, images...) will return errors (`400`) if accessed from `http://127.0.0.1:<REPLICA_PORT>/?canisterId=<FRONTEND_CANISTER_ID>`**. Instead, use the following url to access the locally deployed dapp: `http://<FRONTEND_CANISTER_ID>.localhost:<REPLICA_PORT>/`.

To redeploy your latest changes on `gldt_core`, `gldt_fee_compensation`, or on the frontend:
```sh
npm run restart
```
Once you finished your work, simply type `npm run stop` to stop the local canister execution.

## Deployment
TBD

### Keys management
TBD

...

## TODO
- [x] Scaffold project for selected development frameworks
- [ ] Initialize basic CI/CD (lint, test build, etc)
- [ ] Write CONTRIBUTING guide
- [ ] Setup keys management
- [ ] Update this README
