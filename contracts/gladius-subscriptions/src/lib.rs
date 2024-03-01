#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, vec};
use soroban_sdk::token::Client as TokenClient;


mod models;
mod storage_types;
mod admin;
mod sport_clubs;

use admin::{read_administrator, has_administrator, write_administrator};
use sport_clubs::{write_is_type, read_is_type};
use storage_types::{DataKey, push_course, write_token, read_token,  get_course};
use models::{Course};



pub trait GladiusCoinSubscriptionTrait {

    fn initialize(e: Env, admin: Address, token: Address);

    // Admin Functions
    fn set_is_sport_club(e: Env, addr: Address, is: bool);
    fn set_is_parent(e: Env, addr: Address, is: bool);
    fn set_is_student(e: Env, addr: Address, is: bool);

    // Sport Clubs Functions
    fn create_course(e: Env, sport_club: Address, amount: i128, prizes_amount: i128);

    fn subscribe_course(e:Env, parent_address: Address, student_address: Address, course_index: u32);

    fn is_sport_club(e:Env, addr: Address) -> bool;
    fn is_parent(e:Env, addr: Address) -> bool;
    fn is_student(e:Env, addr: Address) -> bool;
}

#[contract]
struct GladiusCoinSubscription;

#[contractimpl]
impl GladiusCoinSubscriptionTrait for GladiusCoinSubscription {

    
    fn initialize(
        e: Env,
        admin: Address,
        token: Address) {
            
        // if has_administrator(&e) {
        //     return Err(GladiusCoinEmitSubscriptionError::InitializeAlreadyInitialized);
        // }

        write_administrator(&e, &admin);
        write_token(&e, &admin);

        // event::initialize(&e, admin, pegged, ratio);
        // Ok(())
    }

    // Admin Functions
    fn set_is_sport_club(e: Env, addr: Address, is: bool){
        let admin = read_administrator(&e);
        admin.require_auth();
        let key = DataKey::IsSportClub(addr.clone());
        write_is_type(&e, key, is);
    }
    fn set_is_parent(e: Env, addr: Address, is: bool){
        let admin = read_administrator(&e);
        admin.require_auth();
        let key = DataKey::IsParent(addr.clone());
        write_is_type(&e, key, is);
    }
    fn set_is_student(e: Env, addr: Address, is: bool){
        let admin = read_administrator(&e);
        admin.require_auth();
        let key = DataKey::IsStudent(addr.clone());
        write_is_type(&e, key, is);
    }

    // Sport Clubs Functions
    fn create_course(e: Env,
        sport_club: Address,
        price: i128,
        incentive: i128) {
        sport_club.require_auth();
        if Self::is_sport_club(e.clone(), sport_club.clone()) {
            panic!("Not a Sport Club");
        }
        let new_course = Course {
            club: sport_club,
            price: price,
            incentive: incentive,
            subscriptions:vec![&e,].into(),
            active: true
        };
        push_course(&e, new_course); // TODO: Function should return index of pushed course
        // Event of pushed course and index
    }

    // Parents Functions
    fn subscribe_course(
        e:Env,
        parent_address: Address,
        student_address: Address,
        course_index: u32) {

        parent_address.require_auth();
        // TODO: check if parent is parent of student

        // get course
        let mut course = get_course(&e, course_index);
        let total_amount: i128 = course.price.checked_add(course.incentive).unwrap();

        // Function will fail if parent does not have total_amount
        TokenClient::new(&e,
            &read_token(&e))
            .transfer(&parent_address, &e.current_contract_address(), &total_amount);

        // Add student to course
        course.subscriptions.push_back(student_address);
        // Create Subscription
        // subscribe.push_back(env.current_contract_address().clone());
        // public event

    }
    
    
    
    fn is_sport_club(e:Env, addr: Address) -> bool {
        let key = DataKey::IsSportClub(addr.clone());
        read_is_type(&e, key)
    }

    fn is_parent(e:Env, addr: Address) -> bool {
        let key = DataKey::IsParent(addr.clone());
        read_is_type(&e, key)
    }

    fn is_student(e:Env, addr: Address) -> bool {
        let key = DataKey::IsStudent(addr.clone());
        read_is_type(&e, key)
    }

}
