use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

pub type VM = VirtualMemory<DefaultMemoryImpl>;
const UPGRADES: MemoryId = MemoryId::new(0);
pub const STAKE_POSITIONS: MemoryId = MemoryId::new(1);

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

pub fn get_stake_positions_memory() -> VM {
    get_memory(STAKE_POSITIONS)
}
