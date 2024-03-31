#![no_std]

mod erc721traits;
mod types;
mod storage;
mod uri;

pub use crate::erc721traits::burnable::ERC721Burnable;
pub use crate::erc721traits::enumerable::ERC721Enumerable;
pub use crate::erc721traits::erc721::ERC721;
pub use crate::erc721traits::metadata::ERC721Metadata;
pub use crate::types::*;
pub use crate::storage::Storage;
pub use crate::uri::{get_token_uri, set_token_uri};

use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, IntoVal, Map, String, Val, Vec};

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

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        spender.require_auth();
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
        if !is_sender_approved {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) { 
            if addr == from {
                if from != to {
                    // update enumerable datai
                    let from_index_key = DataKeyEnumerable::OwnerOwnedTokenIndices(from.clone());
                    let from_token_key = DataKeyEnumerable::OwnerTokenIndex(from.clone());
                    let to_index_key = DataKeyEnumerable::OwnerOwnedTokenIndices(to.clone());
                    let to_token_key = DataKeyEnumerable::OwnerTokenIndex(to.clone());
                    let mut from_index: Vec<u32> =
                        from_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
                    let mut from_token: Map<u32, u32> =
                        from_token_key.get(&env).unwrap_or_else(|| Map::new(&env));
                    let mut to_index: Vec<u32> =
                        to_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
                    let mut to_token: Map<u32, u32> =
                        to_token_key.get(&env).unwrap_or_else(|| Map::new(&env));

                    // Remove token from index of from address
                    from_index.remove(from_token.get(token_id).unwrap());
                    from_token.remove(token_id);

                    // Remove token from index of to address
                    to_token.set(token_id, to_index.len());
                    to_index.push_back(token_id);

                    // Update from address vec and map
                    from_index_key.set(&env, &from_index);
                    from_token_key.set(&env, &from_token);
                    DataKey::Balance(from.clone()).set(&env, &from_index.len());

                    // Update to address vec and map
                    to_token_key.set(&env, &to_token);
                    to_index_key.set(&env, &to_index);
                    DataKey::Balance(to.clone()).set(&env, &to_index.len());
                }
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NotOwner);
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }
    fn approve(env: Env, caller: Address, operator: Option<Address>, token_id: u32, ttl: u32) {
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
            panic_with_error!(&env, Error::NotNFT);
        }
        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).extend(&env, ttl);
        } else {
            DataKey::Approved(token_id).remove(&env);
        }
    }
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        ttl: u32,
    ) {
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        let key = DataKey::Operator(owner, operator);
        if approved {
            key.set(&env, &true);
            key.extend(&env, ttl);
        } else {
            key.remove(&env);
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
        DataKeyEnumerable::OwnerOwnedTokenIndices(owner)
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

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) { 
            addr
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
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
            // A vector containing indices of tokens owned by a specific address:
            let mut owner_token_indices: Vec<u32> = DataKeyEnumerable::OwnerOwnedTokenIndices(to.clone())
                .get(&env)
                .unwrap_or_else(|| Vec::new(&env)); 

            // A map linking token IDs to their indices for a specific address.
            let mut owner_token_index: Map<u32, u32> =
                DataKeyEnumerable::OwnerTokenIndex(to.clone())
                    .get(&env)
                    .unwrap_or_else(|| Map::new(&env));

            // We set the current token_id with its corresponding index
            token_id_to_index_map.set(token_id, owned_token_indices.len());

            // We push the current created token index to the vetor containing indices of tokens owned
            owned_token_indices.push_back(token_id);

            owner_token_index.set(token_id, owner_token_indices.len());
            owner_token_indices.push_back(token_id);

            DataKeyEnumerable::OwnedTokenIndices.set(&env, &owned_token_indices);
            DataKeyEnumerable::TokenIdToIndex.set(&env, &token_id_to_index_map);
            DataKeyEnumerable::OwnerOwnedTokenIndices(to.clone()).set(&env, &owner_token_indices);
            DataKeyEnumerable::OwnerTokenIndex(to.clone()).set(&env, &owner_token_index);

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



// Gladius NFT wont be burnable

// #[contractimpl]
// impl ERC721Burnable for GladiusNFTContract {
//     fn burn(env: Env, caller: Address, token_id: u32) {
//         let owner: Address = DataKey::TokenOwner(token_id)
//             .get(&env)
//             .unwrap_or_else(|| panic_with_error!(&env, Error::NotNFT));
//         if owner == caller {
//             owner.require_auth();
//         } else if DataKey::Operator(owner.clone(), caller.clone())
//             .get::<bool>(&env)
//             .unwrap_or(false)
//         {
//             caller.require_auth();
//         } else {
//             panic_with_error!(&env, Error::NotAuthorized);
//         }

//         DataKey::Approved(token_id).remove(&env);
//         DataKey::TokenOwner(token_id).remove(&env);

    
//         let mut owned_token_indices: Vec<u32> = DataKeyEnumerable::OwnedTokenIndices.get(&env).unwrap();
//         let mut token_id_to_index_map: Map<u32, u32> =
//             DataKeyEnumerable::TokenIdToIndex.get(&env).unwrap();
//         let from_index_key = DataKeyEnumerable::OwnerOwnedTokenIndices(owner.clone());
//         let from_token_key = DataKeyEnumerable::OwnerTokenIndex(owner.clone());

//         let mut from_index: Vec<u32> =
//             from_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
//         let mut from_token: Map<u32, u32> =
//             from_token_key.get(&env).unwrap_or_else(|| Map::new(&env));

//         from_index.remove(from_token.get(token_id).unwrap());
//         from_token.remove(token_id);
//         owned_token_indices.remove(token_id_to_index_map.get(token_id).unwrap());
//         token_id_to_index_map.remove(token_id);

//         from_index_key.set(&env, &from_index);
//         from_token_key.set(&env, &from_token);
//         DataKeyEnumerable::OwnedTokenIndices.set(&env, &owned_token_indices);
//         DataKeyEnumerable::TokenIdToIndex.set(&env, &token_id_to_index_map);

//         DataKey::Balance(owner).set(&env, &from_index.len());
        
//         let v: Val = token_id.into();
//         Event::Burn.publish(&env, v);
//     }
// }

