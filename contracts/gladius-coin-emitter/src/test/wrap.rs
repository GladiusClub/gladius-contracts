extern crate std;
use crate::test::{GladiusCoinEmitterTest}; 
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    IntoVal
};


#[test]
#[should_panic]
fn wrap_negatives_not_allowed() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = -1000;
    test.contract.wrap_and_mint(
        &test.minter,
        &amount,
        );  
}

#[test]
fn wrap_minter_can_wrap() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 1000;

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.minter.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "wrap_and_mint",
                    args: (test.minter.clone(), amount.clone(),).into_val(&test.env),
                    sub_invokes: &[
                        MockAuthInvoke {
                        contract: &test.pegged_token.address,
                        fn_name: "transfer",
                        args: (test.minter.clone(), test.contract.address.clone(), amount.clone(),).into_val(&test.env),
                        sub_invokes: &[],
                    }],
                },
        }
    ])
    .wrap_and_mint(&test.minter, &amount);
}


#[test]
#[should_panic]
fn wrap_user_cannot_wrap() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 1000;

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "wrap_and_mint",
                    args: (test.user.clone(), amount.clone(),).into_val(&test.env),
                    sub_invokes: &[
                        MockAuthInvoke {
                        contract: &test.pegged_token.address,
                        fn_name: "transfer",
                        args: (test.user.clone(), test.contract.address.clone(), amount.clone(),).into_val(&test.env),
                        sub_invokes: &[],
                    }],
                },
        }
    ])
    .wrap_and_mint(&test.user, &amount);
}


#[test]
fn wrap_correct_amounts() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;
    let minter_original_pegged_token_balance = 123_000_000_000_000_000_000;
    let user_original_pegged_token_balance = 321_000_000_000_000_000_000;

    assert_eq!(test.pegged_token.balance(&test.minter), minter_original_pegged_token_balance);
    assert_eq!(test.pegged_token.balance(&test.user), user_original_pegged_token_balance);
    assert_eq!(test.pegged_token.balance(&test.contract.address), 0);

    assert_eq!(test.contract.balance(&test.minter), 0);
    assert_eq!(test.contract.balance(&test.user), 0);
    assert_eq!(test.contract.balance(&test.contract.address), 0);
    // TODO: Test Total Supply


    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 87654321;

    test.contract.wrap_and_mint(&test.minter, &amount);

    // New pegged token balances
    assert_eq!(test.pegged_token.balance(&test.minter), minter_original_pegged_token_balance - amount);
    assert_eq!(test.pegged_token.balance(&test.user), user_original_pegged_token_balance);
    assert_eq!(test.pegged_token.balance(&test.contract.address), amount);

    // New Gladius Coin token balances
    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128));
    assert_eq!(test.contract.balance(&test.user), 0);
    assert_eq!(test.contract.balance(&test.contract.address), 0);
    // TODO: Test Total Supply

    // TODO: Test mint event

    // Minted Gladius Coin tokens can be sent to any user
    let transfer_amount = 987;
    test.contract.transfer(&test.minter, &test.user, &transfer_amount);

    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128) - transfer_amount);
    assert_eq!(test.contract.balance(&test.user), transfer_amount);
    assert_eq!(test.contract.balance(&test.contract.address), 0);

}