use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
use crate::test::gladius_nft::GladiusNFTError;
use crate::event::{MintEvent};

use soroban_sdk::{
    Address, IntoVal,
    testutils::{
        MockAuth, MockAuthInvoke,
        Events,
        Address as _},
    vec, symbol_short
};

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

    let index = 0;
    let uri = String::from_str(&test.env, "my_uri");

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

    let mint_event = test.env.events().all().last().unwrap();

    let expected_mint_event: MintEvent = MintEvent {
        to: test.user.clone(),
        token_id: index.clone(),
        uri: uri.clone(),
    };

    assert_eq!(
        vec![&test.env, mint_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("GladiusNFT", symbol_short!("mint")).into_val(&test.env),
                (expected_mint_event).into_val(&test.env)
            ),
        ]
    );


    assert_eq!(test.contract.balance_of(&test.user), 1);
    assert_eq!(test.contract.total_supply(), 1);
    assert_eq!(test.contract.owner_of(&0), test.user);
    assert_eq!(test.contract.token_uri(&0), uri);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &&0), 0);
    assert_eq!(test.contract.token_by_index(&0), 0);

    let mut new_index = 99;
    let mut new_uri = String::from_str(&test.env, "my_new_uri");

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (test.user.clone(),new_index.clone(), new_uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&test.user, &new_index, &new_uri);

    assert_eq!(test.contract.balance_of(&test.user), 2);
    assert_eq!(test.contract.total_supply(), 2);
    assert_eq!(test.contract.owner_of(&new_index), test.user);
    assert_eq!(test.contract.token_uri(&new_index), new_uri);
    assert_eq!(test.contract.token_of_owner_by_index(&test.user, &1), new_index);
    assert_eq!(test.contract.token_by_index(&1), new_index);

    new_index = 300;
    new_uri = String::from_str(&test.env, "uriuriuri");
    let new_user = Address::generate(&test.env);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "mint",
                    args: (new_user.clone(),new_index.clone(), new_uri.clone()).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&new_user, &new_index, &new_uri);

    assert_eq!(test.contract.balance_of(&new_user), 1);
    assert_eq!(test.contract.total_supply(), 3);
    assert_eq!(test.contract.owner_of(&new_index), new_user);
    assert_eq!(test.contract.token_uri(&new_index), new_uri);
    assert_eq!(test.contract.token_of_owner_by_index(&new_user, &0), new_index);
    assert_eq!(test.contract.token_by_index(&2), new_index);

    
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

    let index = 0;
    let uri = String::from_str(&test.env, "my_uri");

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
}


#[test]
fn no_owner() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

    let res = test.contract.try_owner_of(&0);
    assert_eq!(res, Err(Ok(GladiusNFTError::NotNFT))); 
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

    let index = 0;
    let uri = String::from_str(&test.env, "my_uri");

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
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

}