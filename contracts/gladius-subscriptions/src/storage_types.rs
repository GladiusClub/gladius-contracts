use soroban_sdk::{contracttype, Address, Env};
use crate::structs::{Course, Student};

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
    Student(Address),
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






// TOKEN

pub fn read_gladius_coin_emitter(e: &Env) -> Address {
    let key = DataKey::GladiusCoinEmitter;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_gladius_coin_emitter(e: &Env, id: &Address) {
    let key = DataKey::GladiusCoinEmitter;
    e.storage().instance().set(&key, id);
}



pub fn set_student(e: &Env, student: Student) {
    e.storage().persistent().set(&DataKey::Student(student.address.clone()), &student);
}
pub fn get_student(e: &Env, addr: Address) -> Student {
    e.storage().persistent().get(&DataKey::Student(addr)).unwrap()
}