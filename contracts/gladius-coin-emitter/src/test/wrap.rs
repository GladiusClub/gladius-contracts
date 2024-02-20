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
    let decimals: u32 = 7;

    test.contract.initialize_gladius(
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
    let decimals: u32 = 7;

    test.contract.initialize_gladius(
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
    let decimals: u32 = 7;

    test.contract.initialize_gladius(
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
