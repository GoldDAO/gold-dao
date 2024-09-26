# TODOS
## TODO - int tests for service status
## TODO - clean up 
## TODO - integrate into pipeline - basic integration done ebut maybe we need more

# Issues
## bitFinfinity wallet only seems to approve one transfer ( need to retest this ). the wallet integration needs to use batch commands but i dont think we have low level access
## the nft canister may send a rouge sale notification - we have logs in place to catch more information. 
## the reverse swap burns more than the nft amount. No idea where its coming from ( see reverse_swap_basic integration test and the final assertions )

## testing 
// transfer ogy to the swap canister
dfx canister --network ic call j5naj-nqaaa-aaaal-ajc7q-cai icrc1_transfer "(record { to = record { owner = principal \"m45be-jaaaa-aaaak-qcgnq-cai\"; subaccount = null; }; amount = 100_000_000_000; })"