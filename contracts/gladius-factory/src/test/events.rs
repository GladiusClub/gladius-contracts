use soroban_sdk::{testutils::{Events, Ledger}, vec, IntoVal, symbol_short, String};
use soroban_sdk::{xdr::{ToXdr}, Bytes}; // For determinisitic address
use crate::test::{GladiusFactoryTest};
use crate::event::{
    InitializedEvent,
    NewPremiumClubEvent};


#[test]
fn initialized_event() {
    let test = GladiusFactoryTest::setup();
    
    let init_time: u64 = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });
    
    test.contract.initialize(
        &test.coin_emitter_wasm,
        &test.subscriptions_wasm,
        &test.nft_wasm);
    
    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        ledger_timestamp: init_time.clone()
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        ledger_timestamp: 0,
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("init")).into_val(&test.env),
                (false_initialized_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("GladiusFactoryY", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn new_premium_club_event() {
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

    let premium_club_addresses = test.contract.create_premium_club(
        &test.admin,
        &sport_club_name,
        &pegged,
        &ratio,
        &nft_token_name,
        &nft_symbol
    );

    let new_premium_club_event = test.env.events().all().last().unwrap();

    let expected_new_premium_club_event: NewPremiumClubEvent = NewPremiumClubEvent {
        admin: test.admin.clone(), 
        sport_club_name: sport_club_name.clone(),
        pegged: pegged.clone(),
        ratio: ratio.clone(),
        nft_token_name: nft_token_name.clone(),
        nft_symbol: nft_symbol.clone(),
        coin_emitter_address: premium_club_addresses.0.clone(),
        subscriptions_address: premium_club_addresses.1.clone(),
        nft_address: premium_club_addresses.2.clone(),
        new_clubs_length: 1
    };

    assert_eq!(
        vec![&test.env, new_premium_club_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("new_club")).into_val(&test.env),
                (expected_new_premium_club_event).into_val(&test.env)
            ),
        ]
    );

    let false_new_premium_club_event: NewPremiumClubEvent = NewPremiumClubEvent {
        admin: test.admin.clone(), 
        sport_club_name: sport_club_name.clone(),
        pegged: pegged.clone(),
        ratio: ratio.clone(),
        nft_token_name: nft_token_name.clone(),
        nft_symbol: nft_symbol.clone(),
        coin_emitter_address: premium_club_addresses.0.clone(),
        subscriptions_address: premium_club_addresses.1.clone(),
        nft_address: premium_club_addresses.2.clone(),
        new_clubs_length: 0 // CHANGED
    };

    assert_ne!(
        vec![&test.env, new_premium_club_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("new_club")).into_val(&test.env),
                (false_new_premium_club_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, new_premium_club_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactory", symbol_short!("new_clubb")).into_val(&test.env),
                (expected_new_premium_club_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, new_premium_club_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusFactoryy", symbol_short!("new_club")).into_val(&test.env),
                (expected_new_premium_club_event).into_val(&test.env)
            ),
        ]
    );

}
