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
fn transfer() {
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

    let mut id = 0;
    let mut uri = String::from_str(&test.env, "my_uri_0");

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),id.clone(), uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &id, &uri);

    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.total_supply(), 1);
    assert_eq!(test.contract.owner_of(&id), test.user);
    assert_eq!(test.contract.token_uri(&id), uri);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &&0), id.clone());
    assert_eq!(test.contract.token_by_index(&0), id.clone());

    id = 1;
    uri = String::from_str(&test.env, "my_uri_1");

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),id.clone(), uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &id, &uri);

    assert_eq!(test.contract.balance_of(&test.user), 2);
    assert_eq!(test.contract.total_supply(), 2);
    assert_eq!(test.contract.owner_of(&id), test.user);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &1), id);
    assert_eq!(test.contract.token_by_index(&1), id);

    id = 2;
    uri = String::from_str(&test.env, "my_uri_2");
    let new_user = Address::generate(&test.env);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (new_user.clone(),id.clone(), uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&new_user, &id, &uri);

    assert_eq!(test.contract.balance_of(&test.user), 2);
    assert_eq!(test.contract.balance_of(&new_user), 1);
    assert_eq!(test.contract.total_supply(), 3);
    assert_eq!(test.contract.owner_of(&0), test.user);
    assert_eq!(test.contract.owner_of(&1), test.user);
    assert_eq!(test.contract.owner_of(&2), new_user);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &0), 0);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &1), 1);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 2);
    assert_eq!(test.contract.token_by_index(&0), 0);
    assert_eq!(test.contract.token_by_index(&1), 1);
    assert_eq!(test.contract.token_by_index(&2), 2);


    // NOW WE WILL TRANSFERS
    // user will transer token_id =0 to new_user

    let id_to_trasfer = 0;
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "transfer_from",
                    args: (
                        test.user.clone(), // spender
                        test.user.clone(),  // from
                        new_user.clone(), //to
                        id_to_trasfer.clone() //token_id
                    ).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer_from(
        &test.user, // spender
        &test.user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.balance_of(&new_user), 2);
    assert_eq!(test.contract.total_supply(), 3);
    assert_eq!(test.contract.owner_of(&0), new_user);
    assert_eq!(test.contract.owner_of(&1), test.user);
    assert_eq!(test.contract.owner_of(&2), new_user);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 2);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &1), 0);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &0), 1);
    assert_eq!(test.contract.token_by_index(&0), 0);
    assert_eq!(test.contract.token_by_index(&1), 1);
    assert_eq!(test.contract.token_by_index(&2), 2);

    let mut id_to_trasfer = 1;
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "transfer_from",
                    args: (
                        test.user.clone(), // spender
                        test.user.clone(),  // from
                        new_user.clone(), //to
                        id_to_trasfer.clone() //token_id
                    ).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer_from(
        &test.user, // spender
        &test.user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(test.contract.balance_of(&test.user), 0);
    assert_eq!(test.contract.balance_of(&new_user), 3);
    assert_eq!(test.contract.total_supply(), 3);
    assert_eq!(test.contract.owner_of(&0), new_user);
    assert_eq!(test.contract.owner_of(&1), new_user);
    assert_eq!(test.contract.owner_of(&2), new_user);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 2);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &1), 0);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &2), 1);
    assert_eq!(test.contract.token_by_index(&0), 0);
    assert_eq!(test.contract.token_by_index(&1), 1);
    assert_eq!(test.contract.token_by_index(&2), 2);

    id = 88;
    uri = String::from_str(&test.env, "my_uri_88");
    let new_user_88 = Address::generate(&test.env);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (new_user_88.clone(),id.clone(), uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&new_user_88, &id, &uri);

    assert_eq!(test.contract.balance_of(&test.user), 0);
    assert_eq!(test.contract.balance_of(&new_user), 3);
    assert_eq!(test.contract.balance_of(&new_user_88), 1);
    assert_eq!(test.contract.total_supply(), 4);
    assert_eq!(test.contract.owner_of(&0), new_user);
    assert_eq!(test.contract.owner_of(&1), new_user);
    assert_eq!(test.contract.owner_of(&2), new_user);
    assert_eq!(test.contract.owner_of(&88), new_user_88);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 2);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &1), 0);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &2), 1);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user_88, &0), 88);
    assert_eq!(test.contract.token_by_index(&0), 0);
    assert_eq!(test.contract.token_by_index(&1), 1);
    assert_eq!(test.contract.token_by_index(&2), 2);
    assert_eq!(test.contract.token_by_index(&3), 88);
 

    id_to_trasfer = 88;
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &new_user_88.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "transfer_from",
                    args: (
                        new_user_88.clone(), // spender
                        new_user_88.clone(),  // from
                        test.user.clone(), //to
                        id_to_trasfer.clone() //token_id
                    ).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer_from(
        &new_user_88, // spender
        &new_user_88,  // from
        &test.user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.balance_of(&new_user), 3);
    // TODO: fix Enumerable indexes
    // assert_eq!(test.contract.balance_of(&new_user_88), 0);
    assert_eq!(test.contract.total_supply(), 4);
    assert_eq!(test.contract.owner_of(&0), new_user);
    assert_eq!(test.contract.owner_of(&1), new_user);
    assert_eq!(test.contract.owner_of(&2), new_user);
    assert_eq!(test.contract.owner_of(&88), test.user);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), 2);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &1), 0);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &2), 1);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &0), 88);
    assert_eq!(test.contract.token_by_index(&0), 0);
    assert_eq!(test.contract.token_by_index(&1), 1);
    assert_eq!(test.contract.token_by_index(&2), 2);
    assert_eq!(test.contract.token_by_index(&3), 88);

    
}

#[test]
fn transfer_not_owner() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");
    let index = 0;
    let uri = String::from_str(&test.env, "my_uri_0");
    let not_owner_user = Address::generate(&test.env);
    let new_user = Address::generate(&test.env);

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

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

    let id_to_trasfer = 0;

    let res = test.contract.try_transfer_from(
        &not_owner_user, // spender
        &not_owner_user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(res, Err(Ok(GladiusNFTError::NotOwner))); 
}


#[test]
fn transfer_not_nft() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");
    let index = 0;
    let uri = String::from_str(&test.env, "my_uri_0");
    let new_user = Address::generate(&test.env);

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

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

    let id_to_trasfer = 1;

    let res = test.contract.try_transfer_from(
        &test.user, // spender
        &test.user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(res, Err(Ok(GladiusNFTError::NotNFT))); 
}

#[test]
fn transfer_not_authorized() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");
    let index = 0;
    let uri = String::from_str(&test.env, "my_uri_0");
    let new_user = Address::generate(&test.env);
    let not_authorized_user = Address::generate(&test.env);


    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

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

    let id_to_trasfer = 0;

    let res = test.contract.try_transfer_from(
        &not_authorized_user, // spender
        &test.user,  // from
        &new_user, //to
        &id_to_trasfer //token_id
    );

    assert_eq!(res, Err(Ok(GladiusNFTError::NotAuthorized))); 
}