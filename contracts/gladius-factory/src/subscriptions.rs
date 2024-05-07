// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{contracttype, contracterror, xdr::ToXdr, Address, Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "../gladius-subscriptions/target/wasm32-unknown-unknown/release/gladius_subscriptions.wasm"
);
