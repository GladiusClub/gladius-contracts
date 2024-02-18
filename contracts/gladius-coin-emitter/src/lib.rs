#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String}; 
use soroban_token_sdk::metadata::TokenMetadata;


mod gladius_coin;

use gladius_coin::{write_metadata};
use gladius_coin::{has_administrator, write_administrator};



pub trait GladiusCoinEmitterTrait {

    fn initialize_gladius(e: Env, admin: Address, decimal: u32, name: String, symbol: String);


}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {

    fn initialize_gladius(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
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
