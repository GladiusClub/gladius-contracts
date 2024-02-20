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



// WRAP EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WrapEvent {
    pub minter: Address,
    pub wrapped_amount: i128,
    pub minted_amount: i128,
    pub to: Address 
}

pub(crate) fn wrap(
    e: &Env, 
    minter: Address,
    wrapped_amount: i128,
    minted_amount: i128,
    to: Address) {
    
    let event: WrapEvent = WrapEvent {
        minter: minter,
        wrapped_amount: wrapped_amount,
        minted_amount: minted_amount,
        to: to
    };
    e.events().publish(("GladiusCoinEmitter", symbol_short!("wrap")), event);
}

