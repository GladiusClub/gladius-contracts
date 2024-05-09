 //! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, String};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {}

pub(crate) fn initialized(e: &Env) {
    
    let event: InitializedEvent = InitializedEvent {};

    e.events().publish(("GladiusFactory", symbol_short!("init")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewPremiumClubEvent {
    pub admin: Address, 
    pub sport_club_name: String,
    pub pegged: Address,
    pub ratio: u32,
    pub nft_token_name: String,
    pub nft_symbol: String,
    pub coin_emitter_address: Address,
    pub subscriptions_address: Address,
    pub nft_address: Address,
    pub new_clubs_length: u32
}

pub(crate) fn new_club(
    e: &Env, 
    admin: Address, 
    sport_club_name: String,
    pegged: Address,
    ratio: u32,
    nft_token_name: String,
    nft_symbol: String,
    coin_emitter_address: Address,
    subscriptions_address: Address,
    nft_address: Address,
    new_clubs_length: u32) {
    
    let event: NewPremiumClubEvent = NewPremiumClubEvent {
        admin: admin,
        sport_club_name: sport_club_name,
        pegged: pegged,
        ratio: ratio,
        nft_token_name: nft_token_name,
        nft_symbol: nft_symbol,
        coin_emitter_address: coin_emitter_address,
        subscriptions_address: subscriptions_address,
        nft_address: nft_address,
        new_clubs_length: new_clubs_length,
    };
    e.events().publish(("GladiusFactory", symbol_short!("new_club")), event);
}

