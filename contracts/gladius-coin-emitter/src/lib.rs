#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String}; 
use soroban_sdk::token::Client as TokenClient;
use soroban_token_sdk::metadata::TokenMetadata;

mod gladius_coin;
mod storage_types;  
mod test;
mod error;
mod event;

use gladius_coin::{write_metadata};
use gladius_coin::{read_administrator, has_administrator, write_administrator};
use gladius_coin::{internal_mint, internal_burn};
use error::GladiusCoinEmitterError;
use storage_types::{GladiusDataKey}; 

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

pub trait GladiusCoinEmitterTrait {

    fn initialize(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32) -> Result<(), GladiusCoinEmitterError>;

    fn wrap_and_mint(
        e: Env, 
        to: Address, 
        amount: i128) -> Result<(), GladiusCoinEmitterError>;

    fn unwrap_and_burn(
        e: Env, 
        from: Address, 
        amount: i128) -> Result<(), GladiusCoinEmitterError>;

    fn ratio(e: Env)  -> u32;

    fn pegged(e: Env)  -> Address;

    fn minter(e: Env)  -> Address;

}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {

    fn initialize(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32) -> Result<(), GladiusCoinEmitterError> {
            
        let name = String::from_str(&e, "Gladius Coin");
        let symbol = String::from_str(&e, "GLC");
        let decimal = 7;

        if has_administrator(&e) {
            return Err(GladiusCoinEmitterError::InitializeAlreadyInitialized);
        }

        write_administrator(&e, &admin);
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

        event::initialize(&e, admin, pegged, ratio);
        Ok(())
}

    // Receives a pegged_amount of pegged_token and mints a ratio*pegged_amount units of gladius coins
    // Wraps a pegged_amount and mints
    fn wrap_and_mint(
        e: Env,
        to: Address,
        wrap_amount: i128) -> Result<(), GladiusCoinEmitterError> {

        if !has_administrator(&e) {
            return Err(GladiusCoinEmitterError::NotInitialized);
        }

        if wrap_amount < 0 {
            return Err(GladiusCoinEmitterError::WrapNegativesNotSupported);
        }

        let admin = read_administrator(&e);
        admin.require_auth();
        // Send peggued token from minter to this contract that will lock it
        TokenClient::new(&e, &read_pegged_token(&e)).transfer(&admin, &e.current_contract_address(), &wrap_amount);
        
        // Amount to mint of Gladius Coins is ratio*wrap_amount
        let mint_amount = wrap_amount.checked_mul(read_ratio(&e) as i128).unwrap();
        internal_mint(e.clone(), to.clone(), mint_amount.clone());

        event::wrap(&e, admin, wrap_amount, mint_amount, to);
        Ok(())
    }

    fn unwrap_and_burn(
        e: Env,
        from: Address,
        unwrap_amount: i128) -> Result<(), GladiusCoinEmitterError> {
        
        if !has_administrator(&e) {
            return Err(GladiusCoinEmitterError::NotInitialized);
        }

        if unwrap_amount < 0 { 
            return Err(GladiusCoinEmitterError::UnWrapNegativesNotSupported);
        }

        // TODO: Check that caller user is Sport Club
        from.require_auth();

        // Send back unwrap_amount units of pegged token
        TokenClient::new(&e, &read_pegged_token(&e)).transfer(&e.current_contract_address(), &from, &unwrap_amount);

        // Amount to burn of Gladius Coins is ratio*unwrap_amount
        let burn_amount = unwrap_amount.checked_mul(read_ratio(&e) as i128).unwrap();
        internal_burn(e.clone(), from.clone(), burn_amount.clone());

        event::unwrap(&e, from, unwrap_amount, burn_amount);
        Ok(())
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
