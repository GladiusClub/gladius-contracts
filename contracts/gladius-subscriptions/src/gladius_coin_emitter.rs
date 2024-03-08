use soroban_sdk::{Env, Address, contractimport};
use crate::storage_types::SubsDataKey;

contractimport!(
    file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.optimized.wasm"
);
pub type GladiusCoinEmitterClient<'a> = Client<'a>;

pub fn read_gladius_coin_emitter(e: &Env) -> Address {
    let key = SubsDataKey::GladiusCoinEmitter;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_gladius_coin_emitter(e: &Env, id: &Address) {
    let key = SubsDataKey::GladiusCoinEmitter;
    e.storage().instance().set(&key, id);
}