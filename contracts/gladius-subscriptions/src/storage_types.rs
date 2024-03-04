use soroban_sdk::{contracttype, Address, Env};
use crate::models::{Course, Student};

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
    Token,
    GladiusCoinEmitter
}


// TOTAL COURSES - INSTANCE STORAGE
pub fn get_total_courses(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalCourses).unwrap()
}
pub fn set_total_courses(e: &Env, new_total_courses: u32) {
    e.storage().instance().set(&DataKey::TotalCourses, &new_total_courses);
}

// COURSES - Each Course in an independent persistent storage
pub fn write_course(e: &Env, course: Course, course_index: u32) {
    e.storage().persistent().set(&DataKey::Course(course_index), &course);
}
pub fn read_course(e: &Env, course_index: u32) -> Course {
    e.storage().persistent().get(&DataKey::Course(course_index)).unwrap()
}
pub fn exist_course(e: &Env, course_index: u32) -> bool {
    e.storage().persistent().has(&DataKey::Course(course_index))
}
pub fn push_course(e: &Env, course: Course) -> u32{
    let next_index = get_total_courses(&e);
    write_course(&e, course, next_index.clone());
    set_total_courses(&e, next_index.checked_add(1).unwrap());
    // Return the pushed couse index
    next_index
}
pub fn desactivate_course(e: &Env, course_index: u32) {
    let mut course = read_course(&e, course_index.clone());
    course.active = false;
    write_course(&e, course, course_index);
}


// TOKEN

pub fn read_token(e: &Env) -> Address {
    let key = DataKey::Token;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_token(e: &Env, id: &Address) {
    let key = DataKey::Token;
    e.storage().instance().set(&key, id);
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