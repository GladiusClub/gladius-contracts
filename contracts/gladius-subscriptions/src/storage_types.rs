use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    SportClubs,
    IsSportClub(Address),
    IsStudent(Address),
    IsParent(Address),
}

// pub fn read_pegged_token(e: &Env) -> Address {
//     let key = GladiusDataKey::PeggedToken;
//     e.storage().instance().get(&key).unwrap()
// }

// pub fn write_pegged_token(e: &Env, id: &Address) {
//     let key = GladiusDataKey::PeggedToken;
//     e.storage().instance().set(&key, id);
// }

// pub fn read_ratio(e: &Env) -> u32 {
//     let key = GladiusDataKey::Ratio;
//     e.storage().instance().get(&key).unwrap()
// }

// pub fn write_ratio(e: &Env, id: &u32) {
//     let key = GladiusDataKey::Ratio;
//     e.storage().instance().set(&key, id);
// }