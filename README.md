# GLDT SWAP MONOREPO

## Project Structure:

- [`client`](client/): Contains the NextJS frontend
- [`canister`](canister/): Contains the source code for the GLDT canisters (`gldt_core` and `gldt_ledger`, as well as other dependencies wasm files)

## How to install and execute the dapp locally

1. Clone this repository:
```sh
git clone https://gitlab.bochslerfinance.com/gldt/gldt-swap
```

2. Install the dependencies.
```sh
npm install
```

3. To deploy and run the project in a local replica, simply run:
```sh
npm start
```
The dapp will be available at `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/`
> **⚠️ Some resources (fonts, images...) will return errors (`400`) if accessed from `http://127.0.0.1:4943/?canisterId=bkyz2-fmaaa-aaaaa-qaaaq-cai`**

To redeploy your latest changes on `gldt_core` or on the frontend:
```sh
npm run deploy
```

If you need to test a redeploy operation for a canister (`gldt_core` or `gldt_ledger`), you can use one of those scripts:
```sh
scripts/deploy-gldt-core.sh --help
scripts/deploy-ledger.sh --help
```
Both contains safeguards against accidental deployments on staging or mainnet.

And to restart a fresh environment and redeploy all canisters, simply redo a `npm start`. It will stop the currently running replica if any, and restart a clean one, then redeploy everything.

## Other scripts:

### Start a NextJS development server:
Launch a **front-end only** development server, with [HMR](https://webpack.js.org/concepts/hot-module-replacement/)

```sh
npm run front:dev
```
The frontend will be available at `http://localhost:3000`

### Build all canisters, including frontend, and generate candid files and declarations:
```sh
npm run build
```

## Technical documentation
- Developers documentation still :construction: WIP (See code comments for now. Documentation will be automatically generated and published at a later time)
- Integrators documentation is :construction: [WIP in the wiki](https://gitlab.bochslerfinance.com/gldt/gldt-swap/-/wikis/home), and will be published in the frontend at a later time.

## DevOps documentation
- :construction: WIP on [this wiki page](https://gitlab.bochslerfinance.com/gldt/gldt-swap/-/wikis/Releases-and-Deployments-process).
