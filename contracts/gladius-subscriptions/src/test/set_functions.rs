use soroban_sdk::{testutils::{Events, MockAuthInvoke, MockAuth}, vec, IntoVal, symbol_short};
use crate::test::{GladiusSubscriptionsTest}; 
use crate::test::gladius_subscriptions::GladiusSubscriptionsError;
use crate::event::{SetSportClubEvent, SetParentEvent, SetStudentEvent};



#[test]
fn no_address_is_role() {
    let test = GladiusSubscriptionsTest::setup();
    
    assert_eq!(test.contract.is_sport_club(&test.user), false);
    assert_eq!(test.contract.is_parent(&test.user), false);
    assert_eq!(test.contract.is_student(&test.user), false);
}

#[test]
fn set_is_sport_club_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    let res = test.contract.try_set_is_sport_club(&test.club_0, &true);
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::NotInitialized))); 
}

#[test]
fn set_is_parent_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    let res = test.contract.try_set_is_parent(&test.club_0, &true);
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::NotInitialized))); 
}
#[test]
fn set_is_student_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    let res = test.contract.try_set_is_student(&test.club_0, &true);
    assert_eq!(res, Err(Ok(GladiusSubscriptionsError::NotInitialized))); 
}

#[test]
fn set_is_sport_club() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    
    assert_eq!(test.contract.is_sport_club(&test.club_0), false);
    assert_eq!(test.contract.is_sport_club(&test.club_1), false);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_sport_club",
                    args: (test.club_0.clone(), true,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_sport_club(&test.club_0, &true);

    let sport_club_set_event = test.env.events().all().last().unwrap();

    let expected_sport_club_set_event: SetSportClubEvent = SetSportClubEvent {
        sport_club: test.club_0.clone(),
        set: true
    };

    assert_eq!(
        vec![&test.env, sport_club_set_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusSubscriptions", symbol_short!("club_set")).into_val(&test.env),
                (expected_sport_club_set_event).into_val(&test.env)
            ),
        ]
    );

    assert_eq!(test.contract.is_sport_club(&test.club_0), true);
    assert_eq!(test.contract.is_sport_club(&test.club_1), false);
    
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_sport_club",
                    args: (test.club_0.clone(), false,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_sport_club(&test.club_0, &false);

    assert_eq!(test.contract.is_sport_club(&test.club_0), false);
    assert_eq!(test.contract.is_sport_club(&test.club_1), false);
}

#[test]
fn set_is_parent() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    
    assert_eq!(test.contract.is_parent(&test.parent_0), false);
    assert_eq!(test.contract.is_parent(&test.parent_1), false);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_parent",
                    args: (test.parent_0.clone(), true,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_parent(&test.parent_0, &true);

    let parent_set_event = test.env.events().all().last().unwrap();
    let expected_parent_set_event: SetParentEvent = SetParentEvent {
        parent: test.parent_0.clone(),
        set: true
    };
    assert_eq!(
        vec![&test.env, parent_set_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusSubscriptions", symbol_short!("paren_set")).into_val(&test.env),
                (expected_parent_set_event).into_val(&test.env)
            ),
        ]
    );

    assert_eq!(test.contract.is_parent(&test.parent_0), true);
    assert_eq!(test.contract.is_parent(&test.parent_1), false);
    
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_parent",
                    args: (test.parent_0.clone(), false,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_parent(&test.parent_0, &false);

    assert_eq!(test.contract.is_parent(&test.parent_0), false);
    assert_eq!(test.contract.is_parent(&test.parent_1), false);
}

#[test]
fn set_is_student() {
    let test = GladiusSubscriptionsTest::setup();

    test.contract.initialize(
        &test.gladius_admin,
        &test.payment_token.address,
        &test.gladius_coin_emitter.address
    );
    
    
    assert_eq!(test.contract.is_student(&test.student_0), false);
    assert_eq!(test.contract.is_student(&test.student_1), false);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_student",
                    args: (test.student_0.clone(), true,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_student(&test.student_0, &true);

    let student_set_event = test.env.events().all().last().unwrap();
    let expected_student_set_event: SetStudentEvent = SetStudentEvent {
        student: test.student_0.clone(),
        set: true
    };
    assert_eq!(
        vec![&test.env, student_set_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusSubscriptions", symbol_short!("stud_set")).into_val(&test.env),
                (expected_student_set_event).into_val(&test.env)
            ),
        ]
    );

    assert_eq!(test.contract.is_student(&test.student_0), true);
    assert_eq!(test.contract.is_student(&test.student_1), false);
    
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.gladius_admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_is_student",
                    args: (test.student_0.clone(), false,).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_is_student(&test.student_0, &false);

    assert_eq!(test.contract.is_student(&test.student_0), false);
    assert_eq!(test.contract.is_student(&test.student_1), false);
}

