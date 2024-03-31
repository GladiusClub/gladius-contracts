#![cfg(test)]
extern crate std;

use soroban_sdk::{
    testutils::{Address as _},
    Address, 
    Env,
};

// GLADIUS TOKEN CONTRACT
pub mod gladius_nft{
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/gladius_nft.wasm");
    pub type GladiusNFTClient<'a> = Client<'a>;
}
use gladius_nft::GladiusNFTClient;

fn create_gladius_nft<'a>(
    e: & Env
) -> GladiusNFTClient<'a> {
    let gladius_nft_address = &e.register_contract_wasm(None, gladius_nft::WASM);
    let gladius_nft_client = GladiusNFTClient::new(e, gladius_nft_address);
    gladius_nft_client
}

// THE TEST
pub struct GladiusNFTTest<'a> {
    env: Env,
    admin: Address,
    user: Address,
    contract: GladiusNFTClient<'a>,
}

impl<'a> GladiusNFTTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let user = Address::generate(&env);
        let admin = Address::generate(&env); 

        let contract = create_gladius_nft(
            &env,
        );

        env.budget().reset_unlimited();
    

        GladiusNFTTest {
            env,
            admin,
            user,
            contract,
        }
    }
}

mod initialize;
mod admin;
mod mint;
mod transfer;