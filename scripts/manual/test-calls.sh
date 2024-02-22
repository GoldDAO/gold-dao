

dfx canister call --network staging icp_neuron manage_nns_neuron_validate '(record { neuron_id = 123;
command= variant{ Spawn = record {percentage_to_spawn=null; new_controller=null; nonce=null }} })'
