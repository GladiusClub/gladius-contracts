use soroban_sdk::Env;
use crate::structs::Course;
use crate::storage_types::SubsDataKey;

pub fn write_total_courses(e: &Env, new_total_courses: u32) {
    e.storage().instance().set(&SubsDataKey::TotalCourses, &new_total_courses);
}

pub fn read_total_courses(e: &Env) -> u32 {
    e.storage().instance().get(&SubsDataKey::TotalCourses).unwrap()
}

pub fn write_course(e: &Env, course: Course, course_index: u32) {
    e.storage().persistent().set(&SubsDataKey::Course(course_index), &course);
}

pub fn read_course(e: &Env, course_index: u32) -> Course {
    e.storage().persistent().get(&SubsDataKey::Course(course_index)).unwrap()
}

pub fn push_course(e: &Env, course: Course) -> u32 {
    let next_index = read_total_courses(&e);
    write_course(&e, course, next_index.clone());
    write_total_courses(&e, next_index.checked_add(1).unwrap());
    // Return the pushed course index
    next_index
}

// pub fn deactivate_course(e: &Env, course_index: u32) {
    //     let mut course = read_course(&e, course_index.clone());
    //     course.active = false;
    //     write_course(&e, course, course_index);
    // }
    
// pub fn exist_course(e: &Env, course_index: u32) -> bool {
//     e.storage().persistent().has(&SubsDataKey::Course(course_index))
// }