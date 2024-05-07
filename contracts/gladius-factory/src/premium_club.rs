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
    String
};

#[contracttype]
#[derive(Clone)]
pub struct PremiumClub(pub Address, pub String);
impl PremiumClub {
    pub fn new(a: Address, b: String)  -> Self {
        PremiumClub(a, b)
    }

    pub fn salt(&self, e: &Env) -> BytesN<32> {
        let mut salt = Bytes::new(e);

        // Append the bytes of token_0 and token_1 to the salt
        salt.append(&self.0.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
        salt.append(&self.1.clone().to_xdr(e));

        // Hash the salt using SHA256 to generate a new BytesN<32> value
        e.crypto().sha256(&salt)
    }

    pub fn club_address(&self) -> &Address {
        &self.0
    }

    pub fn club_name(&self) -> &String {
        &self.1
    }
}

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env,                    // Pass in the current environment as an argument
    contract_wasm_hash: BytesN<32>, // Pass in the hash of contract's WASM file
    premium_club: &PremiumClub,
) -> Address {

    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(premium_club.salt(&e)) // Use the salt as a unique identifier for the new contract instance
        .deploy(contract_wasm_hash) // Deploy the new contract instance using the given wasm hash value
}
