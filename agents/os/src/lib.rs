use std::cell::RefCell;

use candid::Principal;
use domain::State;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

pub mod api;
pub mod domain;

const STATE_MEM_ID: MemoryId = MemoryId::new(0);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMEORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STATE: RefCell<StableBTreeMap<Principal, State, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMEORY_MANAGER.with(|m| m.borrow().get(STATE_MEM_ID))
        )
    );

}
