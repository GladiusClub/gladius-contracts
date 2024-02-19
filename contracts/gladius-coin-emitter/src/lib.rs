#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String}; 
use soroban_sdk::token::Client as TokenClient;

use soroban_token_sdk::metadata::TokenMetadata;



mod gladius_coin;
mod storage_types;  
mod test;

use gladius_coin::{write_metadata};
use gladius_coin::{read_administrator, has_administrator, write_administrator};
use gladius_coin::{receive_balance, spend_balance};
use soroban_token_sdk::TokenUtils;

use storage_types::{GladiusDataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD}; 


pub fn read_pegged_token(e: &Env) -> Address {
    let key = GladiusDataKey::PeggedToken;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_pegged_token(e: &Env, id: &Address) {
    let key = GladiusDataKey::PeggedToken;
    e.storage().instance().set(&key, id);
}


pub fn read_ratio(e: &Env) -> u32 {
    let key = GladiusDataKey::Ratio;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_ratio(e: &Env, id: &u32) {
    let key = GladiusDataKey::Ratio;
    e.storage().instance().set(&key, id);
}


fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}


pub fn read_reserve(e: &Env) -> i128 {
    e.storage().instance().
get(&GladiusDataKey::Reserve).unwrap()
}

pub fn write_reserve(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve: amount cannot be negative")
    }
    e.storage().instance().
set(&GladiusDataKey::Reserve, &amount)
}

pub trait GladiusCoinEmitterTrait {

    fn initialize_gladius(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32);

    fn wrap_and_mint(e: Env, to: Address, amount: i128);

    fn unwrap_and_burn(e: Env, from: Address, amount: i128);

    fn ratio(e: Env)  -> u32;

    fn pegged(e: Env)  -> Address;

    fn minter(e: Env)  -> Address;


}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {

    fn initialize_gladius(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32) {
            
            let name = String::from_str(&e, "Gladius Coin");
            let symbol = String::from_str(&e, "GLC");
            let decimal = 7;

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
            );

            write_pegged_token(&e, &pegged);
            write_ratio(&e, &ratio);
    }

    // Receives a pegged_amount of pegged_token and mints a ratio*pegged_amount units of gladius coins
    // Wraps a pegged_amount and mints
    fn wrap_and_mint(e: Env, to: Address, pegged_amount: i128) {
        check_nonnegative_amount(pegged_amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Send peggued token from minter to this contract that will lock it
        TokenClient::new(&e, &read_pegged_token(&e)).transfer(&admin, &e.current_contract_address(), &pegged_amount);

        // Amount to mint of Gladius Coins is ratio*pegged_amount
        let amount = pegged_amount.checked_mul(read_ratio(&e) as i128).unwrap();
        // Mint amount to user
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    fn unwrap_and_burn(e: Env, from: Address, pegged_amount: i128) {
        from.require_auth();

        check_nonnegative_amount(pegged_amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Amount to mint of Gladius Coins is ratio*pegged_amount
        let amount = pegged_amount.checked_mul(read_ratio(&e) as i128).unwrap();

        // Burn Gladius Coins of user
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from.clone(), amount);

        // Send back pegged_amount units of pegged token
        TokenClient::new(&e, &read_pegged_token(&e)).transfer(&e.current_contract_address(), &from, &pegged_amount);
    }

    fn ratio(e: Env) -> u32 {
        read_ratio(&e)
    }

    fn pegged(e: Env) -> Address {
        read_pegged_token(&e)
    }

    fn minter(e: Env) -> Address {
        read_administrator(&e)
    }
}
