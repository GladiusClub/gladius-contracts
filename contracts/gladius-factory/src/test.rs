#![cfg(test)]
extern crate std;
use soroban_sdk::{testutils::{Address as _},
    Address, 
    BytesN, 
    Env,
    String};
use crate::{GladiusFactoryClient};

// **** TOKEN CONTRACT ****
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
fn create_token_contract<'a>(e: &Env) -> TokenClient<'a> {
    let token_address = &e.register_contract_wasm(None, token::WASM);
    let token = TokenClient::new(e, token_address);
    token
}
use token::TokenClient;

//  **** COIN EMITTER ****
fn coin_emitter_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}


//  **** SUBSCRIPTIONS ****
fn subscriptions_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../gladius-subscriptions/target/wasm32-unknown-unknown/release/gladius_subscriptions.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}


//  **** NFT ****
fn nft_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../gladius-nft/target/wasm32-unknown-unknown/release/gladius_nft.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

//  **** FACTORY CONTRACT (TO BE TESTED) **** 
fn create_factory_contract<'a>(e: & Env) -> GladiusFactoryClient<'a> {
    let factory = GladiusFactoryClient::new(e, &e.register_contract(None, crate::GladiusFactory {}));
    factory
}


// THE TEST
pub struct GladiusFactoryTest<'a> {
    env: Env,
    admin: Address,
    token: TokenClient<'a>,
    coin_emitter_wasm: BytesN<32>,
    subscriptions_wasm: BytesN<32>,
    nft_wasm: BytesN<32>,
    contract: GladiusFactoryClient<'a>,
}

impl<'a> GladiusFactoryTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        // let user = Address::generate(&env);
        let token = create_token_contract(&env);
        
        token.initialize(&admin, &7, &String::from_str(&env, "Token 0"), &String::from_str(&env, "TOKEN0"));
        
        let coin_emitter_wasm = coin_emitter_wasm(&env); 
        let subscriptions_wasm = subscriptions_wasm(&env); 
        let nft_wasm = nft_wasm(&env);  
        let contract = create_factory_contract(&env);

        // TODO: Get rid of this hack?
        env.budget().reset_unlimited();
    

        GladiusFactoryTest {
            env,
            admin,
            token,
            coin_emitter_wasm,
            subscriptions_wasm,
            nft_wasm,
            contract,
        }
    }
}

mod initialize;
mod premium_clubs;
mod events;