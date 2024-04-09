use soroban_sdk::{Address, Env};
use crate::error::GladiusNFTError;

pub trait ERC721 {
    fn balance_of(env: Env, owner: Address) -> u32;
    fn owner_of(env: Env, token_id: u32) -> Result<Address, GladiusNFTError>;
    fn transfer_from(
        env: Env, 
        spender: Address, 
        from: Address, 
        to: Address, 
        token_id: u32) -> Result<(), GladiusNFTError>;
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) -> Result<(), GladiusNFTError>;
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    )-> Result<(), GladiusNFTError>;
    fn get_approved(env: Env, token_id: u32) -> Option<Address>;
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool;
}
