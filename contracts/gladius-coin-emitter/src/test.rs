#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _},
    Address, 
    Env,
    String,
};

// TOKEN TO BE PEGGED (EURC)
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
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm");
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

// THE TEST
pub struct GladiusCoinEmitterTest<'a> {
    env: Env,
    minter: Address,
    user: Address,
    pegged_token: TokenClient<'a>,
    contract: GladiusCoinEmitterClient<'a>,
}

impl<'a> GladiusCoinEmitterTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let user = Address::generate(&env);
        let pegged_token_admin = Address::generate(&env); 
        let minter = Address::generate(&env);
        let pegged_token = create_token_contract(&env);

        let name = String::from_str(&env, "EURC TOKEN");
        let symbol = String::from_str(&env, "EURC");
        let decimals = 7;

        pegged_token.initialize(&pegged_token_admin, &decimals, &name, &symbol);
        pegged_token.mint(&minter, &123_000_000_000_000_000_000);
        pegged_token.mint(&user, &321_000_000_000_000_000_000);

        let contract = create_gladius_coin_emitter(
            &env,
        );

        env.budget().reset_unlimited();
    

        GladiusCoinEmitterTest {
            env,
            minter,
            user,
            pegged_token,
            contract,
        }
    }
}
           
mod initialize;
mod wrap;
mod unwrap;
mod gladius_coin;
