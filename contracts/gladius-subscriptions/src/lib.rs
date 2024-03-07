#![no_std]

// Import necessary items
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
        contract, contractimpl, vec, Address, Env, IntoVal, Symbol,
        String};
use soroban_sdk::token::Client as TokenClient;

// Import modules
mod structs;
mod storage_types;
mod admin;
mod payment_token;
mod courses;
mod user_types;
mod gladius_coin_emitter;
mod test;

// Import specific items from modules
use gladius_coin_emitter::{GladiusCoinEmitterClient, write_gladius_coin_emitter, read_gladius_coin_emitter};
use admin::{read_administrator, has_administrator, write_administrator};
use user_types::{write_is_type, read_is_type};
use courses::{read_course, write_course, push_course, read_total_courses, course_has_student};
use payment_token::{write_payment_token, read_payment_token};
use storage_types::SubsDataKey;
use structs::Course;

pub fn check_initialized(e: &Env) {
    if !has_administrator(&e) {
        panic!("Not yet initialized");
    }
}

pub fn check_positive_amount(amount: i128) {
    if amount<=0 {
        panic!("Amount should be positive");
    }
}

pub fn check_sport_club(e: &Env, addr: &Address) {
     if !GladiusSubscriptions::is_sport_club(e.clone(), addr.clone()) {
        panic!("Not a Sport Club");
    }
}

pub fn check_parent(e: &Env, addr: &Address) {
    if !GladiusSubscriptions::is_parent(e.clone(), addr.clone()) {
       panic!("Not a Parent");
   }
}

pub fn check_student(e: &Env, addr: &Address) {
    if !GladiusSubscriptions::is_student(e.clone(), addr.clone()) {
       panic!("Not a Student");
   }
}

pub trait GladiusSubscriptionsTrait {

    /// Initializes the contract with administrator, token, and Gladius coin emitter addresses.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `admin` - The address of the administrator.
    /// * `token` - The address of the token.
    /// * `gladius_coin_emitter` - The address of the Gladius coin emitter.
    fn initialize(
        e: Env,
        admin: Address,
        token: Address,
        gladius_coin_emitter: Address);

    // Admin Functions

    // TODO: Write change admin function

    /// Sets the status of whether an address is a sport club or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a sport club.
    fn set_is_sport_club(e: Env, addr: Address, is: bool);

    /// Sets the status of whether an address is a parent or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a parent.
    fn set_is_parent(e: Env, addr: Address, is: bool);

    /// Sets the status of whether an address is a student or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a student.
    fn set_is_student(e: Env, addr: Address, is: bool);

    // Sport Clubs Functions

    /// Creates a new course and returns its index.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `sport_club` - The address of the sport club creating the course.
    /// * `price` - The price of the course.
    /// * `incentive` - The incentive for the course.
    /// * `title` - The title of the course.
    ///
    /// # Returns
    ///
    /// The index of the newly created course.
    fn create_course(
        e: Env, 
        sport_club: Address, 
        amount: i128, 
        prizes_amount: i128,
        title: String) -> u32;
    
    /// Distributes Gladius Coins to students enrolled in the specified course.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `course_index` - The index of the course.
    /// * `student` - The address of the student to receive Gladius Coins.
    /// * `amount` - The amount of Gladius Coins to distribute.
    fn distribute_gladius_coins(
        e: Env,
        course_index: u32,
        student: Address,
        amount: i128);
    
    // TODO: Add function to create NFT when NFT contract is ready.

    // Parents Functions

    /// Subscribes a student to a course and handles payment and token transfer.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `parent` - The address of the parent.
    /// * `student` - The address of the student.
    /// * `course_index` - The index of the course to subscribe to.
    fn subscribe_course(e:Env, parent: Address, student: Address, course_index: u32);

    fn is_sport_club(e:Env, addr: Address) -> bool;
    fn is_parent(e:Env, addr: Address) -> bool;
    fn is_student(e:Env, addr: Address) -> bool;

    fn get_admin(e:Env) -> Address;
    fn get_token(e:Env) -> Address;
    fn get_gladius_coin_emitter(e:Env) -> Address;
    fn get_course(e: Env, course_index: u32) -> Course;
    fn get_total_courses(e: Env) -> u32;
}

#[contract]
struct GladiusSubscriptions;

#[contractimpl]
impl GladiusSubscriptionsTrait for GladiusSubscriptions {

    
    /// Initializes the contract with administrator, token, and Gladius coin emitter addresses.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `admin` - The address of the administrator.
    /// * `token` - The address of the token.
    /// * `gladius_coin_emitter` - The address of the Gladius coin emitter.
    fn initialize(
        e: Env,
        admin: Address,
        token: Address,
        gladius_coin_emitter: Address,
    ) {
        // Check if already initialized
        if has_administrator(&e) {
            // TODO: Transform in Error
            panic!("Already Initialized");
        }

        // Write administrator, token, and Gladius coin emitter addresses
        write_administrator(&e, &admin);
        write_payment_token(&e, &token);
        write_gladius_coin_emitter(&e, &gladius_coin_emitter);

        // TODO: Add event
    }

    // Admin Functions

    /// Sets the status of whether an address is a sport club or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a sport club.
    fn set_is_sport_club(e: Env, addr: Address, is: bool) {
        check_initialized(&e);
        // Ensure that the caller is the administrator
        let admin = read_administrator(&e);
        admin.require_auth();
        
        // Set the status of whether the address is a sport club
        let key = SubsDataKey::IsSportClub(addr.clone());
        write_is_type(&e, key, is);
    }

    /// Sets the status of whether an address is a parent or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a parent.
    fn set_is_parent(e: Env, addr: Address, is: bool) {
        check_initialized(&e);
        // Ensure that the caller is the administrator
        let admin = read_administrator(&e);
        admin.require_auth();
        
        // Set the status of whether the address is a parent
        let key = SubsDataKey::IsParent(addr.clone());
        write_is_type(&e, key, is);
    }

    /// Sets the status of whether an address is a student or not.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `addr` - The address to set the status for.
    /// * `is` - The boolean value indicating whether the address is a student.
    fn set_is_student(e: Env, addr: Address, is: bool) {
        check_initialized(&e);
        // Ensure that the caller is the administrator
        let admin = read_administrator(&e);
        admin.require_auth();
        
        // Set the status of whether the address is a student
        let key = SubsDataKey::IsStudent(addr.clone());
        write_is_type(&e, key, is);
    }


    // Sport Clubs Functions

    /// Creates a new course and returns its index.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `sport_club` - The address of the sport club creating the course.
    /// * `price` - The price of the course.
    /// * `incentive` - The incentive for the course.
    /// * `title` - The title of the course.
    ///
    /// # Returns
    ///
    /// The index of the newly created course.
    fn create_course(
        e: Env,
        sport_club: Address,
        price: i128,
        incentive: i128,
        title: String,
    ) -> u32 {
        check_initialized(&e);
        check_sport_club(&e, &sport_club);
        check_positive_amount(price);
        check_positive_amount(incentive);
        
        // Ensure that the caller is the sport club itself
        sport_club.require_auth();
        
        // Create a new course
        let new_course = Course {
            club: sport_club,
            price,
            incentive,
            subscriptions: vec![&e].into(),
            title,
            active: true,
            gladius_coin_balance: 0,
        };

        // Push the course and return its index
        push_course(&e, new_course)
        // TODO: Emit event of pushed course and index
    }

    /// Distributes Gladius Coins to students enrolled in the specified course.
    /// 
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `course_index` - The index of the course.
    /// * `student` - The address of the student to receive Gladius Coins.
    /// * `amount` - The amount of Gladius Coins to distribute.
    fn distribute_gladius_coins(
        e: Env,
        course_index: u32,
        student: Address,
        amount: i128,
    ) {
        check_initialized(&e);
        // Ensure that the caller is the sport club
        let mut course = read_course(&e, course_index);
        course.club.require_auth();

        // Ensure that the student exists in the course
        if !course.subscriptions.contains(student.clone()) {
            panic!("Student does not exist in that Course");
        }

        // Ensure that the course has enough Gladius Coin balance
        if amount > course.gladius_coin_balance {
            panic!("Course does not have enough Gladius Coin balance");
        }

        // Update course balance before sending Gladius Coins
        course.gladius_coin_balance = course.gladius_coin_balance.checked_sub(amount).unwrap();
        write_course(&e, course, course_index);

        // Send Gladius Coins to the student
        let gladius_coin_client = GladiusCoinEmitterClient::new(&e, &read_gladius_coin_emitter(&e));
        gladius_coin_client.transfer(&e.current_contract_address(), &student, &amount);

        // TODO: Emit event
    }


    // Parents Functions

    /// Subscribes a student to a course and handles payment and token transfer.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `parent` - The address of the parent.
    /// * `student` - The address of the student.
    /// * `course_index` - The index of the course to subscribe to.
    fn subscribe_course(
        e: Env,
        parent: Address,
        student: Address,
        course_index: u32,
    ) {
        parent.require_auth();
        check_initialized(&e);
        check_parent(&e, &parent);
        check_student(&e, &student);
        
        // TODO: Check if parent is the parent of the student (not implemented)
        // TODO: Check that parent cannot subscribe same student again 
        
        // Get the course // TODO: Add error if course does not exist
        let mut course = read_course(&e, course_index);
        // let hehas = course.subscriptions.
        if course_has_student(&e, &course, &student) {
            panic!("Student already exist");
        }
        
        // Calculate the total amount required for subscription
        let total_amount = course.price.checked_add(course.incentive).expect("Overflow when calculating total amount");

        // Initialize the token client
        let token_client = TokenClient::new(&e, &read_payment_token(&e));

        // Parent sends the total amount (EURC) to this contract
        // Function will fail if parent does not have enough tokens
        token_client.transfer(&parent, &e.current_contract_address(), &total_amount);

        // This contract sends course.price (EUR) to the sport club
        token_client.transfer(&e.current_contract_address(), &course.club, &course.price);

        // This contract triggers the `wrap_and_mint` function in the Gladius Coin Emitter Contract
        let gladius_coin_emitter_address = read_gladius_coin_emitter(&e);
        let gladius_coin_emitter_client = GladiusCoinEmitterClient::new(&e, &gladius_coin_emitter_address);
        
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: read_payment_token(&e).clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: (
                        e.current_contract_address(),
                        gladius_coin_emitter_address,
                        course.incentive
                    ).into_val(&e),
                },
                sub_invocations: vec![&e]
            })
        ]);
        
        // Wrap and mint the incentive amount
        let minted_amount = gladius_coin_emitter_client.wrap_and_mint(
            &e.current_contract_address(), // to
            &course.incentive // amount
        );

        // Assign the minted amount to the course
        course.gladius_coin_balance = course.gladius_coin_balance.checked_add(minted_amount).expect("Overflow when updating course balance");

        // Add the student to the course subscriptions
        course.subscriptions.push_back(student);

        // Save the updated course
        write_course(&e, course, course_index);
    }
    
    fn is_sport_club(e:Env, addr: Address) -> bool {
        let key = SubsDataKey::IsSportClub(addr.clone());
        read_is_type(&e, key)
    }

    fn is_parent(e:Env, addr: Address) -> bool {
        let key = SubsDataKey::IsParent(addr.clone());
        read_is_type(&e, key)
    }

    fn is_student(e:Env, addr: Address) -> bool {
        let key = SubsDataKey::IsStudent(addr.clone());
        read_is_type(&e, key)
    }
    
    fn get_course(e: Env, course_index: u32) -> Course {
        read_course(&e, course_index)
    }

    fn get_total_courses(e: Env) -> u32 {
        read_total_courses(&e)
    }

    // BASIC INFO
    fn get_admin(e:Env) -> Address {
        check_initialized(&e);
        read_administrator(&e)
    }

    fn get_token(e:Env) -> Address {
        check_initialized(&e);
        read_payment_token(&e)
    }

    fn get_gladius_coin_emitter(e:Env) -> Address {
        check_initialized(&e);
        read_gladius_coin_emitter(&e)
    }

}
