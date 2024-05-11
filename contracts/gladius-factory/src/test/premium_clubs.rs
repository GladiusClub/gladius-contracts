extern crate std;
use crate::test::{GladiusFactoryTest};
use soroban_sdk::{
    IntoVal,
    testutils::{
        AuthorizedInvocation,
        AuthorizedFunction
    },
    Symbol,
    String
};
use crate::error::{GladiusFactoryError};



#[test]
fn not_yet_initialized_create_premium_club() {
    let test = GladiusFactoryTest::setup();
    let res = test.contract.try_create_premium_club(
        &test.admin,
        &String::from_str(&test.env, "SportClubName0"), //sport_club_name: String,
        &test.token.address, // pegged: Address,
        &0, // ratio: u32,
        &String::from_str(&test.env, "NFTName"),// nft_token_name: String,
        &String::from_str(&test.env, "NFTSymbol"),// nft_symbol: String,
    );
    assert_eq!(res, Err(Ok(GladiusFactoryError::NotInitialized)));
}


#[test]
fn create_premium_club() {
    let test = GladiusFactoryTest::setup();
    test.contract.initialize(
        &test.coin_emitter_wasm,
        &test.subscriptions_wasm,
        &test.nft_wasm);

    let sport_club_name = &String::from_str(&test.env, "SportClubName0");//sport_club_name: String,
    let pegged =  &test.token.address; // pegged: Address,
    let ratio = &10; // ratio: u32,
    let nft_token_name = &String::from_str(&test.env, "NFTName");// nft_token_name: String,
    let nft_symbol = &String::from_str(&test.env, "NFTSymbol");// nft_symbol: String,

    assert_eq!(test.contract.all_premium_clubs_length(), 0);

    let res = test.contract.create_premium_club(
        &test.admin,
        &sport_club_name,
        &pegged,
        &ratio,
        &nft_token_name,
        &nft_symbol
    );

    // assert_eq!(test.contract.all_premium_clubs_length(), 1);
}

