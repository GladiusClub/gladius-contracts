extern crate std;
use crate::test::{GladiusFactoryTest};
use soroban_sdk::{
    String
};
use crate::error::{GladiusFactoryError};


#[test]
fn not_yet_initialized_all_premium_clubs_length() {
    let test = GladiusFactoryTest::setup();
    let res = test.contract.try_all_premium_clubs_length();
    assert_eq!(res, Err(Ok(GladiusFactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_get_premium_club_addresses() {
    let test = GladiusFactoryTest::setup();
    let res = test.contract.try_get_premium_club_addresses(
        &test.admin,
        &String::from_str(&test.env, "SportClubName0")
    );
    assert_eq!(res, Err(Ok(GladiusFactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_all_addresses() {
    let test = GladiusFactoryTest::setup();
    let res = test.contract.try_all_addresses(&0);
    assert_eq!(res, Err(Ok(GladiusFactoryError::NotInitialized)));
}


#[test]
fn not_yet_initialized_premium_club_exist() {
    let test = GladiusFactoryTest::setup();
    let res = test.contract.try_premium_club_exist(
        &test.admin,
        &String::from_str(&test.env, "SportClubName0")
    );
    assert_eq!(res, Err(Ok(GladiusFactoryError::NotInitialized)));
}


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
fn double_initialize_factory() {
    let test = GladiusFactoryTest::setup();
    test.contract.initialize(
        &test.coin_emitter_wasm,
        &test.subscriptions_wasm,
        &test.nft_wasm);
    let res = test.contract.try_initialize(
        &test.coin_emitter_wasm,
        &test.subscriptions_wasm,
        &test.nft_wasm);
    assert_eq!(res, Err(Ok(GladiusFactoryError::InitializeAlreadyInitialized)));
}

#[test]
fn initialize_basic_info() {
    let test = GladiusFactoryTest::setup();
    test.contract.initialize(
        &test.coin_emitter_wasm,
        &test.subscriptions_wasm,
        &test.nft_wasm);

    assert_eq!(test.contract.all_premium_clubs_length(), 0);
}

