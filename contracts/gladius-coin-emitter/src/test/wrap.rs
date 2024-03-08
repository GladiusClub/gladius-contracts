extern crate std;
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
};
use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};

use crate::test::{GladiusCoinEmitterTest}; 
use crate::event::{WrapEvent};



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
    assert_eq!(test.contract.total_supply(), 0);


    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 87654321;

    let minted_amount = test.contract.wrap_and_mint(&test.minter, &amount);


    let wrap_event = test.env.events().all().last().unwrap();

    let expected_wrap_event: WrapEvent = WrapEvent {
        minter: test.minter.clone(),
        wrapped_amount: amount,
        minted_amount:  amount * (ratio as i128),
        to: test.minter.clone()
    };

    assert_eq!(
        vec![&test.env, wrap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusCoinEmitter", symbol_short!("wrap")).into_val(&test.env),
                (expected_wrap_event).into_val(&test.env)
            ),
        ]
    );

    // Correct returned minted_amount
    assert_eq!(minted_amount, amount * (ratio as i128));
    

    // New pegged token balances
    assert_eq!(test.pegged_token.balance(&test.minter), minter_original_pegged_token_balance - amount);
    assert_eq!(test.pegged_token.balance(&test.user), user_original_pegged_token_balance);
    assert_eq!(test.pegged_token.balance(&test.contract.address), amount);

    // New Gladius Coin token balances
    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128));
    assert_eq!(test.contract.balance(&test.user), 0);
    assert_eq!(test.contract.balance(&test.contract.address), 0);
    assert_eq!(test.contract.total_supply(), amount * (ratio as i128));

    // TODO: Test mint event

    // Minted Gladius Coin tokens can be sent to any user
    let transfer_amount = 987;
    test.contract.transfer(&test.minter, &test.user, &transfer_amount);

    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128) - transfer_amount);
    assert_eq!(test.contract.balance(&test.user), transfer_amount);
    assert_eq!(test.contract.balance(&test.contract.address), 0);
    assert_eq!(test.contract.total_supply(), amount * (ratio as i128));

}