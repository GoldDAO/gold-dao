

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


# OGY ledger
dfx canister call --network staging icp_ledger icrc1_transfer '(
  record {
    to = record {
      owner = principal "2f5ll-gqaaa-aaaak-qcfuq-cai";
      subaccount = opt blob "\b0\f2\d5\2b\05\bd\be\44\69\dc\94\4c\3b\8c\9b\5e\0b\84\41\5b\3e\91\da\82\e7\6d\d8\86\59\6f\07\03";
    };
    fee = null;
    memo = null;
    from_subaccount = null;
    created_at_time = null;
    amount = 200_000_000 : nat;
  },
)'

2f5ll-gqaaa-aaaak-qcfuq-cai

aad428e12791bf8ef0d9dd50ab67edf8bb05b6d52e279dd7997ca2535265ddc4
b0f2d52b05bdbe4469dc944c3b8c9b5e0b84415b3e91da82e76dd886596f0703
b467c83a69f39a33b8f529bccf75b959afb536e2089a4e9740dfd45af2033faa

d4af110116df4cefc9d3b35c9d003f30e427fdfb4f734973567087dfcdf7a70b
