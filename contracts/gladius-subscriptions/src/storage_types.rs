use soroban_sdk::{contracttype, Address}; // Env

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;


#[derive(Clone)]
#[contracttype]
pub enum SubsDataKey {
    Admin,
    SportClubs,
    // Student(Address),
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

// Do we need to know what is the parent of a student?
// pub fn set_student(e: &Env, student: Student) {
//     e.storage().persistent().set(&SubsDataKey::Student(student.address.clone()), &student);
// }
// pub fn get_student(e: &Env, addr: Address) -> Student {
//     e.storage().persistent().get(&SubsDataKey::Student(addr)).unwrap()
// }