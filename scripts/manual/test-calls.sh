

dfx canister call --network staging icp_neuron manage_nns_neuron_validate '(record { neuron_id = 123;
command= variant{ Spawn = record {percentage_to_spawn=null; new_controller=null; nonce=null }} })'


dfx canister call --network staging icp_neuron stake_nns_neuron

dfx canister call --network staging icp_neuron manage_nns_neuron '(record {neuron_id = 17481076647658761488;
command = variant { Configure = record { operation = opt variant { AddHotKey = record { new_hot_key = opt principal "j2neh-vqaaa-aaaal-aduxq-cai" } }} }})'


dfx canister call --network ic nns_governance manage_neuron '(record { id = opt record {id = 17481076647658761488 }; command = variant { ClaimOrRefresh = record { by = opt variant {NeuronIdOrSubaccount = record {}}} }})'


dfx canister call --network ic rrkah-fqaaa-aaaaa-aaaaq-cai list_neurons '(record {neuron_ids=vec {}; include_neurons_readable_by_caller= true})'
# dfx canister call --network ic rrkah-fqaaa-aaaaa-aaaaq-cai get_full_neuron '(17481076647658761488)'


dfx canister call --network staging icp_neuron manage_reward_recipients '(record { list = vec {
  record { tag = "Team development"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "GLDGov voters"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "Liquidity funds"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "Good DAO"; reward_weight = 100; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } }
}})'
