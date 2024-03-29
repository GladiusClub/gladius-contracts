use soroban_sdk::{Env, Address};
use crate::structs::Course;
use crate::storage_types::SubsDataKey;
use crate::error::GladiusSubscriptionsError;

pub fn write_total_courses(e: &Env, new_total_courses: u32) {
    e.storage().instance().set(&SubsDataKey::TotalCourses, &new_total_courses);
}

pub fn read_total_courses(e: &Env) -> u32 {
    let key = SubsDataKey::TotalCourses;
    if let Some(total_courses) = e.storage().instance().get(&key) {
        // e.storage()
        //     .instance()
        //     .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        total_courses
    } else {
        0
    }
}

pub fn write_course(e: &Env, course: Course, course_index: u32) {
    e.storage().persistent().set(&SubsDataKey::Course(course_index), &course);
}

pub fn read_course(e: &Env, course_index: u32) -> Result<Course, GladiusSubscriptionsError> { 
    let key = SubsDataKey::Course(course_index);
    if let Some(course) = e.storage().persistent().get(&key) {
        // e.storage()
        //     .instance()
        //     .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        Ok(course)
    } else {
        return Err(GladiusSubscriptionsError::CourseDoesNotExist)
    }
}

pub fn push_course(e: &Env, course: Course) -> u32 {
    let next_index = read_total_courses(&e);
    write_course(&e, course, next_index.clone());
    write_total_courses(&e, next_index.checked_add(1).unwrap());
    // Return the pushed course index
    next_index
}

pub fn course_has_student(course: &Course, student: &Address) -> bool {
    course.subscriptions.contains(&*student)
}
