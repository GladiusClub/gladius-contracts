#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod models;
mod storage_types;
mod admin;

use admin::{read_administrator, has_administrator, write_administrator};



pub trait GladiusCoinSubscriptionTrait {

    fn initialize(e: Env,
        admin: Address);
}

#[contract]
struct GladiusCoinSubscription;

#[contractimpl]
impl GladiusCoinSubscriptionTrait for GladiusCoinSubscription {

    
    fn initialize(e: Env,
        admin: Address) {
            
        // if has_administrator(&e) {
        //     return Err(GladiusCoinEmitSubscriptionError::InitializeAlreadyInitialized);
        // }

        write_administrator(&e, &admin);

        // event::initialize(&e, admin, pegged, ratio);
        // Ok(())
    }


}
