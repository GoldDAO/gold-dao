# GLDT swap frontend

This repository contains the source code for the GLDT page frontend, including swap interface, and the swap canister.  
As the front-end will be hosted on the Internet Computer, this projects uses [NextJS](https://nextjs.org/docs) and bundles a full static website with [client-side fetching](https://nextjs.org/docs/pages/building-your-application/data-fetching/client-side).

> :bulb: This project uses the [`pages` routing](https://nextjs.org/docs/getting-started/project-structure#pages-routing-conventions), and not the [`app` routing that just went out of beta](https://nextjs.org/blog/next-13-4#nextjs-app-router).

## Development

### Dfinity SDK

To be able to work on canisters development, or to test in a local canister execution environment, you may need to install the [Dfinity SDK](https://github.com/dfinity/sdk), and especially the [`dfx`](https://internetcomputer.org/docs/current/references/cli-reference/) CLI.

### Dependencies and local scripts

First install the dependencies with

```sh
npm install
```

Then you can launch a **front-end only** development server, with [HMR](https://webpack.js.org/concepts/hot-module-replacement/) with

```sh
npm run dev:swap_app
```

or you can deploy the canister(s) and frontend to test in a [local replica](https://internetcomputer.org/docs/current/references/cli-reference/dfx-start#local-server-configuration) with

```sh
npm start
```

And if you want to see some changes in the code, run

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
