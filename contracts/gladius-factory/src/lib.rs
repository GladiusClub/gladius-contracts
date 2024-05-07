#![no_std]
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, vec,
    Address, Env, IntoVal, Symbol, String};
use soroban_sdk::token::Client as TokenClient;

// Import modules
mod coin_emitter;