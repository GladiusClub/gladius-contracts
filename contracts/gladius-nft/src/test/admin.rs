use soroban_sdk::{String};
use crate::test::{GladiusNFTTest}; 
use soroban_sdk::{
    Address, IntoVal,
    testutils::{
        MockAuth, MockAuthInvoke,
        Events,
        Address as _},
    vec, symbol_short
};
use crate::event::{SetAdminEvent};



#[test]
fn admin_can_change() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );
    
    assert_eq!(test.contract.admin(), test.admin);
    
    let new_admin = Address::generate(&test.env);
    
    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_admin",
                    args: (new_admin.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_admin(&new_admin);

    assert_eq!(test.contract.admin(), new_admin);
    // TODO: test events

}

#[test]
#[should_panic] // TODO: Transform to error
fn user_cannot_set_admin() {
    let test = GladiusNFTTest::setup();

    let name = String::from_str(&test.env, "Cool NFT");
    let symbol = String::from_str(&test.env, "COOL");

    test.contract.initialize(
        &test.admin,
        &name,
        &symbol,
    );

    let new_admin = Address::generate(&test.env);

    test.contract
    .mock_auths(&[
        MockAuth {
            address: &test.user.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.contract.address,
                    fn_name: "set_admin",
                    args: (new_admin.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_admin(&new_admin);
}


