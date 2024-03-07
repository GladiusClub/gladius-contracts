use soroban_sdk::{String};
use crate::test::{GladiusSubscriptionsTest}; 
use crate::test::gladius_subscriptions::{Course, GladiusSubscriptionsError};
use soroban_sdk::{testutils::{Events, MockAuthInvoke, MockAuth}, vec, IntoVal, symbol_short};

// create_course
#[test]
fn create_course_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    let res = test.contract.try_create_course(
        &test.club_0,
        &0,
        &0,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::NotInitialized))); 
}


#[test]
fn create_course_not_sport_club() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    let res = test.contract.try_create_course(
        &test.club_0,
        &100,
        &100,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::SportClubNotFound))); 
}


#[test]
fn create_course_zero_price() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    test.contract.set_is_sport_club(&test.club_0, &true);

    let res = test.contract.try_create_course(
        &test.club_0,
        &0,
        &100,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
}

#[test]
fn create_course_zero_incentive() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    test.contract.set_is_sport_club(&test.club_0, &true);

    let res = test.contract.try_create_course(
        &test.club_0,
        &100,
        &0,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
}


#[test]
fn create_course_negative_price() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    test.contract.set_is_sport_club(&test.club_0, &true);

    let res = test.contract.try_create_course(
        &test.club_0,
        &-10,
        &100,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
}


#[test]
fn create_course_negative_incentive() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    test.contract.set_is_sport_club(&test.club_0, &true);

    let res = test.contract.try_create_course(
        &test.club_0,
        &10,
        &-100,
        &String::from_str(&test.env, "Title")

    );
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::ZeroOrNegativesNotSupported))); 
}


#[test]
fn create_course() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    test.contract.set_is_sport_club(&test.club_0, &true);

    let price = 100;
    let incentive = 10;
    let title = String::from_str(&test.env, "Title");

    assert_eq!(test.contract.get_total_courses(), 0);

    let index = test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.club_0.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "create_course",
                    args: (
                        test.club_0.clone(), 
                        price.clone(), 
                        incentive.clone(), 
                        title.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .create_course(
        &test.club_0, 
        &price,
        &incentive,
        &title
    );

    let expected_course = Course {
        club: test.club_0,
        price,
        incentive,
        subscriptions: vec![&test.env].into(),
        title,
        active: true,
        gladius_coin_balance: 0,
    };
    let read_course = test.contract.get_course(&index);

    assert_eq!(read_course, expected_course);
    assert_eq!(test.contract.get_total_courses(), 1);
}


#[test]
fn get_course_dont_exist() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );

    let res = test.contract.try_get_course(&2);
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::CourseDoesNotExist))); 
}