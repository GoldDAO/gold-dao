# GLDT SWAP MONOREPO

This repository contains the source code for all the Gold DAO controller canisters.

See the [official website](https://gldt.org/) for more informations about the project.

## Project Structure

### Backend canisters

All the backend canisters are included in the folder [`backend/canisters`](backend/canisters/). This includes

- [`gldt_swap (WIP)`](backend/canisters/gldt_swap/): The core logic for the swapping in between `GLD NFT` and `GLDT`.
- [`icp_neuron`](backend/canisters/icp_neuron/): The canister that controls the ICP neurons of the Gold DAO.
- [`sns_rewards (WIP)`](backend/canisters/sns_rewards/): The canister that manages the staking rewards of the Gold DAO. governance participants.

### Frontend canisters

The frontend canisters of the project are included in the folder [`client`](client/)

- [`swap_app`](client/swap_app/): Contains the NextJS frontend for the [swap application](https://app.gldt.org).
- [`landing_page`](client/landing_page/): Contains the NextJS frontend for the [gldt.org landing page](https://gldt.org).
- [`explorer`](client/explorer): Contains the NextJS frontend for the [GLDT explorer](https://explorer.gldt.org).

## Local development instructions

### Clone this repository

```sh
git clone git@github.com:GoldDAO/gldt-swap.git
```

(Or from the DAOlink internal Gitlab url, from which this Github repo is automatically mirrored)

### Install the dependencies

First, ensure that you have [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [`ic-wasm`](https://github.com/dfinity/ic-wasm) installed, as well as [`wasmtime`](https://wasmtime.dev).  
Then from the repository's root folder:

```sh
npm install
```

### Deploy and run the project in a local replica

To deploy and run the project in a local replica, simply run:

```sh
npm start
```

> **⚠️ Some resources (fonts, images...) will return errors (`400`) if accessed from `http://127.0.0.1:<REPLICA_PORT>/?canisterId=<FRONTEND_CANISTER_ID>`**. Instead, use the following url to access the locally deployed dapp: `http://<FRONTEND_CANISTER_ID>.localhost:<REPLICA_PORT>/`.

To redeploy your latest changes on `gldt_swap`, or on the frontend:

```sh
npm run deploy
```

If you need to test a redeploy operation for a canister (`gldt_swap` or `gldt_ledger`), you can use one of those scripts:

```sh
scripts/deploy-gldt-core.sh --help
scripts/deploy-gldt-fee-compensation.sh --help
scripts/deploy-ledger.sh --help
```

Each one contains safeguards against accidental deployments on staging or mainnet.

And to restart a fresh environment and redeploy all canisters, simply redo a `npm start`. It will stop the currently running replica if any, and restart a clean one, then redeploy everything.

## Frontend development server

All frontends are developed in React, using the NextJS compiler to generate the static pages to be deployed in the corresponding assets canisters on the Internet Computer.  
To launch a **front-end only** development server, with [HMR](https://webpack.js.org/concepts/hot-module-replacement/):

```sh
npm run dev:gldt_swap_app # For the GLDT swap application frontend
```

The frontend development server will be available at `http://localhost:3000`.

## Locally build all canisters, frontends, and generate candid files and declarations

```sh
npm run build
```

## Lima Docker for Reproducible Builds

### Install Lima

Follow the instructions at [https://github.com/lima-vm/lima#installation](https://github.com/lima-vm/lima#installation).
You can use this config file to start: [https://github.com/lima-vm/lima/blob/master/examples/docker.yaml](https://github.com/lima-vm/lima/blob/master/examples/docker.yaml) and run the commands below.

Note: If you are running on an ARM chip (e.g. Apple M1), you have to add the line `arch: "x86_64"` to the top of the `docker.yaml` file (see more infos about this [here](https://lima-vm.io/docs/config/multi-arch/)). This will drastically reduce the execution speed of this (building can take >1hr), however, is required on ARM chips! See [`build/docker.yaml`](build/docker.yaml) for an example.

```sh
limactl start docker.yaml
docker context create lima-docker --docker "host=unix:///Users/$USER/.lima/docker/sock/docker.sock"
docker context use lima-docker
```

### Building the Reproducible Image

This step can take a while. Sit back and enjoy some coffee:

```sh
docker build -t gld_reproducible_build -f ./build/Dockerfile_backend .
```

### Running the Reproducible Build

Define the canister you want to build. Check the valid canister names in section [Backend Canisters](#backend-canisters).

```sh
export CANISTER_NAME=<THE_CANISTER_NAME>
```

e.g.

```sh
export CANISTER_NAME=gldt_swap
```

Build the canister

```sh
docker run -v /tmp/lima/:/builds/gldt/gldt-swap/backend/canisters/$CANISTER_NAME/target/wasm32-unknown-unknown/release/ -e CANISTER_NAME=$CANISTER_NAME gld_reproducible_build
```

### Generated WASM Files

```sh
/tmp/lima/$CANISTER_NAME.wasm
/tmp/lima/$CANISTER_NAME_canister.wasm
/tmp/lima/$CANISTER_NAME_canister.wasm.gz
```

### Verify the integrity of the files by computing their SHA256 hashes

We are currently using the `$CANISTER_NAME_canister.wasm.gz` in production, so you can check using:

```sh
shasum -a 256 /tmp/lima/${CANISTER_NAME}_canister.wasm.gz
```

Compare this value to the wasm hash that is provided with the upgrade proposal.

## Technical documentation

- Developers documentation still :construction: WIP (See code comments for now. Documentation will be automatically generated and published at a later time)

## DevOps documentation

- :construction: WIP on DAOlink's internal Gitlab wiki

### Notes

## Integration tests

you may need to set the max open files on your current shell before running `cargo test` by first running `ulimit -n 10240`
