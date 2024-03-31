use soroban_sdk::{Env, String};

use crate::types::*;
use crate::storage::Storage;

pub fn get_token_uri(env: Env, token_id: u32) -> String {
    DatakeyMetadata::Uri(token_id)
        .get(&env)
        .unwrap_or_else(|| String::from_str(&env, "no uri"))
    
}