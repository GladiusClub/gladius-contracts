#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String}; 
use soroban_token_sdk::metadata::TokenMetadata;



mod gladius_coin;
mod storage_types;  

use gladius_coin::{write_metadata};
use gladius_coin::{has_administrator, write_administrator};
use storage_types::GladiusDataKey;


pub fn read_pegged_token(e: &Env) -> Address {
    let key = GladiusDataKey::PeggedToken;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_pegged_token(e: &Env, id: &Address) {
    let key = GladiusDataKey::PeggedToken;
    e.storage().instance().set(&key, id);
}


pub trait GladiusCoinEmitterTrait {

    fn initialize_gladius(e: Env, admin: Address, decimal: u32, name: String, symbol: String, pegged: Address);


}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {

    fn initialize_gladius(e: Env, admin: Address, decimal: u32, name: String, symbol: String, pegged: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }
}
