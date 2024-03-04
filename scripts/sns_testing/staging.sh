# Instructions on how to deploy
# https://forum.dfinity.org/t/sns-testflight-error/27081/11
./sns_cli deploy-testflight --verbose --init-config-file=$(pwd)/sns/config/sns_init_staging.yaml --network ic



# List neurons
dfx canister --network staging call sns_governance list_neurons '(record {of_principal = opt principal "'"$(dfx identity get-principal)"'"; limit = 10; start_page_at= null})'

# List proposals
dfx canister --network staging call sns_governance list_proposals '(record {include_reward_status = vec {}; before_proposal = null; limit = 10; exclude_type=vec {}; include_status = vec {}})'


# ICP neuron testing

## register canister
### add sns root as controller
dfx canister --network staging update-settings --add-controller $(dfx canister --network staging id sns_root) icp_neuron
### register canister in SNS
./scripts/sns_testing/register-canister.sh

## register custom methods
./scripts/sns_testing/register-custom-method.sh

## call custom method
./scripts/sns_testing/call-custom-method.sh
