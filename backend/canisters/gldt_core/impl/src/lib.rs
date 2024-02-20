/*!
# GLDT and GLDT Swapp dApp canisters

GLDT is a digital token 100% backed by physical gold in the form
of NFTs in a ratio of 1 gram of gold NFTs equals 100 GLDT. The
NFTs have their ownership registered to this canister, which is
used to convert NFTs to GLDT and back.  The GLDT canister
purchases NFTs by minting tokens and sells NFTs against the
burning of tokens.

The code of this canister is generic in the sense that it is not
tied to any particular type of NFT, except for the notion of
'grams' which is tied to tokens in a ratio of one gram equals 100
tokens. Thus, in principle, the same code can be used for NFT of
any physical commodity measured in grams. The canister could be
generalized further by replacing grams by some generic quantity,
but doing so right now (2023) seems to have little benefit and only
hamper the readability of the code.

The GLDT canister collaborates with the canisters holding gold
NFTs as well as the GLDT ledger, which follows the ICRC1 standard
of the IC.

```text
User                   NFT                  GLDT            GLDT Ledger
 |     list NFT (1)     |                    |                   |
 +--------------------->|                    |                   |
 |                      |    notify (2)      |                   |
 |                      +------------------->|                   |
 |                      |                    | mint request (3)  |
 |                      |                    +------------------>|
 |                      |<---------------------------------------+
 |                      |                    |                   |
 |                      |<-------------------+                   |
 |                      |      accept (4)    |                   |
 |                      |                    |                   |
 |                      +---+                |                   |
 |                      |   | accept (4a)    |                   |
 |                      |<--+                |                   |
 |                      |                    |                   |
```

The lifecycle of one NFT is as follows.

* Swapping procedure from NFT => GLDT

    1. A user lists an NFT for sale (1) through the NFT canister.

    2. Upon successful listing, the NFT canister notifies the GLDT canister
about the listing (2) with the public method `notify_sale_nft_origyn`
which triggers the swapping sequence.

    3. The GLDT canister mints (3) GLDT to an escrow account on the NFT canister.
This is required for the sale to go through.

    4. The GLDT canister accepts the offer of the listed NFT (4).

    5. The offer is accepted (4a) on the NFT canister: the NFT now belongs
to GLDT canister and the minted tokens on the escrow account are
distributed to the `seller` (user).

* The view of the ownership of NFT from the NFT canister and from
the GLDT canister is periodically audited (to be implemented).

* The GLDT canister releases an NFT against proof that the
corresponding number of tokens have been burned (to be
implemented).

The GLDT ledger uses the account ID of the gldt cansiter (an
instance of this code) as its 'minting account'.  Computed as
`$(dfx ledger account-id --of-canister gldt_core)`. The GLDT canister
also needs to point to the ledger canister as given by `$(dfx
canister id gldt_ledger)`.

## Copyright
© 2023  [Bochsler Assets & Securities (BAS) SA], [Switzerland]

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

[Bochsler Assets & Securities (BAS) SA]: https://bas.tech
[Switzerland]: https://www.zefix.ch/fr/search/entity/list/firm/1579921
*/

#![allow(clippy::must_use_candidate, clippy::too_many_lines, clippy::too_many_arguments)]

use candid::{ CandidType, Deserialize, Nat, Principal };
use ic_cdk::{ api::{ self, call::notify }, storage };
use ic_cdk_macros::{ export_candid, init, query, update };
use icrc_ledger_types::icrc1::{
    account::{ Account, Subaccount },
    transfer::{ BlockIndex, TransferArg, TransferError },
};
use serde::Serialize;
use std::cell::RefCell;
use std::hash::Hash;
use serde_json::{ json, Value };

mod records;
mod registry;

use gldt_libs::types::{
    calculate_tokens_from_weight,
    GldtNumTokens,
    GldtTokenSpec,
    NftId,
    NftWeight,
};

use gldt_libs::gld_nft::{
    self,
    Account as OrigynAccount,
    AskFeature,
    BidRequest,
    BidResponse_txn_type,
    DepositWithdrawDescription,
    EscrowReceipt,
    ManageSaleRequest,
    ManageSaleResponse,
    ManageSaleResult,
    PricingConfigShared,
    SaleStatusShared,
    SaleStatusShared_sale_type,
    SubAccountInfo,
};
use gldt_libs::gldt_ledger;
use gldt_libs::error::{ Custom as CustomError, Type as ErrorType };
use gldt_libs::misc::log_message;

use records::{ GldtRecord, RecordStatus, RecordStatusInfo, RecordType, Records };
use registry::{
    GldtError,
    GldtLedgerEntry,
    GldtLedgerInfo,
    GldtRegistryEntry,
    GldtSwapped,
    Registry,
    SwapInfo,
    SwappingStates,
    UpdateType,
};

/// The configuration points to the canisters that this canister
/// collaborates with, viz., the GLDT ledger canister and the NFT
/// canisters.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    gld_nft_canister_ids: Vec<(Principal, NftCanisterConf)>,
    /// Canister ID of the fee compensation canister to cover the conversion fees.
    gldt_fee_compensation_canister_id: Principal,
}

impl Conf {
    pub fn new(
        gldt_ledger_canister_id: Principal,
        gld_nft_canister_ids: Vec<(Principal, NftCanisterConf)>,
        gldt_fee_compensation_canister_id: Principal
    ) -> Self {
        Self {
            gldt_ledger_canister_id,
            gld_nft_canister_ids,
            gldt_fee_compensation_canister_id,
        }
    }
}

/// Configuration information for a single NFT canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct NftCanisterConf {
    /// The size in grams of the physical NFTs managed by this
    /// canister.  Note that the max value of u16 in grams is over
    /// 65kg. The largest gold bars are 400oz (~11kg) and the largest
    /// silver bars are 1000oz (~31kg).
    grams: NftWeight,
}

impl NftCanisterConf {
    pub fn new(grams: NftWeight) -> Self {
        Self { grams }
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_ids: Vec::new(),
            gldt_fee_compensation_canister_id: Principal::anonymous(),
        }
    }
}

impl Conf {
    pub fn get_weight_by_collection_id(&self, collection_id: &Principal) -> Option<NftWeight> {
        self.gld_nft_canister_ids
            .iter()
            .find(|(x, _)| *x == *collection_id)
            .map(|(_, conf)| conf.grams)
    }
}

thread_local! {
    /* stable */
    static CONF: RefCell<Conf> = RefCell::default();
    static REGISTRY: RefCell<Registry> = RefCell::default();
    static RECORDS: RefCell<Records> = RefCell::default();
    static MANAGERS: RefCell<Vec<Principal>> = RefCell::default();
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    log_message("executing pre_upgrade".to_string());

    let conf = CONF.with(|cell| cell.borrow().clone());
    let registry = REGISTRY.with(|cell| cell.borrow().clone());
    let records = RECORDS.with(|cell| cell.borrow().clone());
    let managers = MANAGERS.with(|cell| cell.borrow().clone());

    match storage::stable_save((conf, registry, records, managers)) {
        Ok(()) => log_message("INFO :: pre_upgrade :: stable memory saved".to_string()),
        Err(msg) =>
            api::trap(
                &format!("ERROR :: pre_upgrade :: failed to save stable memory. Message: {msg}")
            ),
    }
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let stable_data: Result<
        (Conf, Registry, Records, Vec<Principal>),
        String
    > = storage::stable_restore();
    match stable_data {
        Ok((conf, registry, records, managers)) => {
            CONF.with(|cell| {
                *cell.borrow_mut() = conf;
            });
            REGISTRY.with(|cell| {
                *cell.borrow_mut() = registry;
            });
            RECORDS.with(|cell| {
                *cell.borrow_mut() = records;
            });
            MANAGERS.with(|cell| {
                *cell.borrow_mut() = managers;
            });
        }
        Err(msg) => {
            // Traps in pre_upgrade or post_upgrade will cause the upgrade to be reverted
            // and the state to be restored.
            api::trap(
                &format!("Failed to restore from stable memory. Reverting upgrade. Message: {msg}")
            );
        }
    }

    log_message("executed post_upgrade".to_string());
}

#[init]
fn init(conf: Option<Conf>) {
    if let Some(conf) = conf {
        log_message(
            format!(
                "INFO :: new config: gldt_ledger_canister_id = {}, gld_nft_canister_ids = {:?}",
                conf.gldt_ledger_canister_id,
                conf.gld_nft_canister_ids
            )
        );
        CONF.with(|c| {
            *c.borrow_mut() = conf;
        });
    }

    #[cfg(not(test))]
    MANAGERS.with(|cell| {
        *cell.borrow_mut() = vec![api::caller()];
    });
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Copy)]
pub struct GetRecordsRequest {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct GetRecordsResponse {
    total: usize,
    data: Option<Vec<GldtRecord>>,
}

#[query]
fn get_records(req: GetRecordsRequest) -> Result<GetRecordsResponse, String> {
    let page = req.page.unwrap_or(0);
    let limit = match req.limit {
        Some(val) => {
            if val < 1 { 10 } else if val > 100 { 100 } else { val }
        }
        None => 10,
    };
    let Some(start) = page.checked_mul(limit) else {
        return Err("Overflow when calculating start".to_string());
    };
    RECORDS.with(|r| {
        let entries = &r.borrow().entries;
        let paginated_records: Vec<_> = entries.values().skip(start).take(limit).cloned().collect();
        let data = if paginated_records.is_empty() { None } else { Some(paginated_records) };
        Ok(GetRecordsResponse {
            total: entries.len(),
            data,
        })
    })
}

#[query]
fn get_conf() -> Conf {
    CONF.with(|c| c.borrow().clone())
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct InfoRequest {
    source_canister: Principal,
    nft_id: NftId,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct NftInfo {
    info: Option<GldtRegistryEntry>,
}

type TransferResult = Result<BlockIndex, TransferError>;

#[query]
fn nft_info(args: InfoRequest) -> NftInfo {
    REGISTRY.with(|r| NftInfo {
        info: r.borrow().get_entry(&(args.source_canister, args.nft_id)).cloned(),
    })
}

async fn accept_offer(
    nft_id: NftId,
    gld_nft_canister_id: Principal,
    swap_info: SwapInfo
) -> Result<GldtSwapped, String> {
    let gldt_ledger_canister_id = CONF.with(|c| c.borrow().gldt_ledger_canister_id);
    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get();
    let bid = BidRequest {
        broker_id: None,
        sale_id: swap_info.get_nft_sale_id(),
        escrow_receipt: EscrowReceipt {
            token: token_spec,
            seller: OrigynAccount::principal(swap_info.get_receiving_account().owner),
            buyer: OrigynAccount::principal(api::id()),
            token_id: nft_id,
            amount: swap_info.get_num_tokens().get(),
        },
    };
    let service = gld_nft::Service(gld_nft_canister_id);
    log_message(format!("Placing bid with arguments {bid:?}"));
    match service.sale_nft_origyn(ManageSaleRequest::bid(bid)).await {
        Ok((res,)) => {
            log_message("Received response from sale_nft_origyn. Decifering now.".to_string());
            match res {
                ManageSaleResult::ok(val) => {
                    log_message(format!("Successful response: {val:?}"));
                    if let ManageSaleResponse::bid(bid) = *val {
                        // We expect a bid response and if another is returned, something went wrong.
                        // An error is thrown in that case.
                        let sale_id = match bid.txn_type {
                            BidResponse_txn_type::sale_ended { sale_id, .. } => {
                                sale_id.unwrap_or_default()
                            }
                            _ => String::new(),
                        };
                        Ok(GldtSwapped::new(sale_id, bid.index))
                    } else {
                        Err(
                            format!(
                                "Received invalid ManageSaleResponse from sale_nft_origyn. Expected bid, received {val:?}"
                            )
                        )
                    }
                }
                ManageSaleResult::err(err) => {
                    log_message(format!("Error response: ManageSaleResult : {}", err.text));
                    Err(err.text)
                }
            }
        }
        Err((_, msg)) => Err(format!("Severe error while accepting offer. Message: {msg}")),
    }
}

fn validate_inputs(args: SubscriberNotification) -> Result<(NftId, Principal, SwapInfo), String> {
    // verify caller, only accept calls from valid gld nft canisters
    #[cfg(not(test))]
    let the_caller = api::caller();

    #[cfg(test)]
    let the_caller = args.collection;

    // Extract configuration and validate caller.
    let (gld_nft_canister_id, gld_nft_conf, gldt_ledger_canister_id) = CONF.with(
        |c| -> Result<(Principal, NftCanisterConf, Principal), String> {
            let conf = c.borrow();
            let (gld_nft_canister_id, gld_nft_conf) = conf.gld_nft_canister_ids
                .iter()
                .find(|(x, _)| *x == the_caller)
                .ok_or_else(|| {
                    format!(
                        "invalid caller: was {the_caller}, expected one of {:?}",
                        conf.gld_nft_canister_ids
                            .iter()
                            .map(|(x, _)| x)
                            .collect::<Vec<_>>()
                    )
                })?;

            Ok((*gld_nft_canister_id, gld_nft_conf.clone(), conf.gldt_ledger_canister_id))
        }
    )?;

    // verify nft_id
    let nft_id = args.sale.token_id.clone();
    if nft_id.is_empty() {
        return Err(String::from("NFT ID cannot be empty"));
    }

    // verify subaccount for escrow deposit
    let subaccount: Subaccount = match args.escrow_info.account.sub_account.as_slice().try_into() {
        Ok(x) => x,
        Err(_) => {
            return Err(
                format!(
                    "ERROR: expected a subaccount of length 32 but it was {}",
                    args.escrow_info.account.sub_account.len()
                )
            );
        }
    };

    // verify seller acount as ICRC1 account
    let seller_icrc1: Account = (match args.seller {
        OrigynAccount::principal(p) =>
            Ok(Account {
                owner: p,
                subaccount: None,
            }),
        _ => Err("No valid account found for seller.".to_string()),
    })?;

    // extract token information and config and verify if it is valid
    let (token, config) = match args.sale.sale_type {
        SaleStatusShared_sale_type::auction(t) => (t.token, t.config),
    };
    // verify passed token information
    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get();
    if token != token_spec {
        return Err(
            format!(
                "Token specification are not correct. Expected {token_spec:?}, received: {token:?}"
            )
        );
    }

    // 100 tokens per gram.
    let tokens_minted = calculate_tokens_from_weight(gld_nft_conf.grams)?;

    // validate amount information
    match config {
        PricingConfigShared::ask(Some(features)) => {
            for feature in features {
                match feature {
                    AskFeature::buy_now(val) => {
                        if val != tokens_minted.get() {
                            return Err(
                                format!(
                                    "buy_now price doesn't match the expected value. Expected {}, received {val}.",
                                    tokens_minted.get()
                                )
                            );
                        }
                    }
                    AskFeature::token(val) => {
                        if val != token_spec {
                            return Err(
                                format!(
                                    "Token specification are not correct. Expected {token_spec:?}, received: {token:?}"
                                )
                            );
                        }
                    }
                    AskFeature::kyc(_) | AskFeature::notify(_) => {}
                    ask_feature => {
                        return Err(
                            format!(
                                "Unexpected feature in asked, only token, notify, kyc and buy_now accepted and received AskFeature::{ask_feature:?}"
                            )
                        );
                    }
                }
            }
        }
        pricing_config_shared => {
            return Err(
                format!(
                    "Unexpected pricing_config_shared value, only ask value is accepted and received PricingConfigShared::{pricing_config_shared:?}"
                )
            );
        }
    }

    let swap_info = SwapInfo::new(
        args.sale.sale_id,
        subaccount,
        seller_icrc1,
        api::time(),
        tokens_minted
    );

    Ok((nft_id, gld_nft_canister_id, swap_info))
}

async fn mint_tokens(
    gld_nft_canister_id: Principal,
    swap_info: SwapInfo
) -> Result<GldtLedgerInfo, String> {
    // let issue_info = swap_info.get_issue_info();
    let num_tokens = swap_info.get_num_tokens().clone();
    // let num_tokens = GldtNumTokens::from_weight(swap_info.grams)?;

    let transfer_args = TransferArg {
        memo: Some(swap_info.get_requested_memo()),
        amount: num_tokens.get(),
        fee: None,
        from_subaccount: None,
        to: Account {
            owner: gld_nft_canister_id,
            subaccount: Some(swap_info.get_escrow_subaccount()),
        },
        created_at_time: None,
    };
    let gldt_ledger_canister_id = CONF.with(|c| -> Principal {
        c.borrow().gldt_ledger_canister_id
    });

    let service = gldt_ledger::Service(gldt_ledger_canister_id);

    let result: TransferResult = (match service.icrc1_transfer(transfer_args.clone()).await {
        Ok((v,)) => Ok(v),
        Err((code, message)) =>
            Err(format!("Error while calling icrc1_transfer. Code {code:?}, Message: {message}")),
    })?;
    let block_height: BlockIndex = (match result {
        Ok(height) => Ok(height),
        Err(e) =>
            Err(
                format!(
                    "Error while executing icrc1_transfer with args {transfer_args:?}. Message: {e:?}"
                )
            ),
    })?;
    log_message(
        format!(
            "INFO :: minted {} GLDT at block {block_height} to prinicpal {} with subaccount {:?}",
            num_tokens.get(),
            transfer_args.to.owner,
            transfer_args.to.subaccount
        )
    );
    Ok(GldtLedgerInfo::new(block_height, num_tokens))
}

async fn withdraw_and_burn_escrow(
    collection_id: Principal,
    amount: GldtNumTokens
) -> Result<(), String> {
    let gldt_ledger_canister_id = CONF.with(|c| -> Principal {
        c.borrow().gldt_ledger_canister_id
    });
    let service_ledger = gldt_ledger::Service(gldt_ledger_canister_id);
    let minting_account = (match service_ledger.icrc1_minting_account().await {
        Ok((v,)) => Ok(v),
        Err((code, message)) => {
            let msg = format!(
                "Error while calling icrc1_minting_account. Code {code:?}, Message: {message}"
            );
            Err(msg)
        }
    })?;

    let token_spec = GldtTokenSpec::new(gldt_ledger_canister_id).get();

    let service_gldnft = gld_nft::Service(collection_id);
    match
        service_gldnft.sale_nft_origyn(
            ManageSaleRequest::withdraw(
                gld_nft::WithdrawRequest::deposit(DepositWithdrawDescription {
                    token: token_spec,
                    withdraw_to: OrigynAccount::principal(minting_account),
                    buyer: OrigynAccount::principal(minting_account),
                    amount: amount.get(),
                })
            )
        ).await
    {
        Ok(_) => Ok(()),
        Err((code, message)) => {
            let msg = format!(
                "Error while calling sale_nft_origyn. Code {code:?}, Message: {message}"
            );
            Err(msg)
        }
    }
}

fn update_registry(
    entry_type: &UpdateType,
    nft_id: NftId,
    gld_nft_canister_id: Principal,
    entry: SwapInfo
) -> Result<(), String> {
    log_message(
        format!(
            "INFO :: update_registry :: {entry_type:?} called for nft-id {nft_id} with payload {entry:?}"
        )
    );
    REGISTRY.with(|r| {
        let mut registry = r.borrow_mut();
        match entry_type {
            UpdateType::Init => registry.init(&(gld_nft_canister_id, nft_id), entry),
            UpdateType::Mint => { registry.update_minted(&(gld_nft_canister_id, nft_id), entry) }
            UpdateType::Swap => { registry.update_swapped(&(gld_nft_canister_id, nft_id), entry) }
            UpdateType::Failed => { registry.update_failed(&(gld_nft_canister_id, nft_id), entry) }
            UpdateType::Burn => {
                Err("Invalid registry update type. Burn not implemented yet".to_string())
            }
        }
    })
}

/// This method adds the entry to the permanent record history.
/// This is only called when minting or burning is finalised and is meant to
/// keep track of all mints and burns for historic analysis.
fn add_record(
    nft_id: NftId,
    gld_nft_canister_id: Principal,
    swap_info: &SwapInfo,
    status: RecordStatusInfo
) {
    {
        let records: Records = RECORDS.with(|cell| cell.borrow().clone());

        if records.len() >= records::MAX_NUMBER_OF_RECORDS {
            log_message(
                format!("Error :: Records is full (size : {})", records::MAX_NUMBER_OF_RECORDS)
            );
            return;
        } else if records.len() >= (records::MAX_NUMBER_OF_RECORDS * 8) / 10 {
            let fill_percentage: usize = (records.len() * 100) / records::MAX_NUMBER_OF_RECORDS;

            log_message(
                format!(
                    "Warn :: Records will be full soon (fill percentage : {fill_percentage}, size : {})",
                    records::MAX_NUMBER_OF_RECORDS
                )
            );
        }
    }

    // To avoid any erros at this stage, all faulty values are set to default.
    let weight = CONF.with(|c|
        c.borrow().get_weight_by_collection_id(&gld_nft_canister_id)
    ).unwrap_or(0);

    let block_height = match swap_info.get_ledger_entry() {
        Some(GldtLedgerEntry::Minted(minted)) => minted.get_block_height(),
        _ => Nat::from(0u8),
    };
    RECORDS.with(|r| {
        // let mut service = s.borrow_mut();
        let mut records = r.borrow_mut();

        let entries = &mut records.entries;
        let new_index: BlockIndex = match entries.last_key_value() {
            Some((last_index, _)) => (*last_index).clone() + Nat::from(1u8),
            None => Nat::from(0u8),
        };

        #[cfg(not(test))]
        let timestamp = api::time();
        #[cfg(test)]
        let timestamp = 0; // api::time only available in canister

        let new_entry = GldtRecord::new(
            RecordType::Mint,
            timestamp,
            swap_info.get_receiving_account(),
            gld_nft_canister_id,
            nft_id,
            swap_info.get_escrow_subaccount(),
            swap_info.get_nft_sale_id(),
            weight,
            swap_info.get_num_tokens(),
            block_height,
            status
        );
        entries.insert(new_index.clone(), new_entry);

        records.entries_by_user
            .entry(swap_info.get_receiving_account().owner)
            .or_default()
            .push(new_index);
    });
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GetStatusRequest {
    nft_id: NftId,
    gld_nft_canister_id: Principal,
    sale_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct GetStatusResponse {
    status: Option<SwappingStates>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Copy)]
struct GetSwapsRequest {
    account: Option<Account>,
    page: Option<usize>,
    limit: Option<usize>,
}

type GetSwapsResponse = GetRecordsResponse;

#[query]
fn get_historical_swaps_by_user(req: GetSwapsRequest) -> Result<GetSwapsResponse, String> {
    let principal = match req.account {
        Some(a) => a.owner,
        None => api::caller(),
    };
    let page = req.page.unwrap_or(0);
    let limit = match req.limit {
        Some(val) => {
            if val < 1 { 10 } else if val > 100 { 100 } else { val }
        }
        None => 10,
    };

    let Some(start) = page.checked_mul(limit) else {
        return Err("Overflow when calculating start".to_string());
    };

    RECORDS.with(|r| {
        let default_vec = Vec::new();
        let record_list = r.borrow();
        let mut user_records_indices = (*record_list.entries_by_user
            .get(&principal)
            .unwrap_or(&default_vec)).clone();
        let total = user_records_indices.len();

        user_records_indices.sort_by(|a, b| b.cmp(a));

        let mut paginated_records = Vec::new();
        for item in user_records_indices.iter().skip(start).take(limit) {
            match record_list.entries.get(item) {
                None => {
                    continue;
                }
                Some(record) => {
                    paginated_records.push((*record).clone());
                }
            }
        }

        let data = if paginated_records.is_empty() { None } else { Some(paginated_records) };

        Ok(GetSwapsResponse { total, data })
    })
}

#[query]
fn get_ongoing_swaps_by_user(req: GetSwapsRequest) -> Result<GetSwapsResponse, String> {
    let account = match req.account {
        Some(a) => a,
        None =>
            Account {
                owner: api::caller(),
                subaccount: None,
            },
    };
    let page = req.page.unwrap_or(0);
    let limit = match req.limit {
        Some(val) => {
            if val < 1 { 10 } else if val > 100 { 100 } else { val }
        }
        None => 10,
    };

    let Some(start) = page.checked_mul(limit) else {
        return Err("Overflow when calculating start".to_string());
    };
    let response = REGISTRY.with(|r| {
        let swaps = r.borrow().get_ongoing_swaps_by_user(account);
        GetSwapsResponse {
            total: swaps.len(),
            data: Some(swaps.iter().skip(start).take(limit).cloned().collect::<Vec<_>>()),
        }
    });
    Ok(response)
}

#[query]
fn get_status_of_swap(req: GetStatusRequest) -> Result<GetStatusResponse, String> {
    CONF.with(
        |c| -> Result<(), String> {
            c
                .borrow()
                .gld_nft_canister_ids.iter()
                .find(|(x, _)| *x == req.gld_nft_canister_id)
                .ok_or_else(|| {
                    format!(
                        "invalid GLD NFT canister ID: was {}, expected one of {:?}",
                        req.gld_nft_canister_id,
                        c
                            .borrow()
                            .gld_nft_canister_ids.iter()
                            .map(|(x, _)| x)
                            .collect::<Vec<_>>()
                    )
                })?;
            Ok(())
        }
    )?;
    REGISTRY.with(|r| {
        let response = match r.borrow().get_entry(&(req.gld_nft_canister_id, req.nft_id)) {
            None => GetStatusResponse { status: None },
            Some(entry) => {
                let swap_info = entry.get_issue_info();
                if swap_info.get_nft_sale_id() == req.sale_id {
                    GetStatusResponse {
                        status: Some(entry.get_status_of_swap()),
                    }
                } else {
                    GetStatusResponse { status: None }
                }
            }
        };
        Ok(response)
    })
}

fn notify_fee_compensation_canister() {
    log_message("INFO :: notify_fee_compensation_canister :: called".to_string());
    let canister_id = CONF.with(|c| c.borrow().gldt_fee_compensation_canister_id);

    if let Err(err) = notify(canister_id, "notify_compensation_job", ()) {
        log_message(format!("ERROR :: notify_fee_compensation_canister :: {err:?}"));
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriberNotification {
    escrow_info: SubAccountInfo,
    sale: SaleStatusShared,
    seller: OrigynAccount,
    collection: Principal,
}

#[update]
async fn notify_sale_nft_origyn(args: SubscriberNotification) {
    log_message(format!("Sale notifcation: {args:?}"));

    // STEP 1 : validate inputs
    let (nft_id, gld_nft_canister_id, mut swap_info) = match validate_inputs(args.clone()) {
        Ok(res) => res,
        Err(err) => {
            log_message(format!("ERROR :: {err}"));
            return;
        }
    };

    // STEP 2 : add entry in registry to keep track of running listings
    //          and block any attempts of double minting
    if
        let Err(err) = update_registry(
            &UpdateType::Init,
            nft_id.clone(),
            gld_nft_canister_id,
            swap_info.clone()
        )
    {
        log_message(format!("ERROR :: {err}"));
        return;
    }

    // STEP 3 : mint GLDT to escrow address and then swap GLDTs and NFTs
    //          Careful after this point as tokens are being minted and transfers take place.
    //          First step: mint the tokens to the escrow account.
    match mint_tokens(gld_nft_canister_id, swap_info.clone()).await {
        Ok(gldt_minted) => {
            swap_info.set_ledger_entry(GldtLedgerEntry::Minted(gldt_minted.clone()));
            if
                let Err(err) = update_registry(
                    &UpdateType::Mint,
                    nft_id.clone(),
                    gld_nft_canister_id,
                    swap_info.clone()
                )
            {
                log_message(format!("ERROR :: {err}"));
                return;
            }
            // Second step: accept the offer of the listed NFT
            match accept_offer(nft_id.clone(), gld_nft_canister_id, swap_info.clone()).await {
                Ok(gldt_swapped) => {
                    // All went well and registry is updated and record is added.
                    swap_info.set_swapped(gldt_swapped);
                    if
                        let Err(err) = update_registry(
                            &UpdateType::Swap,
                            nft_id.clone(),
                            gld_nft_canister_id,
                            swap_info.clone()
                        )
                    {
                        log_message(format!("ERROR :: {err}"));
                    }
                    add_record(nft_id.clone(), gld_nft_canister_id, &swap_info, RecordStatusInfo {
                        status: RecordStatus::Success,
                        message: None,
                    });
                    // notify the compensation canister
                    notify_fee_compensation_canister();
                    log_message("INFO :: accept_offer :: success".to_string());
                }
                Err(msg) => {
                    // In case of a failure of the swapping after minting, the escrow is withdrawn
                    // to the minting account to burn the tokens from circulation.
                    log_message(
                        format!(
                            "ERROR :: accept_offer :: Error while performing swap of GLD NFT for GLDT.
                                Attempting to clean up and burn already minted tokens. :: {msg}"
                        )
                    );
                    let amount = swap_info.get_num_tokens();
                    match withdraw_and_burn_escrow(gld_nft_canister_id, amount.clone()).await {
                        Ok(()) => {
                            log_message(
                                format!("Successfully burned {amount:?} GLDT from failed swap.")
                            );
                        }
                        Err(msg) => {
                            log_message(format!("ERROR :: accept_offer :: {msg}"));
                        }
                    }
                    swap_info.set_failed(GldtError::SwappingError(None));

                    // Update the registry and add record
                    if
                        let Err(err) = update_registry(
                            &UpdateType::Failed,
                            nft_id.clone(),
                            gld_nft_canister_id,
                            swap_info.clone()
                        )
                    {
                        log_message(format!("ERROR :: {err}"));
                    }
                    add_record(nft_id.clone(), gld_nft_canister_id, &swap_info, RecordStatusInfo {
                        status: RecordStatus::Failed,
                        message: Some("Error while swapping GLD NFT for GLDT.".to_string()),
                    });
                }
            }
        }
        Err(msg) => {
            log_message(format!("ERROR :: mint_tokens :: {msg}"));
            swap_info.set_failed(GldtError::MintingError(None));
            if
                let Err(err) = update_registry(
                    &UpdateType::Failed,
                    nft_id.clone(),
                    gld_nft_canister_id,
                    swap_info.clone()
                )
            {
                log_message(format!("ERROR :: {err}"));
            }
            add_record(nft_id.clone(), gld_nft_canister_id, &swap_info, RecordStatusInfo {
                status: RecordStatus::Failed,
                message: Some("Error while minting GLDT.".to_string()),
            });
        }
    }
}

#[derive(CandidType, Debug, PartialEq)]
pub struct LockedInfoResponse {
    total_number_of_bars_locked: usize,
    total_weight_locked: usize,
}

#[query]
fn get_locked_info() -> LockedInfoResponse {
    let count = REGISTRY.with(|r| r.borrow().count_number_of_nfts_swapped_per_collection());

    let mut total_number_of_bars_locked: usize = 0;
    let mut total_weight_locked: usize = 0;
    count.iter().for_each(|(collection_id, count)| {
        let weight = CONF.with(|c| c.borrow().get_weight_by_collection_id(collection_id)).unwrap_or(
            0
        );
        total_weight_locked += usize::from(weight) * count;
        total_number_of_bars_locked += count;
    });
    LockedInfoResponse {
        total_number_of_bars_locked,
        total_weight_locked,
    }
}

#[query]
fn fetch_metadata() -> String {
    let registry_data = REGISTRY.with(|cell| cell.borrow().clone());
    let conf_data = CONF.with(|cell| cell.borrow().clone());
    let records_data = RECORDS.with(|cell| cell.borrow().clone());
    let managers_data = MANAGERS.with(|cell| cell.borrow().clone());

    json!({
        "registry": registry_data,
        "configuration": conf_data,
        "records": records_data,
        "managers": managers_data
    }).to_string()
}

#[update]
fn import_data(json_data: String) -> Result<String, CustomError> {
    validate_caller()?;

    let previous_metadata = fetch_metadata();

    let data: Value = serde_json
        ::from_str(&json_data)
        .map_err(|e|
            CustomError::new_with_message(
                ErrorType::Other,
                format!("Error parsing JSON: {:?}", e).to_string()
            )
        )?;

    let registry_data = data["registry"].clone();
    let records_data = data["records"].clone();
    let conf_data = data["configuration"].clone();
    let managers_data = data["managers"].clone();

    REGISTRY.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(registry_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing registry data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    RECORDS.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(records_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing registry data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    CONF.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(conf_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing configuration data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    MANAGERS.with(
        |cell| -> Result<(), CustomError> {
            *cell.borrow_mut() = serde_json
                ::from_value(managers_data)
                .map_err(|e|
                    CustomError::new_with_message(
                        ErrorType::Other,
                        format!("Error parsing managers data: {:?}", e).to_string()
                    )
                )?;
            Ok(())
        }
    )?;

    Ok(previous_metadata)
}

fn validate_caller() -> Result<(), CustomError> {
    #[cfg(test)]
    return Ok(());

    #[cfg(not(test))]
    MANAGERS.with(|m| {
        if !m.borrow().contains(&api::caller()) {
            return Err(
                CustomError::new_with_message(ErrorType::Unauthorized, "Invalid caller".to_string())
            );
        }
        Ok(())
    })
}

export_candid!();

#[cfg(test)]
mod test;
