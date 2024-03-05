use soroban_sdk::{Env, Address};
use crate::storage_types::DataKey;

pub fn read_payment_token(e: &Env) -> Address {
    let key = DataKey::PaymentToken;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_payment_token(e: &Env, id: &Address) {
    let key = DataKey::PaymentToken;
    e.storage().instance().set(&key, id);
}