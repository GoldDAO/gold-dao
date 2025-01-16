use crate::ApiFeeBucket;
use candid::CandidType;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use std::fmt;

pub struct DisplayAmount(pub u64);

impl fmt::Display for DisplayAmount {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        const E8S: u64 = 100_000_000;
        let int = self.0 / E8S;
        let frac = self.0 % E8S;

        if frac > 0 {
            let frac_width: usize = {
                // Count decimal digits in the fraction part.
                let mut d = 0;
                let mut x = frac;
                while x > 0 {
                    d += 1;
                    x /= 10;
                }
                d
            };
            debug_assert!(frac_width <= 8);
            let frac_prefix: u64 = {
                // The fraction part without trailing zeros.
                let mut f = frac;
                while f % 10 == 0 {
                    f /= 10
                }
                f
            };

            write!(fmt, "{}.", int)?;
            for _ in 0..(8 - frac_width) {
                write!(fmt, "0")?;
            }
            write!(fmt, "{}", frac_prefix)
        } else {
            write!(fmt, "{}.0", int)
        }
    }
}

#[derive(CandidType, Debug, PartialEq, Eq, Clone)]
pub struct CandidEvent {
    /// The canister time at which the minter generated this event.
    pub timestamp: u64,
    /// The event type.
    pub payload: CandidEventType,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq)]
pub enum CandidEventType {
    Init {
        usdg_ledger_id: Principal,
        gldt_ledger_id: Principal,
        gold_dao_governance_id: Principal,
        xrc_id: Principal,
    },

    Upgrade {
        new_medium_fee_percent: Option<u64>,
    },

    OpenVault {
        owner: Account,
        margin_amount: u64,
        borrowed_amount: u64,
        fee_bucket: ApiFeeBucket,
        block_index: u64,
    },

    Borrow {
        vault_id: u64,
        borrowed_amount: u64,
        block_index: u64,
    },

    AddMargin {
        vault_id: u64,
        margin_added: u64,
        block_index: u64,
    },

    Repay {
        vault_id: u64,
        debt: u64,
        block_index: u64,
    },

    Close {
        vault_id: u64,
        block_index: Option<u64>,
    },
    TransferExecuted {
        transfer_id: u64,
        block_index: u64,
    },

    DepositLiquidity {
        caller: Account,
        amount: u64,
        block_index: u64,
    },

    WithdrawLiquidity {
        caller: Account,
        amount: u64,
        block_index: u64,
    },

    ClaimReturns {
        caller: Account,
        amount: u64,
        block_index: u64,
    },

    Redeem {
        owner: Account,
        current_rate: u64,
        amount: u64,
        block_index: u64,
    },

    ChargeFee,

    Liquidate {
        vault_id: u64,
    },

    Redistribute {
        vault_id: u64,
    },

    UpdateVault {
        vault_id: u64,
        fee_bucket: Option<ApiFeeBucket>,
        new_owner: Option<Account>,
    },
}
