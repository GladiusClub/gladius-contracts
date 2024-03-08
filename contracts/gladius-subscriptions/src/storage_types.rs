use soroban_sdk::{contracttype, Address}; // Env

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
// pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
// pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;


#[derive(Clone)]
#[contracttype]
pub enum SubsDataKey {
    Admin,
    SportClubs,
    IsSportClub(Address),
    IsStudent(Address),
    IsParent(Address),
    TotalCourses,
    Course(u32),
    TotalSubscriptions,
    Subscription(u32),
    PaymentToken,
    GladiusCoinEmitter
}
