use soroban_sdk::{String};
use crate::test::{GladiusNFTTest};
use crate::event::{InitializeEvent};

use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};


#[test]
fn initialize_basic_info() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    assert_eq!(test.contract.name(), name);
    assert_eq!(test.contract.symbol(), symbol);
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.admin(), test.admin);
    let initialize_event = test.env.events().all().last().unwrap();

    let expected_initialize_event: InitializeEvent = InitializeEvent {
        admin: test.admin.clone(),
        name: name.clone(),
        symbol: symbol.clone(),
    };

    assert_eq!(
        vec![&test.env, initialize_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusNFT", symbol_short!("init")).into_val(&test.env),
                (expected_initialize_event).into_val(&test.env)
            ),
        ]
    );

}

#[test]
#[should_panic] // TODO: Transform to error
fn initialize_twice() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    
    // let res = test.contract.try_initialize(
    //     &test.minter,
    //     &test.pegged_token.address,
    //     &ratio
    //     );
    // assert_eq!(res, Err(Ok(GladiusCoinEmitterError::InitializeAlreadyInitialized))); 
}


