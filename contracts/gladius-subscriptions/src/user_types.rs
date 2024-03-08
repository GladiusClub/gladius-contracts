use soroban_sdk::Env;

use crate::storage_types::{
    SubsDataKey, 
    PERSISTENT_BUMP_AMOUNT, 
    PERSISTENT_LIFETIME_THRESHOLD};

pub fn read_is_type(e: &Env, key: SubsDataKey) -> bool {
    if let Some(is_type) = e.storage().persistent().get::<SubsDataKey, bool>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        is_type
    } else {
        false
    }
}

pub fn write_is_type(e: &Env, key: SubsDataKey, is_type: bool) {
    e.storage().persistent().set(&key, &is_type);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}