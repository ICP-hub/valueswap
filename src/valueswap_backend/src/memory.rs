use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

// A memory for upgrades, where data from the heap can be serialized/deserialized.
const UPGRADES: MemoryId = MemoryId::new(0);

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.
const STABLE_BTREE: MemoryId = MemoryId::new(1);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

pub fn get_stable_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STABLE_BTREE))
}

const POOL_DATA: MemoryId = MemoryId::new(0);

const USERS_POOL : MemoryId = MemoryId::new(1);

pub fn get_pool_data() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(POOL_DATA))
}

pub fn get_user_token_memory() -> Memory{
    MEMORY_MANAGER.with(|m| m.borrow().get(USERS_POOL))
}