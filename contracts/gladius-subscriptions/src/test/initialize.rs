use soroban_sdk::{testutils::{Events}, vec, symbol_short, IntoVal, String};
use crate::test::{GladiusSubscriptionsTest}; 
use crate::test::gladius_subscriptions::GladiusSubscriptionsError;
use crate::event::{InitializeEvent};

// use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};


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

    assert_eq!(test.contract.get_total_courses(), 0);
}

#[test]
fn initialize_basic_info() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    let initialize_event = test.env.events().all().last().unwrap();

    let expected_initialize_event: InitializeEvent = InitializeEvent {
        admin: test.gladius_admin.clone(),
        payment_token: test.payment_token.address.clone(),
        gladius_coin_emitter: test.gladius_coin_emitter.address.clone(),
    };

    assert_eq!(
        vec![&test.env, initialize_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusSubscriptions", symbol_short!("init")).into_val(&test.env),
                (expected_initialize_event).into_val(&test.env)
            ),
        ]
    );
    
    // TODO: test admin
    assert_eq!(test.contract.get_admin(), test.gladius_admin);
    assert_eq!(test.contract.get_token(), test.payment_token.address);
    assert_eq!(test.contract.get_gladius_coin_emitter(), test.gladius_coin_emitter.address);
    assert_eq!(test.contract.get_total_courses(), 0);
}


#[test]
fn initialize_twice() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    let res = test.contract.try_initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::AlreadyInitialized))); 
}


