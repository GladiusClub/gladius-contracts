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
    
fn salt(e: &Env, club_address: &Address, club_name: &String ) -> BytesN<32> {
    let mut salt = Bytes::new(e);

    // Append the bytes of the address and name to the salt
    salt.append(&club_address.clone().to_xdr(e)); 
    salt.append(&club_name.clone().to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    e.crypto().sha256(&salt)
}

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env,                    // Pass in the current environment as an argument
    contract_wasm_hash: BytesN<32>, // Pass in the hash of contract's WASM file
    club_address: &Address,
    club_name: &String,
) -> Address {

    let salt: BytesN<32> = salt(&e, &club_address, &club_name);
    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(salt) // Use the salt as a unique identifier for the new contract instance
        .deploy(contract_wasm_hash) // Deploy the new contract instance using the given wasm hash value
}
