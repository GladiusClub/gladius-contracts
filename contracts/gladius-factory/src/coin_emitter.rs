// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{
    contracttype, 
    contracterror, 
    xdr::ToXdr, 
    Address, 
    Bytes, 
    BytesN, 
    Env, 
    String};

soroban_sdk::contractimport!(
    file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm"
);

#[contracttype]
#[derive(Clone)]
pub struct Club(Address, String);
impl Club {
    pub fn new(a: Address, b: String) -> Self {
        Club(a, b)
    }

    pub fn salt(&self, e: &Env) -> BytesN<32> {
        let mut salt = Bytes::new(e);

        // Append the bytes of the address and name to the salt
        salt.append(&self.0.clone().to_xdr(e)); 
        salt.append(&self.1.clone().to_xdr(e));

        // Hash the salt using SHA256 to generate a new BytesN<32> value
        e.crypto().sha256(&salt)
    }

    pub fn address(&self) -> &Address {
        &self.0
    }

    pub fn name(&self) -> &String {
        &self.1
    }
}

// Define a function to create a new contract instance
pub fn create_contract(
    /*
        Overall, this function is designed to create a new contract
        instance on the blockchain with the given pair_wasm_hash
        value and a unique salt value generated from the token_a and
        token_b values. The salt value is used to ensure that each
        contract instance is unique and can be identified by its hash value.

        The deployer() method of the Env instance is used to actually
        create and deploy the new contract instance. The function returns
        the hash value of the newly created contract instance as a
        BytesN<32> value.
    */
    e: &Env,                    // Pass in the current environment as an argument
    pair_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    club: &Club,
) -> Address {
    // Return the hash of the newly created contract as a Address value

    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(club.salt(&e)) // Use the salt as a unique identifier for the new contract instance
        .deploy(pair_wasm_hash) // Deploy the new contract instance using the given pair_wasm_hash value
}
