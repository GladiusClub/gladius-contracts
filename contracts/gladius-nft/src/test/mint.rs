use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};


#[test]
fn mint() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    assert_eq!(test.contract.balance_of(&test.user), 0);
    
}
