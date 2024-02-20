use soroban_sdk::{String};
use crate::test::{GladiusCoinEmitterTest}; 
use crate::test::gladius_coin_emitter::GladiusCoinEmitterError;


#[test]
fn initialize_basic_info() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;
    let decimals: u32 = 7;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );
    
    assert_eq!(test.contract.name(), String::from_str(&test.env, "Gladius Coin"));
    assert_eq!(test.contract.symbol(), String::from_str(&test.env, "GLC"));
    assert_eq!(test.contract.decimals(), decimals);
    assert_eq!(test.contract.pegged(), test.pegged_token.address);
    assert_eq!(test.contract.ratio(), ratio);
    assert_eq!(test.contract.minter(), test.minter);
}

#[test]
fn initialize_twice() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );
    
    let res = test.contract.try_initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );
    assert_eq!(res, Err(Ok(GladiusCoinEmitterError::InitializeAlreadyInitialized))); 
}


