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

    pub fn club_address(&self) -> &Address {
        &self.0
    }

    pub fn club_name(&self) -> &String {
        &self.1
    }
}

fn salt(
    e: &Env, 
    club_address: &Address, 
    club_name: &String, 
    contract_type: &String
) -> BytesN<32> {
    let mut salt = Bytes::new(e);

    // Append the bytes of the address and name to the salt
    salt.append(&club_address.clone().to_xdr(e)); 
    salt.append(&club_name.clone().to_xdr(e));
    salt.append(&contract_type.clone().to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    e.crypto().sha256(&salt)
}


// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env,                    // Pass in the current environment as an argument
    contract_wasm_hash: BytesN<32>, // Pass in the hash of contract's WASM file
    premium_club: &PremiumClub,
    contract_type: &String,
) -> Address {

    let salt: BytesN<32> = salt(
        &e,
        &premium_club.club_address(),
        &premium_club.club_name(), 
        &contract_type);

    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(salt) // Use the salt as a unique identifier for the new contract instance
        .deploy(contract_wasm_hash) // Deploy the new contract instance using the given wasm hash value
}


#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub struct PremiumClubAddresses(
    pub Address,
    pub Address,
    pub Address
);
impl PremiumClubAddresses {
    pub fn new(a: Address, b: Address, c: Address)  -> Self {
        PremiumClubAddresses(a, b, c)
    }
    pub fn coin_emitter(&self) -> &Address {
        &self.0
    }

    pub fn subscriptions(&self) -> &Address {
        &self.1
    }

    pub fn nft(&self) -> &Address {
        &self.2
    }

}
