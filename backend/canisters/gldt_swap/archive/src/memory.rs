use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

pub type VM = VirtualMemory<DefaultMemoryImpl>;
const UPGRADES: MemoryId = MemoryId::new(0);
pub const SWAP_HISTORY: MemoryId = MemoryId::new(1);
pub const USER_SWAP_ID_MAP: MemoryId = MemoryId::new(2);

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl> = MemoryManager::init(
        DefaultMemoryImpl::default()
    );
}

pub fn get_upgrades_memory() -> VM {
    get_memory(UPGRADES)
}

fn get_memory(id: MemoryId) -> VM {
    MEMORY_MANAGER.with(|m| m.get(id))
}

pub fn get_swap_history_memory() -> VM {
    get_memory(SWAP_HISTORY)
}

pub fn get_user_swap_id_memory() -> VM {
    get_memory(USER_SWAP_ID_MAP)
}
