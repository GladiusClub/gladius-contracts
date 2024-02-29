#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod models;
mod storage_types;
mod admin;
mod sport_clubs;

use admin::{read_administrator, has_administrator, write_administrator};
use sport_clubs::{write_is_type, read_is_type};
use storage_types::{DataKey};



pub trait GladiusCoinSubscriptionTrait {

    fn initialize(e: Env, admin: Address);
    fn set_is_sport_club(e: Env, addr: Address, is: bool);
    fn set_is_parent(e: Env, addr: Address, is: bool);
    fn set_is_student(e: Env, addr: Address, is: bool);
    fn is_sport_club(e:Env, addr: Address) -> bool;
    fn is_parent(e:Env, addr: Address) -> bool;
    fn is_student(e:Env, addr: Address) -> bool;
}

#[contract]
struct GladiusCoinSubscription;

#[contractimpl]
impl GladiusCoinSubscriptionTrait for GladiusCoinSubscription {

    
    fn initialize(e: Env, admin: Address) {
            
        // if has_administrator(&e) {
        //     return Err(GladiusCoinEmitSubscriptionError::InitializeAlreadyInitialized);
        // }

        write_administrator(&e, &admin);

        // event::initialize(&e, admin, pegged, ratio);
        // Ok(())
    }

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
