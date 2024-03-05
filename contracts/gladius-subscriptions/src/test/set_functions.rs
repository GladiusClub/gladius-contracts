use soroban_sdk::{String};
use crate::test::{GladiusSubscriptionsTest}; 
use soroban_sdk::{testutils::{Events, MockAuthInvoke, MockAuth}, vec, IntoVal, symbol_short};


#[test]
fn no_address_is_role() {
    let test = GladiusSubscriptionsTest::setup();
    
    assert_eq!(test.contract.is_sport_club(&test.user), false);
    assert_eq!(test.contract.is_parent(&test.user), false);
    assert_eq!(test.contract.is_student(&test.user), false);
}

#[test]
#[should_panic] // TODO: Change for errors
fn set_is_sport_club_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.set_is_sport_club(&test.club_0, &true);
}

#[test]
#[should_panic] // TODO: Change for errors
fn set_is_parent_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.set_is_parent(&test.club_0, &true);
}
#[test]
#[should_panic] // TODO: Change for errors
fn set_is_student_not_initialized() {
    let test = GladiusSubscriptionsTest::setup();
    test.contract.set_is_student(&test.club_0, &true);
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

