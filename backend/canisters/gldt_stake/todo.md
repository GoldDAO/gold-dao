# GLDT Stake

### Major features

-----------
 before 2nd major review
-----------
- 9. extra endpoints
-- get_early_unstake_fee_account()
-- get_early_unstake_fee_account_balance()
-- get_neuron_pool_account_balance()
- 10. integration testing 
---- All error states for claim rewards, unstake, unstake early

-----------
 later
-----------

- 1. token prices - daily cron job to update the price of each token ( used for APY )
- 2. APY endpoint - uses total staked, last weekly reward pool and token prices to make a cummulative APY for the whole pool 
- 3. archive - positions that have been fully dissolved need to be sent to the archive canister for storage