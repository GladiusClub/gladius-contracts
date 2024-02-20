extern crate std;
use crate::test::{GladiusCoinEmitterTest}; 
use crate::test::gladius_coin_emitter::GladiusCoinEmitterError;

use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    IntoVal
};


#[test]
fn unwrap_not_initialized() {
    let test = GladiusCoinEmitterTest::setup();

    let amount: i128 = 1000;
    let res = test.contract.try_unwrap_and_burn(
        &test.minter,
        &amount,
        );  
    
    assert_eq!(res, Err(Ok(GladiusCoinEmitterError::NotInitialized))); 
}

#[test]
fn unwrap_negatives_not_allowed() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = -1000;
    let res = test.contract.try_unwrap_and_burn(
        &test.minter,
        &amount,
        );  
    assert_eq!(res, Err(Ok(GladiusCoinEmitterError::UnWrapNegativesNotSupported))); 
}

#[test]
fn unwrap_user_with_coins_can_unwrap() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 8765;

    test.contract.wrap_and_mint(&test.minter, &amount);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.minter.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "unwrap_and_burn",
                    args: (test.minter.clone(), amount.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .unwrap_and_burn(&test.minter, &amount);
}


#[test]
#[should_panic]
fn unwrap_cannot_unwrap_for_others() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 8765;

    test.contract.wrap_and_mint(&test.minter, &amount);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "unwrap_and_burn",
                    args: (test.minter.clone(), amount.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .unwrap_and_burn(&test.minter, &amount);
}


#[test]
#[should_panic] // TODO: Transform in error object
fn unwrap_insufficient_coin_balance() {
    let test = GladiusCoinEmitterTest::setup();

    let ratio: u32 = 1000;

    test.contract.initialize(
        &test.minter,
        &test.pegged_token.address,
        &ratio
        );

    let amount: i128 = 8765;

    test.contract.wrap_and_mint(&test.minter, &amount);
    let transfer_amount = 987;
    test.contract.transfer(&test.minter, &test.user, &transfer_amount);

    test.contract.unwrap_and_burn(&test.minter, &amount);
}


#[test]
fn unwrap_correct_amounts() {
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
    let transfer_amount = 1000;
    test.contract.transfer(&test.minter, &test.user, &transfer_amount);

    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128) - transfer_amount);
    assert_eq!(test.contract.balance(&test.user), transfer_amount);
    assert_eq!(test.contract.balance(&test.contract.address), 0);

    let unwrap_amount = 1;
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "unwrap_and_burn",
                    args: (test.user.clone(), unwrap_amount.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .unwrap_and_burn(&test.user, &unwrap_amount);

    // New pegged token balances
    assert_eq!(test.pegged_token.balance(&test.minter), minter_original_pegged_token_balance - amount);
    assert_eq!(test.pegged_token.balance(&test.user), user_original_pegged_token_balance + 1);
    assert_eq!(test.pegged_token.balance(&test.contract.address), amount - 1);

    // New Gladius Coin token balances
    assert_eq!(test.contract.balance(&test.minter), amount * (ratio as i128) - transfer_amount);
    assert_eq!(test.contract.balance(&test.user), 0);
    assert_eq!(test.contract.balance(&test.contract.address), 0);
    // TODO: Test Total Supply


}
// TODO: Test that only Sport Clubs can unwrap