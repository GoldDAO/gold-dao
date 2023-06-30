# GLDT SWAP MONOREPO

## Project Structure:

- `Client`: Contain the Next.js application
- `Canister`: Contain the source code of the GLDT Canister

## How to install and execute the app

1. Clone this repository: `git clone https://gitlab.bochslerfinance.com/gldt/gldt-swap`
2. Install the dependencies. (inside folder root)`npm install`

## Scripts:

### Start Next development server:

```sh
npm run frontdev
```

the app will be available at `localhost:3000`

### Build Next frontend:

```sh
npm run frontbuild
```

### Deploy and start/stop/restart the app localy:

```sh
npm run start
```

```sh
npm run stop
```

```sh
npm run restart
```

the app will be available at `http://bd3sg-teaaa-aaaaa-qaaba-cai.localhost:4943/`
⚠️ ressources (front, images...) will returns error 400 if access from `http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai`

## TODO

- [x] Scaffold project for selected development frameworks
- [x] Initialize basic CI/CD (lint, test build, etc)
- [ ] Write CONTRIBUTING guide
- [ ] Setup keys management
- [ ] Update this README
