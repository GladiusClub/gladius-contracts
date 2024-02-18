#![no_std]
use soroban_sdk::{contract, contractimpl}; 

mod gladius_coin;

pub trait GladiusCoinEmitterTrait {

}

#[contract]
struct GladiusCoinEmitter;

#[contractimpl]
impl GladiusCoinEmitterTrait for GladiusCoinEmitter {
}
