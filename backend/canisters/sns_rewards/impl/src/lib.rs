use ic_cdk::export_candid;
use ic_stable_structures::{
    memory_manager::{ MemoryId, MemoryManager, VirtualMemory },
    storable::Bound,
    StableBTreeMap,
    DefaultMemoryImpl,
    Storable,
};
use icrc_ledger_types::icrc1::account::Account;
use candid::{ CandidType, Principal, Deserialize, Encode, Decode };
use std::cell::RefCell;
use std::borrow::Cow;

mod lifecycle;
mod jobs;
mod queries;
mod updates;

type NeuronId = u64;
type Maturity = u64;

type VM = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100000;

const PRINCIPAL_NEURONS_MEM_ID: MemoryId = MemoryId::new(0);
const NEURON_MATURITY_MEM_ID: MemoryId = MemoryId::new(1);

struct PaymentEvent {
    index: u64,
    subaccount: [u8; 32],
    status: PaymentEventStatus,
}

pub enum PaymentEventStatus {
    Received,
    Settled,
}

#[derive(CandidType, Deserialize)]
struct UserInfo {
    neuron_ids: Vec<NeuronId>,
    ogy_reward_address: Option<Account>,
    icp_reward_address: Option<Account>,
    gldgov_reward_address: Option<Account>,
}

impl Storable for UserInfo {
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: MAX_VALUE_SIZE,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // /// Mapping of each principal to its neurons
    // static PRINCIPAL_NEURONS: RefCell<
    //     StableBTreeMap<Principal, NeuronId, VM>
    // > = MEMORY_MANAGER.with(|mm| {
    //     RefCell::new(StableBTreeMap::init(mm.borrow().get(PRINCIPAL_NEURONS_MEM_ID)))
    // });

    /// Mapping of each neuron to its accumulated maturity and last fetched maturity
    static NEURON_MATURITY: RefCell<
        StableBTreeMap<NeuronId, (Maturity, Maturity), VM>
    > = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(NEURON_MATURITY_MEM_ID)))
    });

    // Mapping of each user to their info (principal, user_info)
    static PRINCIPAL_NEURONS: RefCell<
        StableBTreeMap<Principal, UserInfo, VM>
    > = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(PRINCIPAL_NEURONS_MEM_ID)))
    });

    // Mapping
}

export_candid!();
