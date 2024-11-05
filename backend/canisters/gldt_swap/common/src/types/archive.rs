use core::fmt;
use std::fmt::{Display, Formatter};

use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ArchiveCanister {
    pub canister_id: Principal,
    pub start_index: Nat,
    pub end_index: Option<Nat>,
    pub active: bool,
}

impl Display for ArchiveCanister {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{ canister_id: {}, start_index: {}, end_index: {} }}",
            self.canister_id,
            self.start_index.to_string(),
            self.end_index
                .as_ref()
                .map_or("None".to_string(), |v| v.to_string())
        )
    }
}

pub fn format_archive_canisters(canisters: Vec<ArchiveCanister>) -> String {
    let canisters_str: Vec<String> = canisters
        .iter()
        .map(|canister| canister.to_string())
        .collect();

    format!("[\n   {}\n]", canisters_str.join(",\n   "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Nat, Principal};

    #[test]
    fn test_archive_record_formatter() {
        let archives = vec![
            ArchiveCanister {
                start_index: Nat::from(0u64),
                canister_id: Principal::anonymous(),
                end_index: Some(Nat::from(99u64)),
                active: true,
            },
            ArchiveCanister {
                start_index: Nat::from(100u64),
                canister_id: Principal::anonymous(),
                end_index: None,
                active: true,
            },
        ];

        let expected = format_archive_canisters(archives);
        assert_eq!(
            expected,
            String::from(
                "[
   { canister_id: 2vxsx-fae, start_index: 0, end_index: 99 },
   { canister_id: 2vxsx-fae, start_index: 100, end_index: None }
]"
            )
        )
    }
}
