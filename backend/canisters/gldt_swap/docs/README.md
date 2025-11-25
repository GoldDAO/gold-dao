# GLDT Swap

## Description

This document serves the purpose of informing developers and technical reviewers key details of the GLDT Swap. 


## Introduction & terminology
GLDT Swap (GS) serves the main purpose of swapping GLD NFT's to and from the fungible token GLDT. When swapping a ratio of 
1g NFT : 100 GLDT determines how much of each type can be swapped so for example a 10g NFT would be valid to receieve 1000 GLDT.

- When swapping NFT -> GLDT we refer to this as a 'Forward' swap
- When swapping GLDT -> NFT we refer to this as as 'Reverse' swap


## Preliminary technical information

#### NFTs
the GLD NFT's are separated into different origny_nft canisters based on their weight. this means all the 1g NFT's are held in one canister, all the 10g NFT's are held in a different canister and so on. All the nft canisters have the same 
interface it's just the meta details of the NFT, i.e, it's weight that change. You can reference [documentation](https://github.com/ORIGYN-SA/nft) called on the nft canister here.

## Folder structure - core code
- `backend/canisters/gldt_swap`
  - `/common` - this holds types used across all other folders
  - `/api` - holds the API interface types for the main swap canister
  - `/impl` - holds the business logic / core code for the main swap canister

## Folder structure - integration tests
- `/backend/integration_testing/src/gldt_swap_suite` - holds the suite of tests dedicated to the gldt swap backend canister


## Running integration tests locally

1 - Typically you'll first need to make sure you have PocketIC v9.0.0 or above in the correct location below. Please make sure to read the [instructions](https://github.com/dfinity/pocketic) provided by PocketIC
> /backend/integration_testing/pocket-ic

2 - Now you have PocketIC downloaded in the correct location and correct execution permissions. you may need to alter how many open files your computer ( Mac ) may open. so open your terminal of choice and enter the following commands.
```bash
   ulimit -n 202400
   ulimit -f 2024000
```

3 - Now you have run the integration tests. To do this, there is a handy script that builds the project and runs the integration tests for you. 
```bash
./scripts/manual/run-integration-tests.sh
```

## Deployment to staging

1) ensure you have the GLD staging principal

2) build and deploy in one single command
```bash
./scripts/_local/manual/deploy-gldt-swap.sh
```




## Architecture & Features

#### Main swap canister

This canister represents the main canister that a frontend will call in order to actually swap GLD NFT's with GLDT and vise versa. Jump to the sections to see how to perform core functionality as a frontend 
- [Swap GLD NFT -> GLDT](#swap-gld-nft---gldt)
- [Swap GLDT -> GLD NFT](#swap-gldt---gld-nft)

#### fee distribution

This feature is yet to be developed but reverse swaps pay an extra 1GLDT to reverse swap and after transfer fees we send the remaining balance of this fee to a dedicated sub account. Eventually it will distribute the rewards but for now we're simply going to collect the fees in the dedicated sub account.


#### Service status

There is another cron that checks periodically if a swap will be successful based on a different factors. for example, if the OGY balance is too low then we set the service to Down so that we may prevent swaps from failing and so that the frontend can display that the service is not currently up to accept new swaps.


#### Stale swaps

Only if a swap's status is wrapped in `Failed` enum varient will it end up in history. There are some places where a swap fails but we purposefully chose to not wrap in Failed. One such case is during the forward swap when a bid attempted. When a bid fails we set the status to `BidFailed`. This status is then picked up by the cron job manage_stale_swaps where it checks for the sale has expired and attempts to refund both parties.