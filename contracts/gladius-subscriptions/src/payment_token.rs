use soroban_sdk::{Env, Address};
use crate::storage_types::SubsDataKey;

pub fn read_payment_token(e: &Env) -> Address {
    let key = SubsDataKey::PaymentToken;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_payment_token(e: &Env, id: &Address) {
    let key = SubsDataKey::PaymentToken;
    e.storage().instance().set(&key, id);
}