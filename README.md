# GLDT SWAP MONOREPO

## Project Structure:

- `Client`: Contain the Next.js application
- `Canister`: Contain the source code of the GLDT Canister

## How to install and execute the app

1. Clone this repository:

```sh
git clone https://gitlab.bochslerfinance.com/gldt/gldt-swap
```

2. Install the dependencies.

```sh
npm install
```

3. Create .env file at the root of the project, and paste

```sh
DFX_NETWORK = 'local'
```

## Scripts:

### Start Next development server:

launch a **front-end only** development server, with [HMR](https://webpack.js.org/concepts/hot-module-replacement/)

```sh
npm run front:dev
```

the app will be available at `localhost:3000`

### Build Next frontend:

```sh
npm run build:front
```

### Deploy and start/stop/restart the app localy:

Deploy the canister(s) and frontend to test in a [local replica](https://internetcomputer.org/docs/current/references/cli-reference/dfx-start#local-server-configuration) with

```sh
npm run start
```

```sh
npm run stop
```

```sh
npm run restart
```

The app will be available at `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/`

#### ⚠️ Ressources (front, images...) will returns error 400 if access from `http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai`

## TODO

- [x] Scaffold project for selected development frameworks
- [x] Initialize basic CI/CD (lint, test build, etc)
- [ ] Write CONTRIBUTING guide
- [ ] Setup keys management
- [ ] Update this README
