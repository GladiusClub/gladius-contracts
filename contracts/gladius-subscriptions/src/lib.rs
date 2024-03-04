#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, vec, String};
use soroban_sdk::token::Client as TokenClient;


mod models;
mod storage_types;
mod admin;
mod sport_clubs;
mod gladius_coin_emitter;


use gladius_coin_emitter::GladiusCoinEmitterClient;

use admin::{read_administrator, has_administrator, write_administrator};
use sport_clubs::{write_is_type, read_is_type};
use storage_types::{
    DataKey,
    // COURSES
    read_course,
    write_course,
    push_course,
    // TOKEN
    write_token, 
    read_token,
    // COIN EMITTER  
    read_gladius_coin_emitter,
    write_gladius_coin_emitter};
use models::{Course};



pub trait GladiusCoinSubscriptionTrait {

    fn initialize(
        e: Env,
        admin: Address,
        token: Address,
        gladius_coin_emitter: Address);

    // Admin Functions
    fn set_is_sport_club(e: Env, addr: Address, is: bool);
    fn set_is_parent(e: Env, addr: Address, is: bool);
    fn set_is_student(e: Env, addr: Address, is: bool);

    // Sport Clubs Functions
    fn create_course(
        e: Env, 
        sport_club: Address, 
        amount: i128, 
        prizes_amount: i128,
        title: String) -> u32;
    
    fn distribute_gladius_coins(
        e: Env,
        course_index: u32,
        student: Address,
        amount: i128);

    // Parents Functions
    fn subscribe_course(e:Env, parent_address: Address, student_address: Address, course_index: u32);

    fn is_sport_club(e:Env, addr: Address) -> bool;
    fn is_parent(e:Env, addr: Address) -> bool;
    fn is_student(e:Env, addr: Address) -> bool;

    fn get_token(e:Env) -> Address;
    fn get_gladius_coin_emitter(e:Env) -> Address;
    fn get_course(e: Env, course_index: u32) -> Course;
}

#[contract]
struct GladiusCoinSubscription;

#[contractimpl]
impl GladiusCoinSubscriptionTrait for GladiusCoinSubscription {

    
    fn initialize(
        e: Env,
        admin: Address,
        token: Address,
        gladius_coin_emitter: Address) {
            
        // if has_administrator(&e) {
        //     return Err(GladiusCoinEmitSubscriptionError::InitializeAlreadyInitialized);
        // }

        write_administrator(&e, &admin);
        write_token(&e, &token);
        write_gladius_coin_emitter(&e, &gladius_coin_emitter);

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
        incentive: i128,
        title: String) -> u32 {
        
        // This must be called by the sport_club itself
        sport_club.require_auth();
        // Fail if caller is not a sport club
        if Self::is_sport_club(e.clone(), sport_club.clone()) {
            panic!("Not a Sport Club");
        }

        let new_course = Course {
            club: sport_club,
            price: price,
            incentive: incentive,
            subscriptions:vec![&e,].into(),
            title: title,
            active: true,
            gladius_coin_balance: 0
        };
        // push_course function returns the course index
        push_course(&e, new_course)
        // Event of pushed course and index
    }

    fn distribute_gladius_coins(
        e: Env,
        course_index: u32,
        student: Address,
        amount: i128) {
        
        // This function can only be called by the sport club
        let mut course = read_course(&e, course_index);
        course.club.require_auth();

        // Student should exist in the course
        // TODO: Find a way more efficient to do it.
        if !(course.subscriptions.contains(student.clone())) {
            panic!("Student does not exist in that Course");
        }

        //Course should have enough Gladius Coin balance:
        if amount > course.gladius_coin_balance{
            panic!("Course does not have enought Gladius Coin balance");
        }
        // Update course balance before sending gladius coins (good practice)
        course.gladius_coin_balance = course.gladius_coin_balance.checked_sub(amount).unwrap();
        write_course(&e, course, course_index);

        // Send Gladius Coins to Student
        // The GladiusCoinEmitter it's the Gladius Coin Token Contract itself
        let gladius_coin_client = GladiusCoinEmitterClient::new(&e, &read_gladius_coin_emitter(&e));
        // from this contract to student
        gladius_coin_client.transfer(&e.current_contract_address(), &student, &amount);

        // TODO: Emit event





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
        let mut course = read_course(&e, course_index);
        let total_amount: i128 = course.price.checked_add(course.incentive).unwrap();
        
        let token_client = TokenClient::new(&e, &read_token(&e));
        // Parent sends total_amount (EURC) to this contrat
        // Function will fail if parent does not have total_amount
        token_client.transfer(&parent_address, &e.current_contract_address(), &total_amount);
        // This contrat sends course.price (EUR) to Sport Club
        token_client.transfer(&e.current_contract_address(), &course.club, &course.price);

        
        // This contract triggers the wrap_and_mint funtion in the Gladius Coin Emitter Contract
        let gladius_coin_emitter_client = GladiusCoinEmitterClient::new(&e, &read_gladius_coin_emitter(&e));
        //fn wrap_and_mint(e: Env, to: Address, amount: i128)
        let minted_amount = gladius_coin_emitter_client.wrap_and_mint(
            &e.current_contract_address(), // to
            &course.incentive // amount
        );
        // We assign the minted amount to the Course.
        course.gladius_coin_balance = course.gladius_coin_balance.checked_add(minted_amount).unwrap();
        // Add student to course // Can we do it better?
        course.subscriptions.push_back(student_address);

        // Save the course.
        write_course(&e, course, course_index);
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
    fn get_token(e:Env) -> Address {
        read_token(&e)

    }
    fn get_gladius_coin_emitter(e:Env) -> Address {
        read_gladius_coin_emitter(&e)
    }
    fn get_course(e: Env, course_index: u32) -> Course {
        read_course(&e, course_index)
    }

}
