use crate::{
    state::{mutate_state, read_state, FeeAccount},
    utils::trace,
};
use candid::{Nat, Principal};
use canister_time::{run_now_then_interval, MINUTE_IN_MS};
use gldt_swap_common::{gldt::OGYTokenSpec, nft::NftCanisterConf};
use icrc_ledger_canister_c2c_client::{icrc1_balance_of, icrc1_transfer};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use origyn_nft_reference::origyn_nft_reference_canister::{
    Account as OrigynAccount, FeeDepositRequest, ManageSaleRequest, ManageSaleResponse,
    ManageSaleResult, SaleInfoRequest, SaleInfoResponse, SaleInfoResult,
};
use origyn_nft_reference_c2c_client::{sale_info_nft_origyn, sale_nft_origyn};
use std::{borrow::Borrow, time::Duration};
use tracing::{debug, info};
use types::Milliseconds;
use utils::env::Environment;

const MANAGE_FEE_ACCOUNTS: Milliseconds = MINUTE_IN_MS;

pub fn start_job() {
    run_now_then_interval(
        Duration::from_millis(MANAGE_FEE_ACCOUNTS),
        spawn_transfer_job,
    );
}

pub fn spawn_transfer_job() {
    ic_cdk::spawn(manage_fee_accounts())
}

async fn manage_fee_accounts() {
    // sale_info_nft_origyn(#fee_deposit_info

    let ogy_ledger = read_state(|s| s.data.ogy_ledger_id);
    let nft_canisters = read_state(|s| s.data.gldnft_canisters.clone());
    let this_canister_id = read_state(|s| s.env.canister_id());

    let fee_account_minimum_threshold = read_state(|s| s.get_required_ogy_for_1000_swaps().clone());
    let min_total_threshold = read_state(|s| s.get_required_ogy_for_canister()); // we add 1 because the reverse swap needs OGY on the canister itself

    // if one of the fee accounts is not yet set, we need to generate them.
    if nft_canisters
        .iter()
        .any(|(_, _, fee_account)| fee_account.is_none())
    {
        let deposit_accounts = get_fee_deposit_accounts().await;
        mutate_state(|s| {
            s.data.gldnft_canisters = deposit_accounts;
        });
    }

    // get the ogy balance of this canister
    match icrc1_balance_of(
        ogy_ledger,
        Account {
            owner: this_canister_id,
            subaccount: None,
        },
    )
    .await
    {
        Ok(b) => {
            if b < min_total_threshold {
                info!(
                    "MANAGE FEE ACCOUNTS : total OGY balance is {b} for swap canister but needs to be above {min_total_threshold}"
                );
                return;
            }
        }
        Err(e) => {
            info!(
                "ERROR : Cant top up canisters because the canister OGY balance is 0 - error : {e:?}"
            );
        }
    }

    // check each fee account balance and if its under the threshold then we topup ogy
    let nft_canisters = read_state(|s| s.data.gldnft_canisters.clone());

    // we always verify the current ogy balance because if the nft canister upgrades or reboots it will need to have the fee account verified again.
    for (nft_canister_id, _, _) in &nft_canisters {
        verify_current_ogy_balance(nft_canister_id).await;
    }

    for (nft_canister_id, _, account) in nft_canisters {
        // get balance
        match account {
            Some(acc) => {
                let res = icrc1_balance_of(ogy_ledger, acc).await;
                match res {
                    Ok(fee_account_balance) => {
                        if fee_account_balance < fee_account_minimum_threshold.clone() {
                            match icrc1_transfer(
                                ogy_ledger,
                                &(TransferArg {
                                    from_subaccount: None,
                                    to: acc,
                                    fee: None,
                                    created_at_time: None,
                                    memo: None,
                                    amount: Nat::from(fee_account_minimum_threshold.clone()),
                                }),
                            )
                            .await
                            {
                                Ok(transfer_res) => match transfer_res {
                                    Ok(_) => {
                                        info!("MANAGE OGY FEE ACCOUNTS :: topped up fee account");
                                        verify_current_ogy_balance(&nft_canister_id).await;
                                    }
                                    Err(e) => {
                                        trace(&format!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}"));
                                        debug!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}");
                                    }
                                },
                                Err(e) => {
                                    trace(&format!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}"));
                                    debug!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}");
                                }
                            }
                        } else {
                            // balance is more than threshold. nothing to do
                        }
                    }
                    Err(e) => {
                        trace(&format!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}"));
                        debug!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}");
                    }
                }
            }
            None => {
                trace(&format!(
                    "MANAGE OGY FEE ACCOUNTS 1 :: no fee account to transfer to"
                ));
                debug!("MANAGE OGY FEE ACCOUNTS 1 :: no fee account to transfer to");
            }
        }

        // does this canister balance have enough to transfer
    }
}

async fn get_fee_deposit_accounts() -> Vec<(Principal, NftCanisterConf, Option<FeeAccount>)> {
    let nft_canisters = read_state(|s| s.data.gldnft_canisters.clone());
    let this_canister_id = read_state(|s| s.env.canister_id());

    let mut accounts_to_set: Vec<(Principal, NftCanisterConf, Option<FeeAccount>)> = vec![];
    for (nft_canister, config, _) in nft_canisters {
        match sale_info_nft_origyn(
            nft_canister,
            SaleInfoRequest::FeeDepositInfo(Some(OrigynAccount::Account {
                owner: this_canister_id,
                sub_account: None,
            })),
        )
        .await
        {
            Ok(res) => match res {
                SaleInfoResult::Ok(ok_res) => match ok_res {
                    SaleInfoResponse::FeeDepositInfo(fee_deposit_info) => {
                        let g = config.grams;
                        debug!(
                            "for {g}g canister :: get_fee_deposit_accounts :: {fee_deposit_info:?}"
                        );

                        let account = Account {
                            owner: fee_deposit_info.account.principal,
                            subaccount: Some(
                                fee_deposit_info
                                    .account
                                    .sub_account
                                    .as_slice()
                                    .try_into()
                                    .unwrap(),
                            ),
                        };
                        accounts_to_set.push((nft_canister, config, Some(account)));
                    }
                    _ => {}
                },
                SaleInfoResult::Err(error) => {
                    debug!("{error:?}");
                }
            },
            Err(error) => {
                debug!("{error:?}");
            }
        }
    }
    info!("MANAGE OGY SWAP FEES :: fee accounts :: {accounts_to_set:?}");
    accounts_to_set
}

// if the nft canister reboots, we always need to verify the current deposit otherwise the nft canister will assume there is no OGY deposited.
async fn verify_current_ogy_balance(nft_canister_id: &Principal) {
    let ogy_ledger = read_state(|s| s.data.ogy_ledger_id);
    let this_canister_id = read_state(|s| s.env.canister_id());
    debug!("MANAGE OGY FEE ACCOUNT :: attempting to verify ogy fee for  {nft_canister_id:?}");
    match sale_nft_origyn(
        nft_canister_id.clone(),
        ManageSaleRequest::FeeDeposit(FeeDepositRequest {
            token: OGYTokenSpec::new(ogy_ledger).get_token_spec(),
            account: OrigynAccount::Account {
                owner: this_canister_id,
                sub_account: None,
            },
        }),
    )
    .await
    {
        Ok(ok_verify_fee) => match ok_verify_fee {
            ManageSaleResult::Ok(ok_fee_res) => match ok_fee_res.borrow() {
                ManageSaleResponse::FeeDeposit(deposit) => {
                    let balance = deposit.balance.clone();
                    debug!(
                                "MANAGE OGY FEE ACCOUNT :: fee account verified :: {nft_canister_id:?} has balance : {balance:?}"
                            );
                }
                _ => {
                    debug!("MANAGE OGY FEE ACCOUNT :: something else happened {ok_fee_res:?}");
                }
            },
            ManageSaleResult::Err(e) => {
                trace(&format!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}"));
                debug!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}");
            }
        },
        Err(e) => {
            trace(&format!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}"));
            debug!("MANAGE OGY FEE ACCOUNTS ERROR :: {e:?}");
        }
    }
}
