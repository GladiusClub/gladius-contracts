#![cfg(test)]
extern crate std;

use crate::test::{GladiusCoinEmitterTest}; 
use crate::gladius_coin::{GladiusCoinToken, GladiusCoinTokenClient};

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, IntoVal, Symbol,
};


#[test]
fn test() {
    let test = GladiusCoinEmitterTest::setup();
    test.env.mock_all_auths();

    let admin2 = Address::generate(&test.env);
    let user1 = Address::generate(&test.env);
    let user2 = Address::generate(&test.env);
    let user3 = Address::generate(&test.env);

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    test.contract.mint(&user1, &1000);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.minter.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.balance(&user1), 1000);
    assert_eq!(test.contract.total_supply(), 1000);

    test.contract.approve(&user2, &user3, &500, &200);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.allowance(&user2, &user3), 500);

    test.contract.transfer(&user1, &user2, &600);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.balance(&user1), 400);
    assert_eq!(test.contract.balance(&user2), 600);

    test.contract.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.balance(&user1), 800);
    assert_eq!(test.contract.balance(&user2), 200);

    test.contract.transfer(&user1, &user3, &300);
    assert_eq!(test.contract.balance(&user1), 500);
    assert_eq!(test.contract.balance(&user3), 300);

    test.contract.set_admin(&admin2);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.minter.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // Increase to 500
    test.contract.approve(&user2, &user3, &500, &200);
    assert_eq!(test.contract.allowance(&user2, &user3), 500);
    test.contract.approve(&user2, &user3, &0, &200);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn() {
    let test = GladiusCoinEmitterTest::setup();
    test.env.mock_all_auths();

    let user1 = Address::generate(&test.env);
    let user2 = Address::generate(&test.env);

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    test.contract.mint(&user1, &1000);
    assert_eq!(test.contract.balance(&user1), 1000);
    assert_eq!(test.contract.total_supply(), 1000);

    test.contract.approve(&user1, &user2, &500, &200);
    assert_eq!(test.contract.allowance(&user1, &user2), 500);

    test.contract.burn_from(&user2, &user1, &500);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.total_supply(), 500);
    assert_eq!(test.contract.allowance(&user1, &user2), 0);
    assert_eq!(test.contract.balance(&user1), 500);
    assert_eq!(test.contract.balance(&user2), 0);

    test.contract.burn(&user1, &500);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.balance(&user1), 0);
    assert_eq!(test.contract.balance(&user2), 0);
}

#[test]
#[should_panic]
fn transfer_insufficient_balance() {
    let test = GladiusCoinEmitterTest::setup();
    test.env.mock_all_auths();

    let user1 = Address::generate(&test.env);
    let user2 = Address::generate(&test.env);

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    test.contract.mint(&user1, &1000);
    assert_eq!(test.contract.balance(&user1), 1000);
    assert_eq!(test.contract.total_supply(), 2000);

    test.contract.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic]
fn transfer_from_insufficient_allowance() {
    let test = GladiusCoinEmitterTest::setup();
    test.env.mock_all_auths();

    let user1 = Address::generate(&test.env);
    let user2 = Address::generate(&test.env);
    let user3 = Address::generate(&test.env);

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    test.contract.mint(&user1, &1000);
    assert_eq!(test.contract.balance(&user1), 1000);
    assert_eq!(test.contract.total_supply(), 1000);

    test.contract.approve(&user1, &user3, &100, &200);
    assert_eq!(test.contract.allowance(&user1, &user3), 100);

    test.contract.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic]
fn initialize_already_initialized() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );
}

#[test]
fn test_zero_allowance() {
    let test = GladiusCoinEmitterTest::setup();
    // Here we test that transfer_from with a 0 amount does not create an empty allowance
    test.env.mock_all_auths();

    let spender = Address::generate(&test.env);
    let from = Address::generate(&test.env);

    let token_client = GladiusCoinTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, GladiusCoinToken {}));

    test.contract.transfer_from(&spender, &from, &spender, &0);
    assert!(token_client.get_allowance(&from, &spender).is_none());
}
