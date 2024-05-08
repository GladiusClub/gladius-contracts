#![no_std]
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, vec,
    Address, Env, IntoVal, Symbol, String, BytesN};
use soroban_sdk::token::Client as TokenClient;

// Import modules
mod create_contract;
mod premium_club;
mod storage;
mod error;
mod coin_emitter;
mod subscriptions;
mod nft;

use storage::*;
use premium_club::{create_contract, PremiumClub, PremiumClubAddresses};
use error::GladiusFactoryError;
// use create_contract::create_contract;

pub trait GladiusFactoryTrait {

    fn all_premium_clubs_length(e: Env) -> Result<u32, GladiusFactoryError>;

    fn initialize(
        e:                          Env,
        coin_emitter_hash:  BytesN<32>,
        nft_hash:           BytesN<32>,
        subscriptions_hash: BytesN<32>
    ) -> Result<(), GladiusFactoryError> ;

    fn create_premium_club(
        e: Env,
        admin: Address, 
        sport_club_name: String,
        pegged: Address,
        ratio: u32,
        nft_token_name: String,
        nft_symbol: String,
    ) -> Result<(Address, Address, Address), GladiusFactoryError> ;

}

#[contract]
struct GladiusFactory;

#[contractimpl]
impl GladiusFactoryTrait for GladiusFactory {

fn all_premium_clubs_length(e: Env) -> Result<u32, GladiusFactoryError> {
    if !has_total_premium_clubs(&e) {
        return Err(GladiusFactoryError::NotInitialized);
    }
    extend_instance_ttl(&e);
    Ok(get_total_premium_clubs(&e))
}

// fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
//     if !has_total_premium_clubs(&e) {
//         return Err(FactoryError::NotInitialized);
//     }
//     extend_instance_ttl(&e);
//     let token_pair = Pair::new(token_a, token_b)?;
//     get_pair_address_by_token_pair(&e, token_pair)
// }

// /// Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
// /// 
// /// # Arguments
// /// 
// /// * `e` - An instance of the `Env` struct.
// /// * `n` - The index of the pair to retrieve.
// /// 
// /// # Errors
// /// 
// /// Returns an error if the Factory is not yet initialized or if index `n` does not exist.
// fn all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
//     if !has_total_premium_clubs(&e) {
//         return Err(FactoryError::NotInitialized);
//     }
//     extend_instance_ttl(&e);
//     get_all_pairs(e,n)
// }

// /// Checks if a pair exists for the given `token_a` and `token_b`.
// /// 
// /// # Arguments
// /// 
// /// * `e` - An instance of the `Env` struct.
// /// * `token_a` - The address of the first token in the pair.
// /// * `token_b` - The address of the second token in the pair.
// /// 
// /// # Errors
// /// 
// /// Returns an error if the Factory is not yet initialized.
// fn pair_exists(e: Env, token_a: Address, token_b: Address) -> Result<bool, FactoryError> {
//     if !has_total_premium_clubs(&e) {
//         return Err(FactoryError::NotInitialized);
//     }
//     extend_instance_ttl(&e);

//     let token_pair = Pair::new(token_a, token_b)?;
    
//     // Proceed with the existence check
//     Ok(get_pair_exists(&e, token_pair))
// }


/* *** State-Changing Functions: *** */

fn initialize(
    e:                          Env,
    coin_emitter_wasm_hash:  BytesN<32>,
    nft_wasm_hash:           BytesN<32>,
    subscriptions_wasm_hash: BytesN<32>
)  -> Result<(), GladiusFactoryError> {

    if has_total_premium_clubs(&e) {
        return Err(GladiusFactoryError::InitializeAlreadyInitialized);
    }
    put_coin_emitter_wasm_hash(&e, coin_emitter_wasm_hash);
    put_nft_wasm_hash(&e, nft_wasm_hash);
    put_subscriptions_wasm_hash(&e, subscriptions_wasm_hash);
    put_total_premium_clubs(&e, 0);
    // event::initialized(&e, setter);
    extend_instance_ttl(&e);
    Ok(())
}

fn create_premium_club(
    e: Env,
    admin: Address, 
    sport_club_name: String,
    pegged: Address,
    ratio: u32,
    nft_token_name: String,
    nft_symbol: String,
// ) -> Result<Address, FactoryError> {
) -> Result<(Address, Address, Address), GladiusFactoryError>  {
    if !has_total_premium_clubs(&e) {
        return Err(GladiusFactoryError::NotInitialized);
    }

    extend_instance_ttl(&e);
    
    let premium_club: PremiumClub = PremiumClub(admin.clone(), sport_club_name.clone());

    // Install and Deploy Contracts
    let coin_emitter_address = create_contract(
        &e,
        get_coin_emitter_wasm_hash(&e).unwrap(),
        &premium_club   
    );

    let subscriptions_address = create_contract(
        &e,
        get_subscriptions_wasm_hash(&e).unwrap(),
        &premium_club   
    );

    let nft_address = create_contract(
        &e,
        get_nft_wasm_hash(&e).unwrap(),
        &premium_club   
    );

    let premium_club_addresses: PremiumClubAddresses = PremiumClubAddresses (
        coin_emitter_address.clone(),
        subscriptions_address.clone(),
        nft_address.clone()
    );

    // Initialize Contracts
    coin_emitter::Client::new(&e, &coin_emitter_address).initialize(
        &admin, // Address, 
        &pegged, // Address,
        &ratio // u32
    );

    subscriptions::Client::new(&e, &subscriptions_address).initialize(
        &admin, // admin: Address,
        &pegged, // token: Address,
        &coin_emitter_address, //gladius_coin_emitter: Address
    );

    nft::Client::new(&e, &nft_address).initialize(
        &admin, // admin: Address,
        &nft_token_name, //     name: String
        &nft_symbol, //     symbol: String
    );
    put_contracts_addresses_by_premium_club(
        &e,
        premium_club,
        (&coin_emitter_address,
            &subscriptions_address,
            &nft_address)
    );
    // add_pair_to_all_pairs(&e, &pair_address);

    // event::new_pair(&e, token_pair.token_0().clone(), token_pair.token_1().clone(), pair_address.clone(), get_total_pairs(&e));

    // Ok(pair_address)
    Ok((coin_emitter_address,
        nft_address,
        subscriptions_address))
}


}
