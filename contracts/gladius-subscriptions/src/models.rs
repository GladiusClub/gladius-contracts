use soroban_sdk::{contracttype, Vec, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SportClub {
    pub index: i32,
    pub address: Address,
    pub allowed: bool,
} 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parent {
    pub index: i32,
    pub address: Address,
    pub students: Vec<i32>
} 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Student {
    pub index: i32,
    pub address: Address,
    pub parent: i32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub club: Address,
    pub price: i128,
    pub k: i128,
    pub subscriptions: Vec<Student>,
    pub active: bool
}