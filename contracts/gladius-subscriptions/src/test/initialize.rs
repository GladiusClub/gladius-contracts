use soroban_sdk::{String};
use crate::test::{GladiusSubscriptionsTest}; 
use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};


#[test]
fn basic_info_before_initialize() {
    let test = GladiusSubscriptionsTest::setup();
    
    assert_eq!(test.payment_token.name(), String::from_str(&test.env, "EURC TOKEN"));
    assert_eq!(test.payment_token.symbol(), String::from_str(&test.env, "EURC"));
    assert_eq!(test.payment_token.decimals(), 7);

    assert_eq!(test.gladius_coin_emitter.name(), String::from_str(&test.env, "Gladius Coin"));
    assert_eq!(test.gladius_coin_emitter.symbol(), String::from_str(&test.env, "GLC"));
    assert_eq!(test.gladius_coin_emitter.decimals(), 7);
    assert_eq!(test.gladius_coin_emitter.pegged(), test.payment_token.address);
    assert_eq!(test.gladius_coin_emitter.ratio(), 1000);
    assert_eq!(test.gladius_coin_emitter.minter(), test.contract.address);
}

#[test]
fn initialize_basic_info() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    // TODO: test admin
    assert_eq!(test.contract.get_admin(), test.gladius_admin);
    assert_eq!(test.contract.get_token(), test.payment_token.address);
    assert_eq!(test.contract.get_gladius_coin_emitter(), test.gladius_coin_emitter.address);
}


// #[test]
// fn initialize_twice() {
//     let test = GladiusCoinEmitterTest::setup();

//     let ratio: u32 = 1000;

//     test.contract.initialize(
//         &test.minter,
//         &test.pegged_token.address,
//         &ratio
//         );
    
//     let res = test.contract.try_initialize(
//         &test.minter,
//         &test.pegged_token.address,
//         &ratio
//         );
//     assert_eq!(res, Err(Ok(GladiusCoinEmitterError::InitializeAlreadyInitialized))); 
// }


