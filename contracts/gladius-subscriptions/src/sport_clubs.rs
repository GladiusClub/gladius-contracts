use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};


pub fn read_is_type(e: &Env, key: DataKey) -> bool {
    if let Some(is_type) = e.storage().persistent().get::<DataKey, bool>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        is_type
    } else {
        false
    }
}

pub fn write_is_type(e: &Env, key: DataKey, is_type: bool) {
    e.storage().persistent().set(&key, &is_type);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

// pub fn read_is_sport_club(e: &Env, addr: Address) -> bool {
//     let key = DataKey::IsSportClub(addr);
//     if let Some(is_sport_club) = e.storage().persistent().get::<DataKey, bool>(&key) {
//         e.storage()
//             .persistent()
//             .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
//         is_sport_club
//     } else {
//         false
//     }
// }

// pub fn write_is_sport_club(e: &Env, addr: Address, is_sport_club: bool) {
//     let key = DataKey::IsSportClub(addr);
//     e.storage().persistent().set(&key, &is_sport_club);
//     e.storage()
//         .persistent()
//         .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
// }


// pub fn read_is_parent(e: &Env, addr: Address) -> bool {
//     let key = DataKey::IsParent(addr);
//     if let Some(is_parent) = e.storage().persistent().get::<DataKey, bool>(&key) {
//         e.storage()
//             .persistent()
//             .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
//         is_parent
//     } else {
//         false
//     }
// }

// pub fn write_is_parent(e: &Env, addr: Address, is_parent: bool) {
//     let key = DataKey::IsParent(addr);
//     e.storage().persistent().set(&key, &is_parent);
//     e.storage()
//         .persistent()
//         .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
// }
