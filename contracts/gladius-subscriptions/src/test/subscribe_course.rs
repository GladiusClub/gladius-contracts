use soroban_sdk::{String};
use crate::test::{GladiusSubscriptionsTest}; 
use crate::test::gladius_subscriptions::Course;
use soroban_sdk::{testutils::{Events, MockAuthInvoke, MockAuth}, vec, IntoVal, symbol_short};


// subscribe_course
#[test]
#[should_panic] // TODO: Change for errors
fn subscribe_course_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,

    );
}


// subscribe_course
#[test]
#[should_panic] // TODO: Change for errors
fn subscribe_course_not_parent() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,

    );
}


#[test]
#[should_panic] // TODO: Change for errors
fn subscribe_course_not_student() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    test.contract.set_is_parent(&test.parent_0, &true);

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,

    );
}


#[test]
#[should_panic] // TODO: Change for errors
fn subscribe_course_not_exist() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    test.contract.set_is_parent(&test.parent_0, &true);
    test.contract.set_is_student(&test.student_0, &true);

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,

    );
}


#[test]
fn subscribe_course() {
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

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.parent_0.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "subscribe_course",
                    args: (test.parent_0.clone(), test.student_0.clone(), index.clone(),).into_val(&test.env),
                    sub_invokes: &[

                        MockAuthInvoke {
                        contract: &test.payment_token.address,
                        fn_name: "transfer",
                        args: (test.parent_0.clone(), test.contract.address.clone(), total_amount.clone(),).into_val(&test.env),
                        sub_invokes: &[],}
                    ],
                },
        }
    ])
    .subscribe_course(
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
    assert_eq!(test.gladius_coin_emitter.balance(&test.contract.address), expected_gladius_coin_balance);

    let expected_course = Course {
        club: test.club_0.clone(),
        price,
        incentive,
        subscriptions: vec![&test.env, test.student_0.clone()],
        title: title.clone(),
        active: true,
        gladius_coin_balance: expected_gladius_coin_balance,
    };
    let read_course = test.contract.get_course(&index);

    assert_eq!(read_course, expected_course);

    // TODO: Check that parent cannot subscribe same student again

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.parent_0.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "subscribe_course",
                    args: (test.parent_0.clone(), test.student_1.clone(), index.clone(),).into_val(&test.env),
                    sub_invokes: &[

                        MockAuthInvoke {
                        contract: &test.payment_token.address,
                        fn_name: "transfer",
                        args: (test.parent_0.clone(), test.contract.address.clone(), total_amount.clone(),).into_val(&test.env),
                        sub_invokes: &[],}
                    ],
                },
        }
    ])
    .subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_1, // student: Address,
        &index, // course_index: u32,
    );

    assert_eq!(test.payment_token.balance(&test.parent_0), initial_parent_0_balance - total_amount*2 );
    assert_eq!(test.payment_token.balance(&test.contract.address), 0);
    assert_eq!(test.payment_token.balance(&test.gladius_coin_emitter.address), incentive*2 );
    assert_eq!(test.payment_token.balance(&test.club_0), price*2 );
    assert_eq!(test.gladius_coin_emitter.balance(&test.contract.address), expected_gladius_coin_balance*2 );

    let expected_new_course = Course {
        club: test.club_0,
        price,
        incentive,
        subscriptions: vec![&test.env, test.student_0.clone(), test.student_1.clone()],
        title,
        active: true,
        gladius_coin_balance: expected_gladius_coin_balance*2,
    };
    let read_new_course = test.contract.get_course(&index);

    assert_eq!(read_new_course, expected_new_course);
}



#[test]
#[should_panic] // TODO: Change for errors
fn subscribe_course_twice_same_student() {
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

    test.contract.create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,
    );

    test.contract.subscribe_course(
        &test.parent_0, // parent: Address,
        &test.student_0, // student: Address,
        &0, // course_index: u32,
    );
}