use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
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


    // let ratio: u32 = 1000;
    // let decimals: u32 = 7;

    // test.contract.initialize(
    //     &test.minter,
    //     &test.pegged_token.address,
    //     &ratio
    //     );

    // let initialize_event = test.env.events().all().last().unwrap();

    // let expected_initialize_event: InitializeEvent = InitializeEvent {
    //     minter: test.minter.clone(),
    //     pegged: test.pegged_token.address.clone(),
    //     ratio: ratio.clone(),
    // };

    // assert_eq!(
    //     vec![&test.env, initialize_event.clone()],
    //     vec![
    //         &test.env,
    //         (
    //             test.contract.address.clone(),
    //             ("GladiusCoinEmitter", symbol_short!("init")).into_val(&test.env),
    //             (expected_initialize_event).into_val(&test.env)
    //         ),
    //     ]
    // );
    
    // assert_eq!(test.contract.name(), String::from_str(&test.env, "Gladius Coin"));
    // assert_eq!(test.contract.symbol(), String::from_str(&test.env, "GLC"));
    // assert_eq!(test.contract.decimals(), decimals);
    // assert_eq!(test.contract.pegged(), test.pegged_token.address);
    // assert_eq!(test.contract.ratio(), ratio);
    // assert_eq!(test.contract.minter(), test.minter);
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


