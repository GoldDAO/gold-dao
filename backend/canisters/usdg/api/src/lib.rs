use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;
use std::fmt;

pub mod lifecycle;
pub mod queries;
pub mod updates;

pub const USDG_TRANSFER_FEE: u64 = 1_000_000;

#[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
pub enum ApiFeeBucket {
    Low,
    Medium,
    High,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum VaultError {
    TransferFromError(TransferFromError),
    AnonymousCaller,
    AmountTooLow { minimum_amount: u64 },
    NoRecentGoldPrice,
    BorrowedAmountTooBig { maximum_borrowable_amount: u64 }
}

#[derive(CandidType, Deserialize, Debug, Eq, PartialEq)]
pub struct ApiVault {
    pub vault_id: u64,
    pub owner: Account,
    pub borrowed_amount: u64,
    pub margin_amount: u64,
    pub fee_bucket: ApiFeeBucket,
}

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