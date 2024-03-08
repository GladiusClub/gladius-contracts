use soroban_sdk::{contracttype, symbol_short, Env, Address};
use crate::structs::Course;


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeEvent {
    pub admin: Address,
    pub payment_token: Address,
    pub gladius_coin_emitter: Address
}

pub(crate) fn initialized(
    e: &Env, 
    admin: Address,
    payment_token: Address,
    gladius_coin_emitter: Address) {
    
    let event: InitializeEvent = InitializeEvent {
        admin: admin,
        payment_token: payment_token,
        gladius_coin_emitter: gladius_coin_emitter,
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("init")), event);
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetSportClubEvent {
    pub sport_club: Address,
    pub set: bool
}

pub(crate) fn sport_club_set(
    e: &Env, 
    sport_club: Address,
    set: bool) {
    
    let event: SetSportClubEvent = SetSportClubEvent {
        sport_club: sport_club,
        set: set
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("club_set")), event);
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetParentEvent {
    pub parent: Address,
    pub set: bool
}

pub(crate) fn parent_set(
    e: &Env, 
    parent: Address,
    set: bool) {
    
    let event: SetParentEvent = SetParentEvent {
        parent: parent,
        set: set
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("paren_set")), event);
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetStudentEvent {
    pub student: Address,
    pub set: bool
}

pub(crate) fn student_set(
    e: &Env, 
    student: Address,
    set: bool) {
    
    let event: SetStudentEvent = SetStudentEvent {
        student: student,
        set: set
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("stud_set")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateCourseEvent {
    pub course_index: u32,
    pub course: Course,
}

pub(crate) fn course_created(
    e: &Env,
    course_index: u32,
    course: Course) {
    
    let event: CreateCourseEvent = CreateCourseEvent {
        course_index: course_index,
        course: course
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("course_cr")), event);
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributeGladiusCoinsEvent {
    pub course_index: u32,
    pub student: Address,
    pub amount: i128,
    pub new_course_balance: i128
}

pub(crate) fn gladius_coins_distributed(
    e: &Env,
    course_index: u32,
    student: Address,
    amount: i128,
    new_course_balance: i128) {
    
    let event: DistributeGladiusCoinsEvent = DistributeGladiusCoinsEvent {
        course_index: course_index,
        student: student,
        amount: amount,
        new_course_balance: new_course_balance
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("coin_dist")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscribeCourseEvent {
    pub course_index: u32,
    pub parent: Address,
    pub student: Address,
    pub sport_club: Address,
    pub price: i128,
    pub incentive: i128,
    pub new_course_balance: i128
}

pub(crate) fn course_subscribed(
    e: &Env,
    course_index: u32,
    parent: Address,
    student: Address,
    sport_club: Address,
    price: i128,
    incentive: i128,
    new_course_balance: i128) {
    
    let event: SubscribeCourseEvent = SubscribeCourseEvent {
        course_index: course_index,
        parent: parent,
        student: student,
        sport_club: sport_club,
        price: price,
        incentive: incentive,
        new_course_balance: new_course_balance
    };
    e.events().publish(("GladiusSubscriptions", symbol_short!("course_sb")), event);
}
