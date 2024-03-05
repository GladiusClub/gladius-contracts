#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _},
    Address, 
    Env,
    String,
};

// PAYMENT TOKEN TO BE PEGGED (EURC)
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;
fn create_token_contract<'a>(e: &Env) -> TokenClient<'a> {
    let token_address = &e.register_contract_wasm(None, token::WASM);
    let token = TokenClient::new(e, token_address);
    token
}


// GLADIUS COIN EMITTER CONTRACT
pub mod gladius_coin_emitter{
    soroban_sdk::contractimport!(file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm");
    pub type GladiusCoinEmitterClient<'a> = Client<'a>;
}
use gladius_coin_emitter::GladiusCoinEmitterClient;

fn create_gladius_coin_emitter<'a>(
    e: & Env
) -> GladiusCoinEmitterClient<'a> {
    let gladius_coin_emitter_address = &e.register_contract_wasm(None, gladius_coin_emitter::WASM);
    let gladius_coin_emitter_client = GladiusCoinEmitterClient::new(e, gladius_coin_emitter_address);
    gladius_coin_emitter_client
}

// GLADIUS SUBSCRIPTIONS CONTRACT
pub mod gladius_subscriptions{
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/gladius_subscriptions.wasm");
    pub type GladiusSubscriptionsClient<'a> = Client<'a>;
}
use gladius_subscriptions::GladiusSubscriptionsClient;

fn create_gladius_subscriptions<'a>(
    e: & Env
) -> GladiusSubscriptionsClient<'a> {
    let gladius_subscriptions_address = &e.register_contract_wasm(None, gladius_subscriptions::WASM);
    let gladius_subscriptions_client = GladiusSubscriptionsClient::new(e, gladius_subscriptions_address);
    gladius_subscriptions_client
}


// THE TEST
pub struct GladiusSubscriptionsTest<'a> {
    e: Env,
    payment_token_admin: Address,
    gladius_admin: Address,
    parent_0: Address,
    parent_1: Address,
    club_0: Address,
    club_1: Address,
    student_0: Address,
    student_1: Address,
    payment_token: TokenClient<'a>,
    gladius_coin_emitter: GladiusCoinEmitterClient<'a>,
    contract: GladiusSubscriptionsClient<'a>,

}

impl<'a> GladiusSubscriptionsTest<'a> {
    fn setup() -> Self {

        let e = Env::default();
        e.mock_all_auths();

        // Addresses
        let payment_token_admin = Address::generate(&e);
        let gladius_admin = Address::generate(&e);
        let parent_0 = Address::generate(&e);
        let parent_1 = Address::generate(&e);
        let club_0 = Address::generate(&e);
        let club_1 = Address::generate(&e);
        let student_0 = Address::generate(&e);
        let student_1 = Address::generate(&e);

        // Contrats

        let payment_token = create_token_contract(&e);
        let name = String::from_str(&e, "EURC TOKEN");
        let symbol = String::from_str(&e, "EURC");
        let decimals = 7;
        payment_token.initialize(&payment_token_admin, &decimals, &name, &symbol);
        payment_token.mint(&parent_0, &123_000_000_000_000_000_000);
        payment_token.mint(&parent_1, &321_000_000_000_000_000_000);

        let contract = create_gladius_subscriptions(&e);

        let gladius_coin_emitter = create_gladius_coin_emitter(&e);
        let ratio: u32 = 1000;
        gladius_coin_emitter.initialize(
            &contract.address,
            &payment_token.address,
            &ratio
            );

        e.budget().reset_unlimited();  

        GladiusSubscriptionsTest {
            e,
            payment_token_admin,
            gladius_admin,
            parent_0,
            parent_1,
            club_0,
            club_1,
            student_0,
            student_1,
            payment_token,
            gladius_coin_emitter,
            contract,
        }
    }
}
           
// // mod initialize;
// // mod wrap;
// // mod unwrap;
// // mod gladius_coin;
