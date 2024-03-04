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

    /// Initializes the Gladius Coin smart contract with provided parameters.
    ///
    /// This function sets up the Gladius Coin smart contract by initializing its metadata, including
    /// name, symbol, and decimal places. It also sets the administrator, pegged token address, and
    /// the ratio. If the contract has already been initialized, it returns an error indicating that
    /// initialization has already occurred.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `admin` - The address of the administrator to be set.
    /// * `pegged` - The address of the pegged token.
    /// * `ratio` - The ratio of conversion between Gladius Coin and the pegged token.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the initialization is successful, otherwise returns an error of type
    /// `GladiusCoinEmitterError`.
    fn initialize(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32) -> Result<(), GladiusCoinEmitterError>;

    /// Wraps and mints Gladius Coins for the specified recipient.
    ///
    /// This function wraps a specified amount of the pegged token into Gladius Coins and mints them
    /// for the designated recipient. It first checks if the contract has been initialized and ensures
    /// that the wrap amount is non-negative. It then transfers the specified amount of the pegged
    /// token from the minter to the contract, locking it. After that, it calculates the amount of Gladius
    /// Coins to mint based on the wrapping ratio and mints them for the recipient. Finally, it emits
    /// a `wrap` event to notify listeners about the wrapping and minting operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `to` - The address of the recipient to whom the minted Gladius Coins will be sent.
    /// * `wrap_amount` - The amount of the pegged token to wrap and mint into Gladius Coins.
    ///
    /// # Returns
    ///
    /// Returns `Ok((mint_amount))` if the wrapping and minting operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
    fn wrap_and_mint(
        e: Env, 
        to: Address, 
        amount: i128) -> Result<i128, GladiusCoinEmitterError>;

    /// Unwraps and burns Gladius Coins, converting them back to the pegged token.
    ///
    /// This function unwraps a specified amount of Gladius Coins, converting them back to the
    /// pegged token, and burns them. It first checks if the contract has been initialized and ensures
    /// that the unwrap amount is non-negative. It then verifies the caller's authentication and sends
    /// back the unwrapped amount of the pegged token to the caller. After that, it calculates the
    /// amount of Gladius Coins to burn based on the wrapping ratio and burns them. Finally, it emits
    /// an `unwrap` event to notify listeners about the unwrapping and burning operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `from` - The address of the sender who wants to unwrap and burn Gladius Coins.
    /// * `unwrap_amount` - The amount of Gladius Coins to unwrap and burn.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the unwrapping and burning operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
    fn unwrap_and_burn(
        e: Env, 
        from: Address, 
        amount: i128) -> Result<(), GladiusCoinEmitterError>;

    /// Retrieves the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// This function reads and returns the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the current wrapping ratio as a `u32`.
    fn ratio(e: Env)  -> u32;

    /// Retrieves the address of the pegged token.
    ///
    /// This function reads and returns the address of the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the pegged token as an `Address`.
    fn pegged(e: Env)  -> Address;

    /// Retrieves the address of the minter (administrator) of Gladius Coins.
    ///
    /// This function reads and returns the address of the minter (administrator) of Gladius Coins.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the minter (administrator) as an `Address`.
    fn minter(e: Env)  -> Address;

}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {

    /// Initializes the Gladius Coin smart contract with provided parameters.
    ///
    /// This function sets up the Gladius Coin smart contract by initializing its metadata, including
    /// name, symbol, and decimal places. It also sets the administrator, pegged token address, and
    /// the ratio. If the contract has already been initialized, it returns an error indicating that
    /// initialization has already occurred.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `admin` - The address of the administrator to be set.
    /// * `pegged` - The address of the pegged token.
    /// * `ratio` - The ratio of conversion between Gladius Coin and the pegged token.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the initialization is successful, otherwise returns an error of type
    /// `GladiusCoinEmitterError`.

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

    /// Wraps and mints Gladius Coins for the specified recipient.
    ///
    /// This function wraps a specified amount of the pegged token into Gladius Coins and mints them
    /// for the designated recipient. It first checks if the contract has been initialized and ensures
    /// that the wrap amount is non-negative. It then transfers the specified amount of the pegged
    /// token from the minter to the contract, locking it. After that, it calculates the amount of Gladius
    /// Coins to mint based on the wrapping ratio and mints them for the recipient. Finally, it emits
    /// a `wrap` event to notify listeners about the wrapping and minting operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `to` - The address of the recipient to whom the minted Gladius Coins will be sent.
    /// * `wrap_amount` - The amount of the pegged token to wrap and mint into Gladius Coins.
    ///
    /// # Returns
    ///
    /// Returns `Ok((mint_amount))` if the wrapping and minting operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
    fn wrap_and_mint(
        e: Env,
        to: Address,
        wrap_amount: i128) -> Result<i128, GladiusCoinEmitterError> {

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
        Ok(mint_amount)
    }

    /// Unwraps and burns Gladius Coins, converting them back to the pegged token.
    ///
    /// This function unwraps a specified amount of Gladius Coins, converting them back to the
    /// pegged token, and burns them. It first checks if the contract has been initialized and ensures
    /// that the unwrap amount is non-negative. It then verifies the caller's authentication and sends
    /// back the unwrapped amount of the pegged token to the caller. After that, it calculates the
    /// amount of Gladius Coins to burn based on the wrapping ratio and burns them. Finally, it emits
    /// an `unwrap` event to notify listeners about the unwrapping and burning operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `from` - The address of the sender who wants to unwrap and burn Gladius Coins.
    /// * `unwrap_amount` - The amount of Gladius Coins to unwrap and burn.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the unwrapping and burning operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
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

    /// Retrieves the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// This function reads and returns the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the current wrapping ratio as a `u32`.
    fn ratio(e: Env) -> u32 {
        read_ratio(&e)
    }

    /// Retrieves the address of the pegged token.
    ///
    /// This function reads and returns the address of the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the pegged token as an `Address`.
    fn pegged(e: Env) -> Address {
        read_pegged_token(&e)
    }

    /// Retrieves the address of the minter (administrator) of Gladius Coins.
    ///
    /// This function reads and returns the address of the minter (administrator) of Gladius Coins.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the minter (administrator) as an `Address`.
    fn minter(e: Env) -> Address {
        read_administrator(&e)
    }

}
