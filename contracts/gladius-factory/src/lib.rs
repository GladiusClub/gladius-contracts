#![no_std]
use soroban_sdk::{
    contract, contractimpl,
    Address, Env, String, BytesN};

// Import modules
mod create_contract;
mod premium_club;
mod storage;
mod error;
mod coin_emitter;
mod subscriptions;
mod nft;
mod event;
mod test;

use storage::*;
use premium_club::{create_contract, PremiumClub, PremiumClubAddresses};
use error::GladiusFactoryError;

pub trait GladiusFactoryTrait {

    fn all_premium_clubs_length(e: Env) -> Result<u32, GladiusFactoryError>;

    fn get_premium_club_addresses(
        e: Env, 
        admin: Address, 
        sport_club_name: String
    )-> Result<PremiumClubAddresses, GladiusFactoryError>;

    fn premium_club_exist(
        e: Env,
        admin: Address, 
        sport_club_name: String
    ) -> Result<bool, GladiusFactoryError>;

    fn all_addresses(
        e: Env, 
        n: u32
    ) -> Result<PremiumClubAddresses, GladiusFactoryError>;
    
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
    ) -> Result<PremiumClubAddresses, GladiusFactoryError> ;

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

    fn get_premium_club_addresses(
        e: Env, 
        admin: Address, 
        sport_club_name: String
    ) -> Result<PremiumClubAddresses, GladiusFactoryError> {
        if !has_total_premium_clubs(&e) {
            return Err(GladiusFactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        let premium_club: PremiumClub = PremiumClub::new(admin, sport_club_name);
        let (coin_emitter,
            subscriptions,
            nft): (Address, Address, Address) = get_contracts_addresses_by_premium_club(&e, premium_club)?;
        
        Ok(PremiumClubAddresses::new(coin_emitter,
            subscriptions,
            nft))
    }

    fn all_addresses(e: Env, n: u32) -> Result<PremiumClubAddresses, GladiusFactoryError> {
        if !has_total_premium_clubs(&e) {
            return Err(GladiusFactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        let (coin_emitter,
            subscriptions,
            nft): (Address, Address, Address) = get_all_addresses(e,n)?;
        
        Ok(PremiumClubAddresses::new(coin_emitter,
            subscriptions,
            nft))    
    }


    fn premium_club_exist(
        e: Env,
        admin: Address, 
        sport_club_name: String
    ) -> Result<bool, GladiusFactoryError> {
        if !has_total_premium_clubs(&e) {
            return Err(GladiusFactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);

        let premium_club: PremiumClub = PremiumClub::new(admin, sport_club_name);
        
        // Proceed with the existence check
        Ok(get_premium_club_exists(&e, premium_club))
    }


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
 
        event::initialized(&e, e.ledger().timestamp());

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
    ) -> Result<PremiumClubAddresses, GladiusFactoryError>  {
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
        add_addresses_to_all_addresses(
            &e, 
            (&coin_emitter_address,
            &subscriptions_address,
            &nft_address));

        event::new_club(
            &e,
            admin, 
            sport_club_name,
            pegged,
            ratio,
            nft_token_name,
            nft_symbol,
            coin_emitter_address,
            subscriptions_address,
            nft_address,
            get_total_premium_clubs(&e));

        Ok(premium_club_addresses)
    }
}
