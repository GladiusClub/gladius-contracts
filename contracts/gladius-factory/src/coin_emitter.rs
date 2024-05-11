// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{contracttype, contracterror, xdr::ToXdr, Address, Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm"
);
