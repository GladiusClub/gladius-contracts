use soroban_sdk::{Address, Env};
use crate::storage_types::SubsDataKey;
use crate::structs::Subscription;


// Subscription - Each Course in an independent persistent storage
pub fn set_course(e: &Env, course: Course, course_index: u32) {
    e.storage().persistent().set(&SubsDataKey::Course(course_index), &course);
}
pub fn get_course(e: &Env, course_index: u32) -> Course {
    e.storage().persistent().get(&SubsDataKey::Course(course_index)).unwrap()
}
pub fn exist_course(e: &Env, course_index: u32) -> bool {
    e.storage().persistent().has(&SubsDataKey::Course(course_index))
}
pub fn push_course(e: &Env, course: Course) {
    let next_index = read_total_courses(&e);
    set_course(&e, course, next_index.clone());
    write_total_courses(&e, next_index.checked_add(1).unwrap());
}
pub fn desactivate_course(e: &Env, course_index: u32) {
    let mut course = get_course(&e, course_index.clone());
    course.active = false;
    set_course(&e, course, course_index);
}