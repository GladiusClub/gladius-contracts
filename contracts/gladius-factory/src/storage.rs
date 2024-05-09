use soroban_sdk::{
    contracttype, Address, BytesN, Env, Val, TryFromVal
};

// use soroswap_factory_interface::{FactoryError};
use crate::error::{GladiusFactoryError};
use crate::premium_club::{PremiumClub, PremiumClubAddresses};


const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
const PERSISTENT_BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    CoinEmitterWasmHash, // BytesN<32>. 
    NFTWasmHash, // BytesN<32>. 
    SubscriptionsWasmHash, // BytesN<32>. 

    // FeeTo,      // Address. Instance storage
    // FeeToSetter, // Address. Instance storage
    // FeesEnabled, // Bool. Instance storage
    TotalPairs, // Total pairs created by the Factory. u32, Instance storage
    ContractsAddressesNIndexed(u32), // Addresses of contracts created by the Factory. Persistent Storage
    ContractsAddressesByPremiumClub(PremiumClub)
}

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_instance_extend_or_error<V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &DataKey,
    error: GladiusFactoryError
) -> Result<V, GladiusFactoryError> {
    if let Some(result) = e.storage().instance().get(key) {
        extend_instance_ttl(&e);
        result
    } else {
        return Err(error);
    }
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_persistent_extend_or_error<V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &DataKey,
    error: GladiusFactoryError
) -> Result<V, GladiusFactoryError> {
    if let Some(result) = e.storage().persistent().get(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        result
    } else {
        return Err(error);
    }
}


// PUT FUNCTIONS

pub fn put_coin_emitter_wasm_hash(e: &Env, coin_emitter_wasm_hash: BytesN<32>) {
    let key = DataKey::CoinEmitterWasmHash;
    e.storage().instance().set(&key, &coin_emitter_wasm_hash);
    extend_instance_ttl(&e);
}

pub fn put_nft_wasm_hash(e: &Env, nft_wasm_hash: BytesN<32>) {
    let key = DataKey::CoinEmitterWasmHash;
    e.storage().instance().set(&key, &nft_wasm_hash);
    extend_instance_ttl(&e);
}

pub fn put_subscriptions_wasm_hash(e: &Env, subscriptions_wasm_hash: BytesN<32>) {
    let key = DataKey::CoinEmitterWasmHash;
    e.storage().instance().set(&key, &subscriptions_wasm_hash);
    extend_instance_ttl(&e);
}

// GET FUNCTIONS

pub fn get_coin_emitter_wasm_hash(e: &Env) -> Result<BytesN<32>, GladiusFactoryError>{
    let key = DataKey::CoinEmitterWasmHash;
    get_instance_extend_or_error(&e, &key, GladiusFactoryError::NotInitialized)
}

pub fn get_nft_wasm_hash(e: &Env) -> Result<BytesN<32>, GladiusFactoryError>{
    let key = DataKey::NFTWasmHash;
    get_instance_extend_or_error(&e, &key, GladiusFactoryError::NotInitialized)
}

pub fn get_subscriptions_wasm_hash(e: &Env) -> Result<BytesN<32>, GladiusFactoryError>{
    let key = DataKey::SubscriptionsWasmHash;
    get_instance_extend_or_error(&e, &key, GladiusFactoryError::NotInitialized)
}


// //// --- Storage helper functions ---

// Total Premuim Clubs
pub fn put_total_premium_clubs(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalPairs, &n);
}
pub fn get_total_premium_clubs(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalPairs).unwrap_or(0)
}
// Helper function in order to know if the contract has been initialized or not
pub fn has_total_premium_clubs(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::TotalPairs)
}


// // ContractsAddressesByPremiumClub(Address, Address)
pub fn put_contracts_addresses_by_premium_club(
    e: &Env,
    premium_club: PremiumClub,
    addresses: (&Address, &Address, &Address)) {
    let key = DataKey::ContractsAddressesByPremiumClub(premium_club); 

    e.storage()
        .persistent()
        .set(&key, &addresses);

    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT)
}
pub fn get_contracts_addresses_by_premium_club(
    e: &Env,
    premium_club: PremiumClub
) -> Result<(Address, Address, Address), GladiusFactoryError> {
    let key = DataKey::ContractsAddressesByPremiumClub(premium_club);
    get_persistent_extend_or_error(&e, &key, GladiusFactoryError::PremiumClubDoesNotExist)
}

pub fn get_premium_club_exists(e: &Env, premium_club: PremiumClub) -> bool {
    let key:DataKey = DataKey::ContractsAddressesByPremiumClub(premium_club);
    if e.storage().persistent().has(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        true
    } else {
        false
    } 
}

pub fn add_addresses_to_all_addresses(
    e: &Env,
    addresses: (&Address, &Address, &Address)) {
    // total_premium_clubs is the total amount of pairs created by the Factory
    let mut total_premium_clubs = get_total_premium_clubs(e);
    // Because ContractsAddressesNIndexed is 0-indexed, we start with 0, default value of total_premium_clubs

    let key = DataKey::ContractsAddressesNIndexed(total_premium_clubs);
    e.storage().persistent().set(&key, &addresses);
    
    e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);

    total_premium_clubs = total_premium_clubs.checked_add(1).unwrap();
    put_total_premium_clubs(&e, total_premium_clubs);
}

// pub fn get_all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
//     let key = DataKey::PairAddressesNIndexed(n);
//     get_persistent_extend_or_error(&e, &key, FactoryError::IndexDoesNotExist)
// }