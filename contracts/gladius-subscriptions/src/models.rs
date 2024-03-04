use soroban_sdk::{contracttype, Vec, Address, String};

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
    pub students: Vec<Address>
} 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Student {
    // pub index: i32,
    pub address: Address,
    pub parent: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub club: Address,
    pub price: i128,
    pub incentive: i128,
    pub subscriptions: Vec<Address>,
    pub title: String,
    pub active: bool
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub course_index: i32,
    pub student: Address,
    pub deadline: u32,
}
