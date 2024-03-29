use soroban_sdk::{
    testutils::{Events, MockAuthInvoke, MockAuth},
    vec, IntoVal, symbol_short, String};

use crate::event::{DistributeGladiusCoinsEvent};
use crate::test::gladius_subscriptions::GladiusSubscriptionsError;
use crate::test::{GladiusSubscriptionsTest}; 


#[test]
fn distribute_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    let res = test.contract.try_distribute_gladius_coins(
        &0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,
    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::NotInitialized))); 
}


#[test]
fn distribute_course_does_not_exist() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    let res = test.contract.try_distribute_gladius_coins(
        &0, // index
        &test.student_0, // student: Address,
        &100, // amount

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::CourseDoesNotExist))); 
}



#[test]
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
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);
    let index = test.contract
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    let res = test.contract.try_distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &100, // amount

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::CourseDoesNotContainsStudent))); 
}


#[test]
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

    let res = test.contract.try_distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &-10, // amount

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
}



#[test]
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

    let res = test.contract.try_distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &10000000000000, // amount

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::InsufficientFunds))); 
}

#[test]
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

    let res = test.contract.try_distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &0, // amount

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
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
    let expected_new_course_balance = expected_gladius_coin_balance- distribute_amount_gladius_coins;

    // test.contract
    // .mock_auths(&[
    //     MockAuth {
    //         address: &test.club_0.clone(),
    //         invoke: 
    //             &MockAuthInvoke {
    //                 contract: &test.contract.address,
    //                 fn_name: "distribute_gladius_coins",
    //                 args: (index.clone(), test.student_0.clone(), distribute_amount_gladius_coins.clone(),).into_val(&test.env),
    //                 sub_invokes: &[

    //                     MockAuthInvoke {
    //                     contract: &test.payment_token.address,
    //                     fn_name: "transfer",
    //                     args: (test.parent_0.clone(), test.contract.address.clone(), total_amount.clone(),).into_val(&test.env),
    //                     sub_invokes: &[],}
    //                 ],
    //             },
    //     }
    // ])
    // .distribute_gladius_coins(
    //     &test.parent_0, // parent: Address,
    //     &test.student_1, // student: Address,
    //     &index, // course_index: u32,
    // );

    // TODO: Test with auth
    test.contract.distribute_gladius_coins(
        &index, // index
        &test.student_0, // student: Address,
        &distribute_amount_gladius_coins, // amount

    );

    let gladius_coins_distributed_event = test.env.events().all().last().unwrap();

    let expected_gladius_coins_distributed_event: DistributeGladiusCoinsEvent = DistributeGladiusCoinsEvent {
        course_index: index,
        student: test.student_0.clone(),
        amount: distribute_amount_gladius_coins,
        new_course_balance: expected_new_course_balance
    };

    assert_eq!(
        vec![&test.env, gladius_coins_distributed_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusSubscriptions", symbol_short!("coin_dist")).into_val(&test.env),
                (expected_gladius_coins_distributed_event).into_val(&test.env)
            ),
        ]
    );

    assert_eq!(test.gladius_coin_emitter.total_supply(), expected_gladius_coin_balance);
    assert_eq!(test.gladius_coin_emitter.balance(&test.contract.address), expected_new_course_balance);
    assert_eq!(test.gladius_coin_emitter.balance(&test.student_0), distribute_amount_gladius_coins);


}

