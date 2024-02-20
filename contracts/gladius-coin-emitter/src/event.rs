//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address};

// INITIALIZE EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeEvent {
    pub minter: Address,
    pub pegged: Address,
    pub ratio: u32
}

pub(crate) fn initialize(
    e: &Env, 
    minter: Address,
    pegged: Address,
    ratio: u32) {
    
    let event: InitializeEvent = InitializeEvent {
        minter: minter,
        pegged: pegged,
        ratio: ratio,
    };
    e.events().publish(("GladiusCoinEmitter", symbol_short!("init")), event);
}

