#![no_std]
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, vec,
    Address, Env, IntoVal, Symbol, String, BytesN};
use soroban_sdk::token::Client as TokenClient;

// Import modules
mod create_contract;
mod storage;

use storage::*;
use create_contract::create_contract;

pub trait GladiusFactoryTrait {
    fn initialize(
        e:                          Env,
        coin_emitter_hash:  BytesN<32>,
        nft_hash:           BytesN<32>,
        subscriptions_hash: BytesN<32>
    );

}

#[contract]
struct GladiusFactory;

#[contractimpl]
impl GladiusFactoryTrait for GladiusFactory {

// /// Returns the total number of pairs created through the factory so far.
// /// 
// /// # Arguments
// /// 
// /// * `e` - An instance of the `Env` struct.
// /// 
// /// # Errors
// /// 
// /// Returns an error if the Factory is not yet initialized.
// fn all_pairs_length(e: Env) -> Result<u32, FactoryError> {
//     if !has_total_pairs(&e) {
//         return Err(FactoryError::NotInitialized);
//     }
//     extend_instance_ttl(&e);
//     Ok(get_total_pairs(&e))
// }

// /// Returns the address of the pair for `token_a` and `token_b`, if it has been created.
// /// 
// /// # Arguments
// /// 
// /// * `e` - An instance of the `Env` struct.
// /// * `token_a` - The address of the first token in the pair.
// /// * `token_b` - The address of the second token in the pair.
// /// 
// /// # Errors
// /// 
// /// Returns an error if the Factory is not yet initialized or if the pair does not exist
// fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
//     if !has_total_pairs(&e) {
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
//     if !has_total_pairs(&e) {
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
//     if !has_total_pairs(&e) {
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
)  {

    // if has_total_pairs(&e) {
    //     return Err(FactoryError::InitializeAlreadyInitialized);
    // }
    put_coin_emitter_wasm_hash(&e, coin_emitter_wasm_hash);
    put_nft_wasm_hash(&e, nft_wasm_hash);
    put_subscriptions_wasm_hash(&e, subscriptions_wasm_hash);
    // put_total_pairs(&e, 0);
    // event::initialized(&e, setter);
    extend_instance_ttl(&e);
    // Ok(())
}

// fn create_premium_club(
//     e: Env,
//     sport_club_address: Address,
//     sport_club_name: String
// // ) -> Result<Address, FactoryError> {
// ) {
//     // if !has_total_pairs(&e) {
//     //     return Err(FactoryError::NotInitialized);
//     // }

//     extend_instance_ttl(&e);
//     let coin_emitter = CoinEmitter::new(sport_club_address, sport_club_name)?;

//     // if get_pair_exists(&e, token_pair.clone()) {
//     //     return Err(FactoryError::CreatePairAlreadyExists);
//     // }

//     // let pair_wasm_hash = get_pair_wasm_hash(&e)?;
//     // let pair_address = create_contract(&e, pair_wasm_hash, &token_pair);

//     // pair::Client::new(&e, &pair_address).initialize(
//     //     &e.current_contract_address(),
//     //     &token_pair.token_0(), 
//     //     &token_pair.token_1()
//     // );

//     // put_pair_address_by_token_pair(&e, token_pair.clone(), &pair_address);
//     // add_pair_to_all_pairs(&e, &pair_address);

//     // event::new_pair(&e, token_pair.token_0().clone(), token_pair.token_1().clone(), pair_address.clone(), get_total_pairs(&e));

//     // Ok(pair_address)
// }


}
