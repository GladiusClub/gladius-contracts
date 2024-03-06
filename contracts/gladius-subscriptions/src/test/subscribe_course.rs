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