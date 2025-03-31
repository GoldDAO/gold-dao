use canister_client::generate_candid_c2c_call;
use wtn_protocol_api::{
    cancel_withdrawal, claim_airdrop, get_airdrop_allocation, get_events, get_info,
    get_pending_rewards, get_transfer_statuses, get_withdrawal_requests, get_wtn_proposal_id,
    icp_to_nicp, icrc_10_supported_standards, icrc_21_canister_call_consent_message, nicp_to_icp,
};

// Queries
generate_candid_c2c_call!(get_airdrop_allocation);
generate_candid_c2c_call!(get_events);
generate_candid_c2c_call!(get_info);
generate_candid_c2c_call!(get_pending_rewards);
generate_candid_c2c_call!(get_transfer_statuses);
generate_candid_c2c_call!(get_withdrawal_requests);
generate_candid_c2c_call!(get_wtn_proposal_id);
generate_candid_c2c_call!(icrc_10_supported_standards);

// Updates
generate_candid_c2c_call!(cancel_withdrawal);
generate_candid_c2c_call!(claim_airdrop);
generate_candid_c2c_call!(icp_to_nicp);
generate_candid_c2c_call!(icrc_21_canister_call_consent_message);
generate_candid_c2c_call!(nicp_to_icp);
