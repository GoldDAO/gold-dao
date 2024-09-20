use crate::client::gldt_swap::{ get_active_swap_ids_by_user, get_swap, insert_fake_swap };
use crate::gldt_swap_suite::{ init, CanisterIds, PrincipalIds, TestEnv };
use crate::utils::tick_n_blocks;

use canister_time::timestamp_millis;
use gldt_swap_common::gldt::GldtNumTokens;
use gldt_swap_common::nft::NftID;
use gldt_swap_common::swap::{ SwapDetailForward, SwapInfo, SwapStatusForward };
use icrc_ledger_types::icrc1::account::Account;
use candid::{ Nat, Principal };

#[cfg(test)]
mod tests {
    use gldt_swap_common::swap::SwapIndex;

    use super::*;
    #[test]
    pub fn get_swap_returns_correctly() {
        let mut env = init::init();
        let TestEnv {
            ref mut pic,
            canister_ids: CanisterIds { origyn_nft, gldt_swap, .. },
            principal_ids: PrincipalIds { nft_owner, controller, .. },
        } = env;
        tick_n_blocks(pic, 2); // need to wait for cron job to finish creating the archive

        let nft_id = NftID(Nat::from(1u64));
        // 3. insert the fake swap ( just after nft transfer is successful )
        insert_fake_swap(
            pic,
            controller,
            gldt_swap,
            &SwapInfo::Forward(SwapDetailForward {
                index: SwapIndex::from(0u64),
                sale_id: "something".to_string(),
                nft_id: nft_id.clone(),
                nft_id_string: "1".to_string(),
                status: SwapStatusForward::BidRequest,
                created_at: timestamp_millis(),
                tokens_to_mint: GldtNumTokens::new(Nat::from(10_000_000_000u64)).unwrap(),
                escrow_sub_account: nft_id.into(),
                gldt_receiver: Account { owner: nft_owner, subaccount: None },
                nft_canister: origyn_nft,
            })
        ).unwrap();
        tick_n_blocks(pic, 5);

        let active_swap_ids = get_active_swap_ids_by_user(
            pic,
            nft_owner,
            gldt_swap,
            &Some(nft_owner)
        );
        assert_eq!(active_swap_ids.len(), 1);

        let swap = get_swap(pic, Principal::anonymous(), gldt_swap, &active_swap_ids[0]).unwrap();

        if let SwapInfo::Forward(detail) = swap.1 {
            assert_eq!(detail.status, SwapStatusForward::BidRequest);
        }
    }
}
