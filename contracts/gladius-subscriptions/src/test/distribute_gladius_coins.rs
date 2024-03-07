use soroban_sdk::{String};
use crate::test::{GladiusSubscriptionsTest}; 
use crate::test::gladius_subscriptions::Course;
use soroban_sdk::{testutils::{Events, MockAuthInvoke, MockAuth}, vec, IntoVal, symbol_short};


#[test]
#[should_panic] // TODO: Change for errors
fn distribute_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.distribute_gladius_coins(
        &0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,

    );
}


#[test]
#[should_panic] // TODO: Change for errors
fn distribute_course_does_not_exist() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    test.contract.distribute_gladius_coins(
        &0, // index
        &test.student_0, // student: Address,
        &100, // amount

    );
}


#[test]
#[should_panic] // TODO: Change for errors
fn distribute_student_not_subscribed() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

        
    test.contract.set_is_sport_club(&test.club_0, &true);
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);
    test.contract.set_is_student(&test.student_1, &true);

    let price = 100;
    let incentive = 10;
    let ratio: u32 = 1000;
    let total_amount = price + incentive;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &100, // amount

    );
}


#[test]
#[should_panic] // TODO: Change for errors
fn distribute_negative() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

        
    test.contract.set_is_sport_club(&test.club_0, &true);
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);
    test.contract.set_is_student(&test.student_1, &true);

    let price = 100;
    let incentive = 10;
    let ratio: u32 = 1000;
    let total_amount = price + incentive;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    test.contract.subscribe_course(
                &test.parent_0, // parent: Address,
                &test.student_0, // student: Address,
                &index, // course_index: u32,
            );

    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &-10, // amount

    );
}



#[test]
#[should_panic] // TODO: Change for errors
fn distribute_not_enough() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

        
    test.contract.set_is_sport_club(&test.club_0, &true);
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);
    test.contract.set_is_student(&test.student_1, &true);

    let price = 100;
    let incentive = 10;
    let ratio: u32 = 1000;
    let total_amount = price + incentive;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    test.contract.subscribe_course(
                &test.parent_0, // parent: Address,
                &test.student_0, // student: Address,
                &index, // course_index: u32,
            );

    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &10000000000000, // amount

    );
}

#[test]
#[should_panic] // TODO: Change for errors
fn distribute_zero() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

        
    test.contract.set_is_sport_club(&test.club_0, &true);
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);
    test.contract.set_is_student(&test.student_1, &true);

    let price = 100;
    let incentive = 10;
    let ratio: u32 = 1000;
    let total_amount = price + incentive;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    test.contract.subscribe_course(
                &test.parent_0, // parent: Address,
                &test.student_0, // student: Address,
                &index, // course_index: u32,
            );

    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &0, // amount

    );
}


#[test]
fn distribute_gladius_coins() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    test.contract.set_is_sport_club(&test.club_0, &true);
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);
    test.contract.set_is_student(&test.student_1, &true);

    let price = 100;
    let incentive = 10;
    let ratio: u32 = 1000;
    let total_amount = price + incentive;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );
    assert_eq!(test.contract.get_total_courses(), 1);

    let initial_parent_0_balance = 123_000_000_000_000_000_000;
    assert_eq!(test.payment_token.balance(&test.parent_0), initial_parent_0_balance);
    assert_eq!(test.gladius_coin_emitter.total_supply(), 0);

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &index, // course_index: u32,

    );

    assert_eq!(test.contract.get_total_courses(), 1);

    let expected_gladius_coin_balance = incentive*(ratio as i128);

    assert_eq!(test.payment_token.balance(&test.parent_0), initial_parent_0_balance - total_amount);
    assert_eq!(test.payment_token.balance(&test.contract.address), 0);
    assert_eq!(test.payment_token.balance(&test.gladius_coin_emitter.address), incentive);
    assert_eq!(test.payment_token.balance(&test.club_0), price);
    assert_eq!(test.gladius_coin_emitter.balance(&test.student_0), 0);
    assert_eq!(test.gladius_coin_emitter.balance(&test.contract.address), expected_gladius_coin_balance);
    assert_eq!(test.gladius_coin_emitter.total_supply(), expected_gladius_coin_balance);

    let distribute_amount_gladius_coins = 99;

    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &distribute_amount_gladius_coins, // amount

    );

    assert_eq!(test.gladius_coin_emitter.total_supply(), expected_gladius_coin_balance);
    assert_eq!(test.gladius_coin_emitter.balance(&test.contract.address), expected_gladius_coin_balance- distribute_amount_gladius_coins);
    assert_eq!(test.gladius_coin_emitter.balance(&test.student_0), distribute_amount_gladius_coins);


}

