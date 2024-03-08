use soroban_sdk::Env;
use crate::structs::Student;
use crate::storage_types::SubsDataKey;

pub fn set_student(e: &Env, student: Student) {
    e.storage().persistent().set(&SubsDataKey::Student(student.address.clone()), &student);
}
pub fn get_student(e: &Env, addr: Address) -> Student {
    e.storage().persistent().get(&SubsDataKey::Student(addr)).unwrap()
}