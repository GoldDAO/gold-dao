use candid::{ CandidType, Deserialize, Nat, Principal };

use icrc_ledger_types::icrc1::{ account::{ Account, Subaccount }, transfer::{ BlockIndex, Memo } };
use serde::Serialize;
use std::collections::{ BTreeMap, btree_map };

use gldt_libs::types::{ NftId, GldtNumTokens, NftWeight };
use crate::records::{ GldtRecord, RecordType, RecordStatusInfo, RecordStatus };

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct Registry {
    registry: BTreeMap<(Principal, NftId), GldtRegistryEntry>,
}

/// Entry into the GLDT registry that keeps track of the NFTs that
/// have been swapped for GLDT.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct GldtRegistryEntry {
    /// The lifecycle of an NFT starts with the issuance of GLDT
    gldt_issue: SwapInfo,
    /// The lifecycle of an NFT ends with the burning of GLDT
    gldt_redeem: Option<SwapInfo>,
    /// Optional reference to a previous minting/burning pair for this
    /// NFT as a historial record.
    older_record: Option<Box<GldtRegistryEntry>>,
}

impl GldtRegistryEntry {
    pub fn new(swap_info: SwapInfo) -> Self {
        Self {
            gldt_issue: SwapInfo::new(
                swap_info.nft_sale_id,
                swap_info.escrow_subaccount,
                swap_info.receiving_account,
                swap_info.swap_request_timestamp,
                swap_info.num_tokens
            ),
            gldt_redeem: None,
            older_record: None,
        }
    }

    pub fn get_status_of_swap(&self) -> SwappingStates {
        if self.is_minted() {
            if self.is_swapped() {
                if self.is_burned() { SwappingStates::Burned } else { SwappingStates::Swapped }
            } else {
                SwappingStates::Minted
            }
        } else {
            SwappingStates::Initialised
        }
    }

    pub fn is_minted(&self) -> bool {
        self.gldt_issue.ledger_entry.is_some()
    }

    pub fn is_swapped(&self) -> bool {
        self.gldt_issue.swapped.is_some()
    }

    pub fn is_burned(&self) -> bool {
        // This logics needs to become more sophisticated once actual burning is implemented.
        self.gldt_redeem.is_some()
    }

    pub fn get_issue_info(&self) -> &SwapInfo {
        &self.gldt_issue
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum UpdateType {
    Init,
    Mint,
    Swap,
    Failed,
    Burn,
}

///
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct SwapInfo {
    /// The sale id of the NFT listing in the GLD NFT canister
    nft_sale_id: String,
    /// The escrow account where the GLDT tokens are sent to for the trade.
    escrow_subaccount: Subaccount,
    /// The receiving account for the swap
    receiving_account: Account,
    /// The timestamp when this entry was made
    swap_request_timestamp: u64,
    /// The number of tokens swapped.
    num_tokens: GldtNumTokens,
    /// The requested memo
    requested_memo: Memo,

    /// Filled when tokens are successfully minted or burned.
    ledger_entry: Option<GldtLedgerEntry>,
    /// Filled when NFT has been successfully swapped.
    swapped: Option<GldtSwapped>,
    /// Filled in case of errors during the minting and swapping process
    failed: Option<GldtError>,
}

impl SwapInfo {
    pub fn new(
        nft_sale_id: String,
        escrow_subaccount: Subaccount,
        receiving_account: Account,
        swap_request_timestamp: u64,
        num_tokens: GldtNumTokens
    ) -> Self {
        Self {
            nft_sale_id,
            escrow_subaccount,
            receiving_account,
            swap_request_timestamp,
            num_tokens,
            requested_memo: Memo::default(),
            ledger_entry: None,
            swapped: None,
            failed: None,
        }
    }
    pub fn is_failed(&self) -> bool {
        self.failed.is_some()
    }

    pub fn set_failed(&mut self, error: GldtError) {
        self.failed = Some(error);
    }
    pub fn set_ledger_entry(&mut self, ledger_entry: GldtLedgerEntry) {
        self.ledger_entry = Some(ledger_entry);
    }

    pub fn set_swapped(&mut self, swapped: GldtSwapped) {
        self.swapped = Some(swapped);
    }

    pub fn get_num_tokens(&self) -> GldtNumTokens {
        self.num_tokens.clone()
    }
    pub fn get_nft_sale_id(&self) -> String {
        self.nft_sale_id.clone()
    }
    pub fn get_escrow_subaccount(&self) -> Subaccount {
        self.escrow_subaccount
    }
    pub fn get_swap_request_timestamp(&self) -> u64 {
        self.swap_request_timestamp
    }
    pub fn get_receiving_account(&self) -> Account {
        self.receiving_account
    }
    pub fn get_requested_memo(&self) -> Memo {
        self.requested_memo.clone()
    }
    pub fn get_ledger_entry(&self) -> Option<GldtLedgerEntry> {
        self.ledger_entry.clone()
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum GldtLedgerEntry {
    Minted(GldtLedgerInfo),
    Burned(GldtLedgerInfo),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Error {
    error_code: Nat,
    error_message: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum GldtError {
    /// The minting of GLDT failed.
    MintingError(Option<Error>),
    /// The swapping of NFT for GLDT failed.
    SwappingError(Option<Error>),
    /// Extensible error types
    Other(Option<Error>),
}

impl Default for GldtError {
    fn default() -> Self {
        Self::Other(
            Some(Error { error_code: Nat::from(0), error_message: "unknown error".to_string() })
        )
    }
}

/// Record of information about an NFT for which GLDT has been minted.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Default)]
pub struct GldtLedgerInfo {
    /// Block height when this entry was made. Must be non-zero and
    /// point to a block with a minting or burning transaction with the right
    /// number of tokens and subaccount.
    block_height: BlockIndex,

    /// The number of tokens that were part of this transaction.
    /// It should alway be 1g : 100 GLDT
    num_tokens: GldtNumTokens,
}

impl GldtLedgerInfo {
    pub fn new(block_height: BlockIndex, num_tokens: GldtNumTokens) -> Self {
        Self {
            block_height,
            num_tokens,
        }
    }
    pub fn get_block_height(&self) -> BlockIndex {
        self.block_height.clone()
    }
}

/// Record of information about an NFT that has been successfully swapped for GLDT.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct GldtSwapped {
    /// Sale ID of the successful sale
    sale_id: String,
    /// Index of the bid
    index: Nat,
}

impl GldtSwapped {
    pub fn new(sale_id: String, index: Nat) -> Self {
        Self {
            sale_id,
            index,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub enum SwappingStates {
    Initialised,
    Minted,
    Swapped,
    Burned,
}

impl Registry {
    pub fn get_entry(&self, key: &(Principal, NftId)) -> Option<&GldtRegistryEntry> {
        self.registry.get(key)
    }
    pub fn init(&mut self, key: &(Principal, NftId), entry: SwapInfo) -> Result<(), String> {
        Self::explicit_sequence_check(&self, key, UpdateType::Init)?;
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(v) => {
                v.insert(GldtRegistryEntry::new(entry));
                Ok(())
            }
            btree_map::Entry::Occupied(mut o) => {
                // If there is already an entry when initialising, it may be due to
                // a failed previous swap or because the tokens have been burned.
                // If not, then there may be an attempt to double mint and the
                // procedure is cancelled.
                if o.get().is_burned() || o.get().gldt_issue.is_failed() {
                    o.insert(GldtRegistryEntry {
                        gldt_issue: SwapInfo::new(
                            entry.nft_sale_id,
                            entry.escrow_subaccount,
                            entry.receiving_account,
                            entry.swap_request_timestamp,
                            entry.num_tokens
                        ),
                        gldt_redeem: None,
                        older_record: Some(Box::new(o.get().clone())),
                    });
                    Ok(())
                } else {
                    Err(
                        format!(
                            "There is already an active entry for NFT: {}. Canceling new minting of tokens.",
                            key.1
                        )
                    )
                }
            }
        }
    }

    pub fn update_minted(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::sanity_check_inputs(self, key, &entry)?;
        Self::explicit_sequence_check(&self, key, UpdateType::Mint)?;
        // check that the input to the function is as expected
        // we are expecting the the ledger_entry is of type Minted
        match entry.ledger_entry.clone() {
            None => {
                return Err(
                    format!(
                        "There is no ledger entry for NFT: {}. Cannot update minting of tokens.",
                        key.1
                    )
                );
            }
            Some(ledger_entry) =>
                match ledger_entry {
                    GldtLedgerEntry::Minted(_) => (), // this is the happy path
                    GldtLedgerEntry::Burned(_) => {
                        return Err(
                            format!(
                                "Burning not implemented yet. There is no valid ledger entry for NFT: {}. Cannot update minting of tokens.",
                                key.1
                            )
                        );
                    }
                }
        }
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(_) => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update minting of tokens.",
                        key.1
                    )
                )
            }
            btree_map::Entry::Occupied(mut o) => {
                match &o.get().gldt_issue.ledger_entry {
                    Some(_) => {
                        Err(
                            format!(
                                "There is already a ledger entry for NFT: {}. Cannot update minting of tokens.",
                                key.1
                            )
                        )
                    }
                    None => {
                        // This is the happy path when tokens are minted
                        o.get_mut().gldt_issue.ledger_entry = entry.ledger_entry;
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn update_swapped(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::sanity_check_inputs(self, key, &entry)?;
        Self::explicit_sequence_check(&self, key, UpdateType::Swap)?;
        // check that the input to the function is as expected
        // we are expecting the key `swapped` to be Some
        match entry.swapped.clone() {
            None => {
                return Err(
                    format!(
                        "There is no swap info for NFT: {}. Cannot update swapping of tokens.",
                        key.1
                    )
                );
            }
            Some(_) => (),
        }
        match self.registry.entry(key.clone()) {
            btree_map::Entry::Vacant(_) => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update swapping of tokens.",
                        key.1
                    )
                )
            }
            btree_map::Entry::Occupied(mut o) => {
                match &o.get().gldt_issue.swapped {
                    Some(_) => {
                        Err(
                            format!(
                                "There is already a swap info for NFT: {}. Cannot update swapping of tokens.",
                                key.1
                            )
                        )
                    }
                    None => {
                        // This is the happy path when tokens are swapped
                        o.get_mut().gldt_issue.swapped = entry.swapped;
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn update_failed(
        &mut self,
        key: &(Principal, NftId),
        entry: SwapInfo
    ) -> Result<(), String> {
        Self::explicit_sequence_check(&self, key, UpdateType::Failed)?;
        match self.registry.get_mut(key) {
            Some(r) => {
                r.gldt_issue.set_failed(entry.failed.unwrap_or_default());
            }
            None => {
                return Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot update failed minting of tokens.",
                        key.1
                    )
                );
            }
        }
        Ok(())
    }

    fn explicit_sequence_check(
        &self,
        key: &(Principal, NftId),
        update_type: UpdateType
    ) -> Result<(), String> {
        // Proper sequence of entries needs to be ensured.
        // Possible sequences currently are
        // 1. Init -> Mint -> Swap
        // 2. Any -> Failed

        match self.registry.get(key) {
            None => {
                if let UpdateType::Init = update_type {
                    // Only init is allowed when there is no entry yet.
                    return Ok(());
                } else {
                    Err(
                        format!(
                            "There is no active entry for NFT: {}. Cannot perform sequence check.",
                            key.1
                        )
                    )
                }
            }
            Some(r) => {
                let previous_status = r.get_status_of_swap();
                match update_type {
                    UpdateType::Init => {
                        // Init is further validated in the init function
                        return Ok(());
                    }
                    UpdateType::Mint => {
                        // Mint can only come after Init
                        if previous_status == SwappingStates::Initialised {
                            return Ok(());
                        }
                    }
                    UpdateType::Swap => {
                        // Swap can only come after Mint
                        if previous_status == SwappingStates::Minted {
                            return Ok(());
                        }
                    }
                    UpdateType::Failed => {
                        // Failed needs no validation as it can come from any state
                        return Ok(());
                    }
                    UpdateType::Burn => {
                        return Err("Burning not implemented yet.".to_string());
                    }
                }
                Err(
                    format!(
                        "Invalid sequence of updates for NFT: {}. Previous status was {previous_status:?}, new status was supposed to be {update_type:?}.",
                        key.1
                    )
                )
            }
        }
    }

    fn sanity_check_inputs(
        &self,
        key: &(Principal, NftId),
        entry: &SwapInfo
    ) -> Result<(), String> {
        match self.registry.get(key) {
            None => {
                Err(
                    format!(
                        "There is no active entry for NFT: {}. Cannot perform sanity check.",
                        key.1
                    )
                )
            }
            Some(r) => {
                let gldt_issue = r.gldt_issue.clone();
                let mut problems = Vec::new();

                if gldt_issue.nft_sale_id != entry.nft_sale_id {
                    problems.push(
                        format!(
                            "NFT sale ID - recorded: {:?}, expected: {:?}",
                            gldt_issue.nft_sale_id,
                            entry.nft_sale_id
                        )
                    );
                }
                if gldt_issue.receiving_account != entry.receiving_account {
                    problems.push(
                        format!(
                            "Receiving account - recorded: {:?}, expected: {:?}",
                            gldt_issue.receiving_account,
                            entry.receiving_account
                        )
                    );
                }
                if gldt_issue.num_tokens != entry.num_tokens {
                    problems.push(
                        format!(
                            "Number of tokens - recorded: {:?}, expected: {:?}",
                            gldt_issue.num_tokens,
                            entry.num_tokens
                        )
                    );
                }
                if gldt_issue.requested_memo != entry.requested_memo {
                    problems.push(
                        format!(
                            "memo - recorded: {:?}, expected: {:?}",
                            gldt_issue.requested_memo,
                            entry.requested_memo
                        )
                    );
                }
                if gldt_issue.escrow_subaccount != entry.escrow_subaccount {
                    problems.push(
                        format!(
                            "escrow subaccount - recorded: {:?}, expected: {:?}",
                            gldt_issue.escrow_subaccount,
                            entry.escrow_subaccount
                        )
                    );
                }
                if gldt_issue.swap_request_timestamp != entry.swap_request_timestamp {
                    problems.push(
                        format!(
                            "timestamp - recorded: {}, expected: {}",
                            gldt_issue.swap_request_timestamp,
                            entry.swap_request_timestamp
                        )
                    );
                }
                if !problems.is_empty() {
                    // If there are problems, it is most likely the
                    // case that the response we are handing is
                    // spurious, i.e., not corresponding to the
                    // request made.
                    let msg = format!(
                        "ERROR: ignoring canister response because request state does not match response state: problems {}, record {:?}",
                        problems.join("; "),
                        entry
                    );
                    return Err(msg);
                }
                Ok(())
            }
        }
    }

    pub fn get_ongoing_swaps_by_user(&self, account: Account) -> Vec<GldtRecord> {
        let mut result = Vec::new();
        for ((gld_nft_canister_id, nft_id), entry) in &self.registry {
            if entry.gldt_issue.receiving_account == account {
                if entry.is_swapped() || entry.gldt_issue.is_failed() {
                    continue;
                }
                result.push(
                    GldtRecord::new(
                        RecordType::Mint,
                        entry.gldt_issue.get_swap_request_timestamp(),
                        entry.gldt_issue.get_receiving_account(),
                        *gld_nft_canister_id,
                        (*nft_id).clone(),
                        entry.gldt_issue.get_escrow_subaccount(),
                        entry.gldt_issue.get_nft_sale_id(),
                        0 as NftWeight,
                        entry.gldt_issue.get_num_tokens(),
                        Nat::from(0),
                        RecordStatusInfo {
                            status: RecordStatus::Ongoing,
                            message: None,
                        }
                    )
                );
            }
        }
        result
    }
}
