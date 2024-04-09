use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
use crate::test::gladius_nft::GladiusNFTError;

use soroban_sdk::{
    Address, IntoVal,
    testutils::{
        MockAuth, MockAuthInvoke,
        // Events,
        Address as _},
    // vec, symbol_short
};

#[test]
fn approve() {
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

    let index = 0;
    let uri = String::from_str(&test.env, "my_uri_0");

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),index.clone(), uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &index, &uri);

    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.total_supply(), 1);
    assert_eq!(test.contract.owner_of(&0), test.user);
    assert_eq!(test.contract.token_uri(&0), uri);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &&0), 0);
    assert_eq!(test.contract.token_by_index(&0), 0);

    let id_to_approve = 0;
    let ttl = 1000;
    let operator = Address::generate(&test.env);


    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "approve",
                    args: (
                        test.user.clone(), // caller
                        operator.clone(),  // operator
                        id_to_approve.clone(), //token_id
                        ttl.clone() //ttl
                    ).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .approve(
        &test.user, // caller
        &Some(operator.clone()),  // operator
        &id_to_approve, //token_id
        &ttl //ttl
    );

    assert_eq!(test.contract.get_approved(&0), Some(operator.clone()));

    let new_user = Address::generate(&test.env);
    let id_to_trasfer = 0;

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &operator.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "transfer_from",
                    args: (
                        operator.clone(), // spender
                        test.user.clone(),  // from
                        new_user.clone(), //to
                        id_to_trasfer.clone() //token_id
                    ).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer_from(
        &operator, // spender
        &test.user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(test.contract.balance_of(&test.user), 0);
    assert_eq!(test.contract.balance_of(&new_user), 1);
    assert_eq!(test.contract.total_supply(), 1);
    assert_eq!(test.contract.owner_of(&0), new_user);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 0);
    assert_eq!(test.contract.token_by_index(&0), 0);
    
}

// TODO: Test approve not authorized
// TODO: Test approve for all
// TODO: Test revoke approve
// TT Test approve for all not authorized