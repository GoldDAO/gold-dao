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
interface it's just the meta details of the NFT, i.e, it's weight that change. You can reference [documentation](https://github.com/ORIGYN-SA/origyn_nft/tree/develop/docs) called on the nft canister here. Please be aware some of the documention is not fully up to date.

Particular functions of note to read about are

- `sale_nft_origyn` - this function is capable of performing various types of actions including bidding, verifying escrow deposits.
- `market_transfer_nft_origyn` - this function creates a sale on the NFT so that it may accept bids. 


## Folder structure - core code
- `backend/canisters/gldt_swap`
  - `/common` - this holds types used across all other folders
  - `/api` - holds the API interface types for the main swap canister
  - `/api_archive` - holds the API interface types for the archive canister
  - `/impl` - holds the business logic / core code for the main swap canister
  - `/archive` - holds the business logic / core code for the archive canister

## Folder structure - integration tests
- `/backend/integration_testing/src/gldt_swap_suite` - holds the suite of tests dedicated to the gldt swap backend canister


## Running integration tests locally

1 - Typically you'll first need to make sure you have PocketIC v4.0.0 or above in the correct location below. Please make sure to read the [instructions](https://github.com/dfinity/pocketic) provided by PocketIC
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

#### Archive canister

Archive canisters are spawned and maintained by the main swap canister. New archive canisters are created when the current used archive canister's stable memory reaches the threshold limit defined in the main swap canister. The main swap canister will also maintain the archive canisters in the following ways:
- ensure each archive canister has enough cycles
- upgrade archive canisters when the main swap canister is upgraded


#### OGY Fee account topup

When performing either a Forward or Reverse swap, the swap canister is setup to pay for the OGY fees associated with the sale. each NFT canister automically creates a dedicated fee account associated with the swap canister. Without a positive balance of OGY in the fee account's its impossible for forward or reverse swaps to complete successfully. We created a cron job to monitor the balance of OGY in each fee account and top them up when they are below a certain balance. 

#### fee distribution

This feature is yet to be developed but reverse swaps pay an extra 1GLDT to reverse swap and after transfer fees we send the remaining balance of this fee to a dedicated sub account. Eventually it will distribute the rewards but for now we're simply going to collect the fees in the dedicated sub account.


#### Service status

There is another cron that checks periodically if a swap will be successful based on a different factors. for example, if the OGY balance is too low then we set the service to Down so that we may prevent swaps from failing and so that the frontend can display that the service is not currently up to accept new swaps.


#### Stale swaps

Only if a swap's status is wrapped in `Failed` enum varient will it end up in history. There are some places where a swap fails but we purposefully chose to not wrap in Failed. One such case is during the forward swap when a bid attempted. When a bid fails we set the status to `BidFailed`. This status is then picked up by the cron job manage_stale_swaps where it checks for the sale has expired and attempts to refund both parties.  


### Forward Swap ( GLD NFT -> GLDT ) 

![image](/backend/canisters/gldt_swap/docs/forward-swap-flow.png)

> 1 Intent to swap.
--------
An intent to perform a forward swap is registered on the swap canister via the frontend calling `swap_nft_for_tokens`. This primes the canister for a swap by setting up an active swap record in the canister with relevent information.

> 2 oepning the sale.
--------
Only the owner may open a sale on the nft canister by calling market_transfer_request_orgyn. As such, if the previous step was successful then the frontend opens the sale on the user's behalf

> 3 notification
when the sale has been opened on the nft canister it will call a function on our swap canister called `notify_sale_nft_origyn`. This is where the rest of the swap process happens according to the diagram referenced in this section. At this point it's better to read the code of the following functions to understand what is happening

- forward_swap_validate_notification(&swap_id, &args);
- forward_swap_perform_mint_to_escrow(&swap_id).await;
- forward_swap_perform_bid_on_nft(&swap_id, args).await;
- forward_swap_perform_burn_fees(&swap_id).await;


### Reverse Swap ( GLDT -> GLD NFT )

![image](/backend/canisters/gldt_swap/docs/reverse-swap-flow.png)

> 1 swap_tokens_for_nft
This is much simpler than the forward swap because at this point the swap canister owns an NFT so it can open a sale on the nft canister by itself. to understand the core steps involved read the code of swap_tokens_for_nft and the code of the steps involved 

- transfer_to_escrow(&swap_id).await;
- transfer_nft(swap_id).await;
- burn_gldt(swap_id).await;
- transfer_fees(swap_id).await;
- refund(swap_id).await;