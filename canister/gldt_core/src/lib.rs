//! GLDT is a digital token 100% backed by physical gold in the form
//! of NFTs in a ratio of 1 gram of gold NFTs equals 100 GLDT. The
//! NFTs have their ownership registered to this canister, which is
//! used to convert NFTs to GLDT and back.  The GLDT canister
//! purchases NFTs by minting tokens and sells NFTs against the
//! burning of tokens.
//!
//! The code of this canister is generic in the sense that it is not
//! tied to any particular type of NFT, except for the notion of
//! 'grams' which is tied to tokens in a ratio of one gram equals 100
//! tokens. Thus, in principle, the same code can be used for NFT of
//! any physical commodity measured in grams. The cansiter could
//! generalized further by replacing grams by some generic quantity,
//! but doing so right now (2023) seems to have little benefit and only
//! hamper the readability of the code.
//!
//! The GLDT canister collaborates with the canisters holding gold
//! NFTs as well as the GLDT ledger, which an instance of a standard
//! IC ledger.
//!
//!
//! ```
//! NFT                  GLDT            Ledger
//!  |                    |                |
//!  | offer request (1)  |                |
//!  +------------------->|                |
//!  |                    |   mint (2)     |
//!  |                    +--------------->|
//!  |                    |                |
//!  |                    |<---------------+
//!  |                    |                |
//!  |<-------------------+                +---+
//!  |        offer       |                |   | transact (3b)
//!  |                    |                |<--+
//!  +---+                |                |
//!  |   | accept (3a)    |                |
//!  |<--+                |                |
//!  |                    |                |
//! ```
//!
//! The lifecycle of one NFT is as follows.
//!
//! * Offer request made (1), i.e., an NFT canister requests an offer for
//! a particular NFT.
//!
//! * An offer is made by  minting (2) tokens to an escrow account of NFT
//! canister.
//!
//! * The offer is accepted (3a) on the NFT canister: the NFT now belongs
//! to GLDT canister and the minted tokens are in
//! circulation (3b). Alternatively, the offer is rejected and the tokens
//! are burned.
//!
//! * The view of the ownership of NFT from the NFT canister and from
//! the GLDT canister is periodically audited (to be implemented).
//!
//! * The GLDT canister releases an NFT against proof that the
//! corresponding number of tokens have been burned (to be
//! implemented).
//!
//! The GLDT ledger uses the account ID of the gldt cansiter (an
//! instance of this code) as its 'minting account'.  Computed as
//! `$(dfx ledger account-id --of-canister gldt)`. The GLDT canister
//! also needs to point to the ledger canister as given by `$(dfx
//! canister id ledger)`.

use candid::{ candid_method, CandidType, Deserialize, Principal, Nat };
// use canistergeek_ic_rust::logger::log_message;
use ic_cdk::{ api, api::call::CallResult, call, storage };
use ic_cdk_macros::{ init, query, update };
// use ic_ledger_types::Block;
// use ic_ledger_types::{AccountIdentifier, Memo, Subaccount, Tokens};
// use ic_ledger_types::ic;
use icrc_ledger_types::icrc1::{
    account::{ Account, Subaccount },
    transfer::{ BlockIndex, Memo, NumTokens, TransferArg, TransferError },
};
use serde::Serialize;
// use serde_bytes::ByteBuf;
// use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::hash::Hash;

// mod declarations;
mod declarations;
use declarations::gld_nft::{
    self,
    ManageSaleRequest,
    BidRequest,
    SaleStatusShared,
    Account as OrigynAccount,
    SubAccountInfo,
    EscrowReceipt,
    TokenSpec,
    ICTokenSpec,
    SaleStatusShared_sale_type,
    PricingConfigShared__1,
    AskFeature,
    ManageSaleResult,
    ICTokenSpec_standard,
};
use declarations::icrc1;

/// Constants
pub const GLDT_SUBDIVIDABLE_BY: u64 = 100_000_000;
pub const GLDT_PRICE_RATIO: u8 = 100;
pub const GLD_NFT_ESCROW_FEE_ADDITION: u64 = 10_000 * 10;

/// The configuration points to the canisters that this canister
/// collaborates with, viz., the GLDT ledger canister and the NFT
/// canisters.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    /// The canister ID of the GLDT ledger canister.
    gldt_ledger_canister_id: Principal,
    /// Canister IDs of the Origyn NFT canisters that manages gold NFTs.
    gld_nft_canister_ids: Vec<(Principal, NftCanisterConf)>,
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

impl Default for Conf {
    fn default() -> Self {
        Conf {
            gldt_ledger_canister_id: Principal::anonymous(),
            gld_nft_canister_ids: Vec::new(),
        }
    }
}

/// An NFT is identified by a string.
type NftId = String;

/// An NFT has a certain weight <65535
type NftWeight = u16;

/// Record of information about an NFT for which GLDT has been burned.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtBurned {
    /// The block height at which the tokens minted for this NFT were
    /// burned.
    burn_block_height: u64,
}

/// Record of information about an NFT for which GLDT has been minted.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtMinted {
    /// Block height when the GLDT was minted. Must be non-zero and
    /// point to a block with a minting transaction with the right
    /// number of tokens and subaccount.
    mint_block_height: BlockIndex,

    /// The last timestamp when this NFT was audited, i.e., when it
    /// was verified that this NFT belongs to this canister, or zero
    /// if no audit has been made.
    last_audited_timestamp_seconds: u64,

    /// The end of an NFT lifecycle in the GLDT canister is the
    /// burning of the minted tokens and the release of the
    /// corresponding NFT.
    burned: Option<GldtBurned>,
}

/// Record of information about an NFT for which an offer has been made.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldNft {
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gld_nft_canister_id: Principal,
    /// The number of grams that this NFT is reported to have.
    grams: NftWeight,
    /// This field is passed verbatim from the offer request.
    requested_memo: Memo,
    /// The subaccount to which tokens are minted. This is always a subaccount of a GLD NFT canister.
    to_subaccount: Subaccount,
    /// The account who owned the NFT and triggered the swap for GLDT.
    receiving_account: Account,
    /// The timestamp when the request to issue GLDT was issued.
    gldt_minting_timestamp_seconds: u64,

    /// Filled when tokens are successfully minted.
    minted: Option<GldtMinted>,

    /// Optional reference to a previous minting/burning pair for this
    /// NFT as a historial record. If specified, the record must
    /// satisfy 'is_burned'.
    older_record: Option<Box<GldNft>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
enum RecordType {
    Mint,
    Burn,
}

/// Record of successful minting or burning of GLDT for GLD NFTs
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GldtRecord {
    /// The index of the transaction record in the GLDT swapping record history
    index: BlockIndex,
    /// The type of transaction, either "mint" or "burn"
    record_type: RecordType,
    /// The canister ID of the Origyn NFT canister that manages this NFT.
    gld_nft_canister_id: Principal,
    /// The escrow account where the GLDT tokens are minted to for the sale
    escrow_subaccount: Subaccount,
    /// The sale id of the NFT listing in the GLD NFT canister
    nft_sale_id: String,
    /// The number of grams that this NFT is reported to have.
    grams: NftWeight,
    /// The id of the NFT that was locked up
    nft_id: NftId,
    /// The account who is swapping the NFT for GLDT or vice versa.
    counterparty: Account,
    /// The timestamp when the request to issue GLDT was issued.
    gldt_minting_timestamp_seconds: u64,
    /// The amount of tokens minted.
    gldt_minted: NumTokens,
    /// The block index on the GLDT ledger when the GLDT were minted or burned.
    block_height: BlockIndex,
    /// The memo added to the GLDT ledger on minting
    memo: Memo,
}

impl GldNft {
    fn is_burned(&self) -> bool {
        if let Some(minted) = &self.minted {
            if let Some(burned) = &minted.burned { burned.burn_block_height > 0 } else { false }
        } else {
            false
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, Default)]
pub struct GldtService {
    conf: Conf,
    nfts: BTreeMap<(Principal, NftId), GldNft>,
    records: BTreeMap<BlockIndex, GldtRecord>,
}

thread_local! {
    /* stable */
    static SERVICE: RefCell<GldtService> = RefCell::default();
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("executing pre_upgrade");
    canistergeek_ic_rust::logger::log_message(format!("executing pre_upgrade"));

    // canister geek data
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    let service = SERVICE.with(|cell| cell.borrow_mut().clone());

    storage::stable_save((service, monitor_stable_data, logger_stable_data)).unwrap();
    // SERVICE.with(|cell| storage::stable_save((cell.take(),)).unwrap());
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let stable_data: Result<
        (
            GldtService,
            canistergeek_ic_rust::monitor::PostUpgradeStableData,
            canistergeek_ic_rust::logger::PostUpgradeStableData,
        ),
        String
    > = storage::stable_restore();
    match stable_data {
        Ok((service, monitor_stable_data, logger_stable_data)) => {
            SERVICE.with(|cell| {
                *cell.borrow_mut() = service;
            });
            canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
            canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
        }
        Err(_) => {}
    }

    ic_cdk::println!("executed post_upgrade");
    canistergeek_ic_rust::logger::log_message(format!("executed post_upgrade"));
}

#[init]
#[candid_method(init)]
fn init(conf: Option<Conf>) {
    if let Some(conf) = conf {
        canistergeek_ic_rust::logger::log_message(
            format!(
                "INFO :: new config: gldt_ledger_canister_id = {}, gld_nft_canister_ids = {:?}",
                conf.gldt_ledger_canister_id,
                conf.gld_nft_canister_ids
            )
        );
        SERVICE.with(|s| {
            s.borrow_mut().conf = conf;
        })
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GetRecordsRequest {
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct GetRecordsResponse {
    total: u64,
    data: Option<Vec<GldtRecord>>,
}

#[query]
#[candid_method(query)]
fn get_records(req: GetRecordsRequest) -> GetRecordsResponse {
    let page = req.page.unwrap_or(0);
    let limit = match req.limit {
        Some(val) => if val < 1 { 10 } else if val > 100 { 100 } else { val }
        None => 10,
    };
    let res: GetRecordsResponse = SERVICE.with(|s| {
        let records = &mut s.borrow_mut().records;
        let start = (page - 1) * limit;
        let paginated_records = records
            .values()
            .skip(start as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        return GetRecordsResponse { total: records.len() as u64, data: Some(paginated_records) };
    });
    return res;
}

#[update]
#[candid_method(update)]
fn get_conf() -> Conf {
    canistergeek_ic_rust::logger::log_message(format!("INFO :: get_conf"));
    SERVICE.with(|s| s.borrow_mut().conf.clone())
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct InfoRequest {
    source_canister: Principal,
    nft_id: NftId,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct NftInfo {
    info: Option<GldNft>,
}

type TransferResult = Result<BlockIndex, TransferError>;

#[update]
#[candid_method(update)]
async fn nft_info(args: InfoRequest) -> NftInfo {
    canistergeek_ic_rust::logger::log_message(format!("INFO :: nft_info. Arguments: {:?}", args));
    SERVICE.with(|s| NftInfo {
        info: s
            .borrow()
            .nfts.get(&(args.source_canister, args.nft_id))
            .map(|x| x.clone()),
    })
}

fn delete_nft_entry_from_list(nft_id: &NftId) -> Result<(), String> {
    let the_caller = api::caller();
    SERVICE.with(|s| {
        let nfts = &mut s.borrow_mut().nfts;
        match nfts.entry((the_caller, nft_id.to_string())) {
            btree_map::Entry::Occupied(o) => {
                o.remove_entry();
                Ok(())
            }
            _ => Err(format!("NFT ID {} not found in list.", nft_id)),
        }
    })
}

async fn accept_offer(record: GldtRecord, token_info: TokenSpec) -> Result<(), String> {
    let bid = BidRequest {
        broker_id: None,
        sale_id: record.nft_sale_id,
        escrow_receipt: EscrowReceipt {
            token: token_info,
            seller: OrigynAccount::principal(record.counterparty.owner),
            buyer: OrigynAccount::principal(record.gld_nft_canister_id),
            token_id: record.nft_id,
            amount: record.gldt_minted,
        },
    };
    let service = gld_nft::SERVICE(api::caller());
    canistergeek_ic_rust::logger::log_message(format!("Placing bid with arguments {:?}", bid));
    match service.sale_nft_origyn(ManageSaleRequest::bid(bid)).await {
        Ok((res,)) => {
            match res {
                ManageSaleResult::ok(val) => {
                    canistergeek_ic_rust::logger::log_message(
                        format!("Successfuly response: {:?}", val)
                    );
                    return Ok(());
                }
                ManageSaleResult::err(err) => Err(err.text),
            }
        }
        Err((_, msg)) => Err(format!("Severe error while accepting offer. Message: {}", msg)),
    }
}

async fn validate_inputs(args: SubscriberNotification) -> Result<(GldtRecord, TokenSpec), String> {
    // verify nft_id
    let nft_id = args.sale.token_id.clone();
    if nft_id.clone().is_empty() {
        return Err(String::from("NFT ID cannot be empty"));
    }

    // verify caller, only accept calls from valid gld nft canisters
    let the_caller = api::caller();
    // Extract configuration and validate caller.
    let (gld_nft_canister_id, gld_nft_conf) = SERVICE.with(
        |s| -> Result<(Principal, NftCanisterConf), String> {
            let conf = &s.borrow().conf;
            let (gld_nft_canister_id, gld_nft_conf) = conf.gld_nft_canister_ids
                .iter()
                .find(|(x, _)| *x == the_caller)
                .ok_or_else(|| {
                    format!(
                        "invalid caller: was {}, expected one of {:?}",
                        the_caller,
                        conf.gld_nft_canister_ids
                            .iter()
                            .map(|(x, _)| x)
                            .collect::<Vec<_>>()
                    )
                })?;

            Ok((gld_nft_canister_id.clone(), gld_nft_conf.clone()))
        }
    )?;

    // verify subaccount for escrow deposit
    let subaccount: Subaccount = match args.escrow_info.account.sub_account.as_slice().try_into() {
        Ok(x) => x,
        Err(_) => {
            let msg = format!(
                "ERROR: expected a subaccount of length {} but it was {}",
                32,
                args.escrow_info.account.sub_account.len()
            );
            canistergeek_ic_rust::logger::log_message(msg.clone());
            return Err(msg);
        }
    };

    // verify seller acount as ICRC1 account
    let seller_icrc1: Account = (match args.seller.clone() {
        OrigynAccount::principal(p) =>
            Ok(Account {
                owner: p.clone(),
                subaccount: None,
            }),
        _ => {
            let msg = format!("No valid account found for seller.");
            canistergeek_ic_rust::logger::log_message(msg.clone());
            Err(msg)
        }
    })?;

    // extract token information and config and verify if it is valid
    let (token, config) = match args.sale.sale_type {
        SaleStatusShared_sale_type::auction(t) => (t.token, t.config),
    };
    // verify passed token info
    let token_info: TokenSpec = TokenSpec::ic(ICTokenSpec {
        id: None,
        fee: Some(Nat::from(10000)),
        decimals: Nat::from(8),
        canister: api::id(),
        standard: ICTokenSpec_standard::ICRC1,
        symbol: String::from("GLDT"),
    });
    if token != token_info {
        let msg = format!(
            "Token specification are not correct. Expected {:?}, received: {:?}",
            token_info,
            token
        );
        canistergeek_ic_rust::logger::log_message(msg.clone());
        return Err(msg);
    }

    // 100 tokens per gram.
    let tokens_minted: NumTokens = NumTokens::from(
        GLDT_SUBDIVIDABLE_BY * (gld_nft_conf.grams as u64) * 100
    );
    // extract amount information and validate
    let amount = (match config {
        PricingConfigShared__1::ask(Some(features)) => {
            let mut amount: Nat = Nat::from(0);
            for feature in features {
                if let AskFeature::buy_now(val) = feature {
                    amount = val;
                }
            }
            if amount == tokens_minted {
                Ok(amount)
            } else {
                let msg = format!(
                    "buy_now price doesn't match the expected value. Expected {}, received {}.",
                    tokens_minted,
                    amount
                );
                canistergeek_ic_rust::logger::log_message(msg.clone());
                Err(msg)
            }
        }
        _ => Err(String::from("Couldn't find buy_now price.")),
    })?;

    let record = GldtRecord {
        index: Nat::from(0),
        record_type: RecordType::Mint,
        gld_nft_canister_id,
        escrow_subaccount: subaccount,
        nft_sale_id: args.sale.sale_id,
        grams: gld_nft_conf.grams,
        nft_id,
        counterparty: seller_icrc1,
        gldt_minting_timestamp_seconds: 0,
        gldt_minted: amount,
        block_height: Nat::from(0),
        memo: Memo::from(0),
    };

    Ok((record, token_info))
}

fn add_to_queue(record: GldtRecord) -> Result<(), String> {
    // Insert a new entry for this nft_id. If an entry is already
    // present, replace it if inactive; otherwise it is an error to
    // request a new offer for this NFT.
    SERVICE.with(|s| {
        let nfts = &mut s.borrow_mut().nfts;
        let new_entry = GldNft {
            gld_nft_canister_id: record.gld_nft_canister_id.clone(),
            grams: record.grams.clone(),
            requested_memo: record.memo.clone(),
            to_subaccount: record.escrow_subaccount.clone(),
            receiving_account: record.counterparty.clone(),
            gldt_minting_timestamp_seconds: record.gldt_minting_timestamp_seconds.clone(),
            minted: None,
            older_record: None,
        };
        match nfts.entry((record.gld_nft_canister_id.clone(), record.nft_id.clone())) {
            btree_map::Entry::Vacant(v) => {
                v.insert(new_entry);
            }
            btree_map::Entry::Occupied(mut o) => {
                if o.get().is_burned() {
                    canistergeek_ic_rust::logger::log_message(
                        format!(
                            "INFO :: replacing inactive entry for NFT: {}: old entry: {:?}",
                            record.nft_id.clone(),
                            o.get()
                        )
                    );
                    o.insert(GldNft {
                        older_record: Some(Box::new(o.get().clone())),
                        ..new_entry
                    });
                } else {
                    let msg = format!(
                        "There is already an active entry for NFT: {}. Canceling new minting of tokens.",
                        record.nft_id.clone()
                    );
                    return Err(msg);
                }
            }
        }
        Ok(())
    })
}

async fn mint_tokens(record: GldtRecord) -> Result<BlockIndex, String> {
    let transfer_args = TransferArg {
        memo: Some(record.memo),
        amount: record.gldt_minted.clone(),
        fee: None,
        from_subaccount: None,
        to: Account {
            owner: record.gld_nft_canister_id,
            subaccount: Some(record.escrow_subaccount),
        },
        created_at_time: None,
    };
    let gldt_ledger_canister_id = SERVICE.with(|s| -> Principal {
        s.borrow().conf.gldt_ledger_canister_id
    });

    let service = icrc1::SERVICE(gldt_ledger_canister_id);

    // let result: TransferResult = match icrc1_transfer(transfer_args.clone()).await {
    let result: TransferResult = match service.icrc1_transfer(transfer_args.clone()).await {
        Ok((v,)) => v,
        Err((code, message)) => {
            let _ = delete_nft_entry_from_list(&record.nft_id);
            return Err(
                format!("Error while calling icrc1_transfer. Code {:?}, Message: {}", code, message)
            );
        }
    };
    let block_height: BlockIndex = match result {
        Ok(height) => height,
        Err(e) => {
            let _ = delete_nft_entry_from_list(&record.nft_id);
            return Err(
                format!(
                    "Error while executing icrc1_transfer with args {:?}. Message: {:?}",
                    transfer_args,
                    e
                )
            );
        }
    };
    canistergeek_ic_rust::logger::log_message(
        format!(
            "INFO :: minted {} GLDT at block {} to prinicpal {} with subaccount {:?}",
            record.gldt_minted,
            block_height,
            transfer_args.to.owner,
            transfer_args.to.subaccount
        )
    );
    Ok(block_height)
}

async fn update_record(record: GldtRecord) -> Result<String, String> {
    SERVICE.with(|s| {
        let records = &mut s.borrow_mut().records;
        let new_index: BlockIndex = match records.last_key_value() {
            Some((last_index, _)) => (*last_index).clone() + Nat::from(1),
            None => Nat::from(0),
        };
        let new_record = GldtRecord { index: new_index.clone(), ..record.clone() };
        records.insert(new_index, new_record);

        // IMPORTANT: don't panic after async call!!!
        //
        // The code that follows looks rather complicated, but it is
        // almost only error checking due to the fact that we cannot be
        // sure that the canister state before and after the async call
        // match, even for the entry of the NFT being offered.

        let nfts = &mut s.borrow_mut().nfts;
        match nfts.entry((record.gld_nft_canister_id.clone(), record.nft_id.clone())) {
            btree_map::Entry::Vacant(v) => {
                // This should never happen as an entry for this NFT
                // was created before the async call was made, but we
                // try to handle it gracefully anyway. Perhaps the
                // canister was upgraded in between and state was lost.
                ic_cdk::println!(
                    "WARNING: tokens minted, but no entry to update! Attempting to rectify by creating a new entry. Record: {:?}",
                    record
                );
                v.insert(GldNft {
                    gld_nft_canister_id: record.gld_nft_canister_id.clone(),
                    grams: record.grams.clone(),
                    requested_memo: record.memo.clone(),
                    to_subaccount: record.escrow_subaccount.clone(),
                    receiving_account: record.counterparty.clone(),
                    gldt_minting_timestamp_seconds: record.gldt_minting_timestamp_seconds.clone(),
                    minted: Some(GldtMinted {
                        mint_block_height: record.block_height.clone(),
                        last_audited_timestamp_seconds: 0,
                        burned: None,
                    }),
                    older_record: None,
                });
            }
            btree_map::Entry::Occupied(mut o) => {
                // Do sanity checking.
                let mut problems = Vec::new();
                if o.get().gld_nft_canister_id != record.gld_nft_canister_id {
                    problems.push(
                        format!(
                            "NFT canister ID - recorded: {}, expected: {}",
                            o.get().gld_nft_canister_id,
                            record.gld_nft_canister_id
                        )
                    );
                }
                if o.get().grams != record.grams {
                    problems.push(
                        format!(
                            "weight in grams - recorded: {}, expected: {}",
                            o.get().grams,
                            record.grams
                        )
                    );
                }
                if o.get().requested_memo != record.memo {
                    problems.push(
                        format!(
                            "memo - recorded: {:?}, expected: {:?}",
                            o.get().requested_memo,
                            record.memo
                        )
                    );
                }
                if o.get().to_subaccount != record.escrow_subaccount {
                    problems.push(
                        format!(
                            "escrow subaccount - recorded: {:?}, expected: {:?}",
                            o.get().to_subaccount,
                            record.escrow_subaccount
                        )
                    );
                }
                if o.get().gldt_minting_timestamp_seconds != record.gldt_minting_timestamp_seconds {
                    problems.push(
                        format!(
                            "timestamp - recorded: {}, expected: {}",
                            o.get().gldt_minting_timestamp_seconds,
                            record.gldt_minting_timestamp_seconds
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
                        record
                    );
                    canistergeek_ic_rust::logger::log_message(msg.clone());
                } else {
                    match &o.get().minted {
                        None => {
                            // This is the happy path.
                            ic_cdk::println!(
                                "INFO: offer for NFT {} with block height {}",
                                record.nft_id,
                                record.block_height
                            );
                            o.get_mut().minted = Some(GldtMinted {
                                mint_block_height: record.block_height.clone(),
                                last_audited_timestamp_seconds: 0,
                                burned: None,
                            });
                        }
                        // This should never happen as a request to mint
                        // token is only issued with the 'minted' set to
                        // None. But we try to handle it as gracefully as
                        // possible.
                        Some(minted) => {
                            if minted.burned.is_some() {
                                ic_cdk::println!(
                                    "WARNING: offer for NFT {} with block height {} - inactive entry overwritten {:?}",
                                    record.nft_id,
                                    record.block_height,
                                    minted
                                );
                                o.get_mut().minted = Some(GldtMinted {
                                    mint_block_height: record.block_height.clone(),
                                    last_audited_timestamp_seconds: 0,
                                    burned: None,
                                });
                            } else {
                                // If the block heights are equal, there is no issue
                                if minted.mint_block_height != record.block_height.clone() {
                                    ic_cdk::println!(
                                        "ERROR: possible double minting for NFT {}; tokens minted to {:?} at block height {}, previous record indicating minting at block height {}",
                                        record.nft_id,
                                        record.escrow_subaccount,
                                        record.block_height.clone(),
                                        minted.mint_block_height
                                    );
                                } else {
                                    ic_cdk::println!(
                                        "WARNING: ignoring double response for NFT {}",
                                        record.nft_id
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    Ok(format!("Successfully minted GLDT tokens for GLD NFT. Record: {:?}", record))
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriberNotification {
    escrow_info: SubAccountInfo,
    sale: SaleStatusShared,
    seller: OrigynAccount,
    collection: Principal,
}

#[update]
#[candid_method(update)]
async fn notify_sale_nft_origyn(args: SubscriberNotification) -> Result<String, String> {
    canistergeek_ic_rust::logger::log_message(format!("Sale notifcation: {:?}", args));
    canistergeek_ic_rust::monitor::collect_metrics();
    ic_cdk::println!("Sale notifcation: {:?}", args);

    // validate inputs
    let (record, token_info) = (match validate_inputs(args.clone()).await {
        Ok(res) => Ok(res),
        Err(err) => {
            let msg = format!("ERROR :: {}", err);
            canistergeek_ic_rust::logger::log_message(msg.clone());
            Err(msg)
        }
    })?;

    // add temporary record entry to keep track of running listings
    add_to_queue(record.clone())?;

    // Careful after this point as tokens are being minted and transfers take place.
    // First mint the tokens to the escrow account.
    let res = match mint_tokens(record.clone()).await {
        Ok(block_height) => {
            let updated_record = GldtRecord { block_height, ..record };
            // Second accept the offer of the listed NFT
            match accept_offer(updated_record.clone(), token_info.clone()).await {
                Ok(_) => update_record(updated_record.clone()).await,
                Err(msg) => Err(msg),
            }
        }
        Err(msg) => {
            Err(msg)
            // handle error case
            // 1. remove entry from temporary list
            // 2. return notification that minting failed and it needs to be retried
            // Case: What happens if the minting fails and the NFT stays listed? Can we let the frontend
            // call directly the GLDT canister with the notify method if the values are correct?
        }
    };
    return res;

    // validate_inputs()
    // add_tmp_record()
    // mint_tokens()
    // accept_offer()
    // update_registry()
    // clean_up()

    // match request_offer(request).await {
    //     Ok(r) => {
    //         let msg = format!("INFO: minted {} GLDT at block {}", r.tokens_minted, r.block_height);
    //         canistergeek_ic_rust::logger::log_message(msg.clone());
    //         return Ok(msg);
    //     }
    //     Err(e) => {
    //         let msg = format!("ERROR: {}", e);
    //         canistergeek_ic_rust::logger::log_message(msg.clone());
    //         return Err(msg);
    //     }
    // }
}

// for monitoring during development
#[query(name = "getCanistergeekInformation")]
#[candid_method(query)]
async fn get_canistergeek_information(
    request: canistergeek_ic_rust::api_type::GetInformationRequest
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    validate_caller();
    canistergeek_ic_rust::get_information(request)
}

#[update(name = "updateCanistergeekInformation")]
#[candid_method(query)]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest
) -> () {
    validate_caller();
    canistergeek_ic_rust::update_information(request);
}

fn validate_caller() -> () {}

#[cfg(not(test))]
candid::export_service!();

#[cfg(not(test))]
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[test]
fn check_candid_interface() {
    use candid::utils::{ service_compatible, CandidSource };
    use std::path::Path;

    candid::export_service!();
    let new_interface = __export_service();

    service_compatible(
        CandidSource::Text(&new_interface),
        CandidSource::File(Path::new("src/gldt_core.did"))
    ).unwrap();
}

// export_candid!();
