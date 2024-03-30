use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
use soroban_sdk::{
    Address, IntoVal,
    testutils::{
        MockAuth, MockAuthInvoke,
        Events,
        Address as _},
    vec, symbol_short};

#[test]
fn mint() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.admin(), test.admin);
    assert_eq!(test.contract.balance_of(&test.user), 0);

    let amount = 0;

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),amount.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &amount);

    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.total_supply(), 1);
    assert_eq!(test.contract.owner_of(&0), test.user);
    assert_eq!(test.contract.token_uri(&0), String::from_str(&test.env, "no uri"));
    
}

#[test]
#[should_panic] // TODO: Transform to error
fn mint_double_index() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.balance_of(&test.user), 0);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),0).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &0);

    test.contract
        .mock_auths(&[
            MockAuth {
                address: &test.admin.clone(),
                invoke: 
                    &MockAuthInvoke {
                        contract: &test.contract.address,
                        fn_name: "mint",
                        args: (test.user.clone(),0).into_val(&test.env),
                        sub_invokes: &[],
                    },
            }
        ])
        .mint(&test.user, &0);
}


#[test]
#[should_panic] // TODO: Transform to error
fn no_owner() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    let dummy = test.contract.owner_of(&0);
   
}


#[test]
#[should_panic] // TODO: Transform to error
fn mint_not_admin() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.balance_of(&test.user), 0);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),0).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &0);

}