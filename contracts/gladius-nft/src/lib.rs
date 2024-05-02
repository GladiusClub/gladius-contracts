#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, IntoVal, Map, String, Val, Vec};

mod erc721traits;
mod types;
mod storage;
mod uri;
mod error;

pub use crate::erc721traits::burnable::ERC721Burnable;
pub use crate::erc721traits::enumerable::ERC721Enumerable;
pub use crate::erc721traits::erc721::ERC721;
pub use crate::erc721traits::metadata::ERC721Metadata;
pub use crate::types::*;
pub use crate::storage::Storage;
pub use crate::uri::{get_token_uri, set_token_uri};

use error::GladiusNFTError;



mod test;

#[contract]
pub struct GladiusNFTContract;

#[contractimpl]
impl ERC721 for GladiusNFTContract {

    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner)
            .extend(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }
    fn owner_of(env: Env,token_id: u32) -> Result<Address, GladiusNFTError> {
        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) { 
            Ok(addr)
        } else {
            return Err(GladiusNFTError::NotNFT);
        }
    }

    // Transfer a non-fungible token (NFT) from one address to another, with optional approval mechanism.
    // 
    // This function facilitates the transfer of a specific non-fungible token (NFT) from one address (`from`) to another address (`to`). 
    // The transfer can only be initiated by an authorized spender. If the sender (`from`) is not the spender (`spender`), 
    // the sender must be either approved to transfer the token or must have operator privileges. If the sender is the spender, 
    // they are assumed to be authorized. Upon successful transfer, the token's ownership is updated, and its balance is adjusted 
    // accordingly for both the sender and the receiver. If the token is not owned by the sender or does not exist, an error is returned.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `spender`: The address initiating the transfer, who must be authorized to transfer the token.
    // - `from`: The address currently owning the token to be transferred.
    // - `to`: The address to which the token will be transferred.
    // - `token_id`: The identifier of the specific token to be transferred.
    // 
    // Returns:
    // - `Ok(())`: If the transfer is successful.
    // - `Err(GladiusNFTError)`: If the transfer fails due to authorization issues, ownership issues, or if the token does not exist.


    fn transfer_from(
        env: Env, 
        spender: Address, 
        from: Address, 
        to: Address, 
        token_id: u32) -> Result<(), GladiusNFTError> {
    
        // Ensure spender is authorized to transfer
        spender.require_auth();
    
        // Check if sender is approved to transfer token
        let is_sender_approved = if spender != from {
            let has_approved =
                if let Some(approved) = DataKey::Approved(token_id).get::<Address>(&env) {
                    // Clear the approval on transfer
                    DataKey::Approved(token_id).remove(&env);
                    approved == spender
                } else {
                    false
                };
            if !has_approved {
                DataKey::Operator(from.clone(), spender).has(&env)
            } else {
                true
            }
        } else {
            true
        };
    
        // If sender is not approved, return error
        if !is_sender_approved {
            return Err(GladiusNFTError::NotAuthorized);
        }
    
        // Check if token exists and sender is owner
        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) { 
            if addr == from {
                if from != to {
                    // Fetch token IDs owned by 'from' and 'to'
                    let from_owned_token_ids_key = DataKeyEnumerable::OwnerOwnedTokenIds(from.clone());
                    let to_owned_token_ids_key = DataKeyEnumerable::OwnerOwnedTokenIds(to.clone());
    
                    // Get owned token IDs or initialize empty Vec if none
                    let mut from_owned_token_ids: Vec<u32> =
                        from_owned_token_ids_key.get(&env).unwrap_or_else(|| Vec::new(&env));
                    let mut to_owned_token_ids: Vec<u32> =
                        to_owned_token_ids_key.get(&env).unwrap_or_else(|| Vec::new(&env));
    
                    // Remove token ID from 'from' and add to 'to'
                    if let Some(index) = from_owned_token_ids.iter().position(|x| x == token_id) {
                        from_owned_token_ids.remove(index.try_into().unwrap());
                    }
                    to_owned_token_ids.push_back(token_id); 
    
                    // Update storage with new token IDs and balances
                    from_owned_token_ids_key.set(&env, &from_owned_token_ids);
                    to_owned_token_ids_key.set(&env, &to_owned_token_ids);
                    DataKey::Balance(from.clone()).set(&env, &from_owned_token_ids.len());                    
                    DataKey::Balance(to.clone()).set(&env, &to_owned_token_ids.len());
                }
                // Update token owner
                DataKey::TokenOwner(token_id).set(&env, &to);
                Ok(())
            } else {
                return Err(GladiusNFTError::NotOwner);
            }
        } else {
            return Err(GladiusNFTError::NotNFT);
        }
    }
    
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>, 
        token_id: u32, 
        ttl: u32) -> Result<(), GladiusNFTError> {

        if let Some(owner) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if owner == caller {
                owner.require_auth();
            } else if DataKey::Operator(owner, caller.clone())
                .get::<bool>(&env)
                .unwrap_or(false)
            {
                caller.require_auth();
            }
        } else {
            return Err(GladiusNFTError::NotNFT);
        }
        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).extend(&env, ttl);
            Ok(())
        } else {
            DataKey::Approved(token_id).remove(&env);
            Ok(())
        }
    }
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        ttl: u32,
    ) -> Result<(), GladiusNFTError> {
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            return Err(GladiusNFTError::NotAuthorized);
        }
        let key = DataKey::Operator(owner, operator);
        if approved {
            key.set(&env, &true);
            key.extend(&env, ttl);
            Ok(())
        } else {
            key.remove(&env);
            Ok(())
        }
    }
    fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        DataKey::Approved(token_id).get(&env).unwrap_or(None)
    }
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        DataKey::Operator(owner, operator)
            .get(&env)
            .unwrap_or(false)
    }
}

#[contractimpl]
impl ERC721Metadata for GladiusNFTContract {
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }
    fn symbol(env: Env) -> String {
        DatakeyMetadata::Symbol.get(&env).unwrap()
    }
    fn token_uri(env: Env, token_id: u32) -> String {
        DatakeyMetadata::Uri(token_id)
        .get(&env)
        .unwrap_or_else(|| String::from_str(&env, "no uri"))
    }
}


#[contractimpl]
impl ERC721Enumerable for GladiusNFTContract {
    fn total_supply(env: Env) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .len()
    }
    fn token_by_index(env: Env, index: u32) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }
    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32 {
        DataKeyEnumerable::OwnerOwnedTokenIds(owner)
            .get::<Vec<u32>>(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }
}

#[contractimpl]
impl GladiusNFTContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
    ) {
        if Admin::User.has(&env) {
            panic!("Already initialized")
        }
        Admin::User.set(&env, &admin);

        env.storage().instance().extend_ttl(10000, 10000);
    
        env.storage().instance().set(&DatakeyMetadata::Name, &name);
        env.storage()
            .instance()
            .set(&DatakeyMetadata::Symbol, &symbol);


        DataKeyEnumerable::OwnedTokenIndices.set(&env, &Vec::<u32>::new(&env));
        DataKeyEnumerable::TokenIdToIndex.set(&env, &Map::<u32, u32>::new(&env));
        // todo: events
    }

    pub fn admin(env: Env) -> Address {
        get_admin(&env)
    }

    pub fn set_admin(env: Env, addr: Address) {
        get_admin(&env).require_auth();
        Admin::User.set(&env, &addr);
        // TODO: Set set_admin event
    }

    pub fn mint(env: Env, to: Address, token_id: u32, uri: String) {
        get_admin(&env).require_auth();

        if !DataKey::TokenOwner(token_id).has(&env) {
            DataKey::TokenOwner(token_id).set(&env, &to);
            DatakeyMetadata::Uri(token_id).set(&env, &uri);
        
            // A vector containing indices of tokens owned.
            let mut owned_token_indices: Vec<u32> = DataKeyEnumerable::OwnedTokenIndices.get(&env).unwrap();

            // A map linking token IDs to their indices
            let mut token_id_to_index_map: Map<u32, u32> =
                DataKeyEnumerable::TokenIdToIndex.get(&env).unwrap();

            // Related to an especific owner:
            // A vector containing ids of tokens owned by a specific address:
            let mut owner_token_indices: Vec<u32> = DataKeyEnumerable::OwnerOwnedTokenIds(to.clone())
                .get(&env)
                .unwrap_or_else(|| Vec::new(&env)); 

            // We set the current token_id with its corresponding index
            token_id_to_index_map.set(token_id, owned_token_indices.len());

            // We push the current created token index to the vetor containing indices of tokens owned
            owned_token_indices.push_back(token_id);

            owner_token_indices.push_back(token_id);

            DataKeyEnumerable::OwnedTokenIndices.set(&env, &owned_token_indices);
            DataKeyEnumerable::TokenIdToIndex.set(&env, &token_id_to_index_map);
            DataKeyEnumerable::OwnerOwnedTokenIds(to.clone()).set(&env, &owner_token_indices);

            DataKey::Balance(to.clone()).set(&env, &owner_token_indices.len());
        } else {
            panic!("Token already exist")
        }
        let mut v: Vec<Val> = Vec::new(&env);
        v.push_back(to.into_val(&env));
        v.push_back(token_id.into());
        Event::Mint.publish(&env, v);
    }
}

pub fn get_admin(env: &Env) -> Address {
    if let Some(addr) = Admin::User.get(env) {
        addr
    } else {
        panic_with_error!(env, Error::NotAuthorized)
    }
}