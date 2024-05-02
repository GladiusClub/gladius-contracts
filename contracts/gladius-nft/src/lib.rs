#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Map, String, Vec};

mod erc721traits;
mod types;
mod storage;
mod uri;
mod error;
mod event;

pub use crate::erc721traits::burnable::ERC721Burnable;
pub use crate::erc721traits::enumerable::ERC721Enumerable;
pub use crate::erc721traits::erc721::ERC721;
pub use crate::erc721traits::metadata::ERC721Metadata;
pub use crate::types::*;
pub use crate::storage::Storage;
pub use crate::uri::{get_token_uri, set_token_uri};

use error::GladiusNFTError;

mod test;

pub fn get_admin(env: &Env) -> Address {
    if let Some(addr) = Admin::User.get(env) {
        addr
    } else {
        panic_with_error!(env, Error::NotAuthorized)
    }
}

#[contract]
pub struct GladiusNFTContract;

#[contractimpl]
impl ERC721 for GladiusNFTContract {

    // Get the balance of a specific address.
    // 
    // This function retrieves the balance of a specific address (`owner`) in the context of the contract's state and functionality (`env`). 
    // The balance represents the number of non-fungible tokens (NFTs) owned by the specified address. If the address does not exist 
    // in the storage or if there is an issue retrieving the balance, it defaults to 0.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `owner`: The address for which to retrieve the balance.
    // 
    // Returns:
    // - `u32`: The balance of the specified address.
    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner)
            .extend(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }

    // Get the owner of a specific non-fungible token (NFT).
    // 
    // This function retrieves the owner of a specific non-fungible token (NFT) identified by its token ID (`token_id`) in the context 
    // of the contract's state and functionality (`env`). If the token does not exist or if there is an issue retrieving the owner, 
    // an error is returned.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `token_id`: The identifier of the non-fungible token (NFT) for which to retrieve the owner.
    // 
    // Returns:
    // - `Result<Address, GladiusNFTError>`: The address of the owner if the token exists, otherwise an error.
    fn owner_of(env: Env, token_id: u32) -> Result<Address, GladiusNFTError> {
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
                DataKey::Operator(from.clone(), spender.clone()).has(&env)
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
                event::transfer_from(&env, spender, from, to, token_id);
                Ok(())
            } else {
                return Err(GladiusNFTError::NotOwner);
            }
        } else {
            return Err(GladiusNFTError::NotNFT);
        }
    }
    
    // Approve an operator to manage a specific non-fungible token (NFT) on behalf of the caller.
    // 
    // This function allows the caller (either the owner of the token or an operator authorized by the owner) 
    // to approve another address (`operator`) to manage a specific non-fungible token (NFT) identified by its 
    // token ID (`token_id`). The approval can be temporary, with a specified time-to-live (TTL) value (`ttl`). 
    // If `operator` is `None`, any existing approval for the token is revoked. If the caller is not the owner 
    // of the token or an authorized operator, or if the token does not exist, an error is returned.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `caller`: The address initiating the approval.
    // - `operator`: The address to be approved as an operator, or `None` to revoke existing approval.
    // - `token_id`: The identifier of the non-fungible token (NFT) for which to manage approval.
    // - `ttl`: The time-to-live (TTL) value for the approval, indicating its duration.
    // 
    // Returns:
    // - `Ok(())`: If the approval operation is successful.
    // - `Err(GladiusNFTError)`: If the approval operation fails due to authorization issues or if the token does not exist.
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>, 
        token_id: u32, 
        ttl: u32) -> Result<(), GladiusNFTError> {

        // Check if token exists and caller is authorized to approve
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
        
        event::approve(&env, caller, token_id, ttl);

        // Perform approval or revoke based on operator presence
        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).extend(&env, ttl);
            Ok(())
        } else {
            DataKey::Approved(token_id).remove(&env);
            Ok(())
        }
    }

    // Set approval status for all tokens owned by a specific address.
    // 
    // This function allows the caller to set the approval status for all tokens owned by a specific address (`owner`) 
    // to be managed by another address (`operator`). If the caller is not the owner or an authorized operator of the 
    // tokens, an error is returned. The approval status can be either granted (`approved = true`) or revoked (`approved = false`), 
    // with an optional time-to-live (TTL) value (`ttl`) indicating the duration of the approval. 
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `caller`: The address initiating the approval status change.
    // - `owner`: The address owning the tokens for which to set the approval status.
    // - `operator`: The address to which the approval status for all tokens owned by `owner` will be set.
    // - `approved`: A boolean value indicating whether the approval status should be granted (`true`) or revoked (`false`).
    // - `ttl`: The time-to-live (TTL) value for the approval status, indicating its duration (optional).
    // 
    // Returns:
    // - `Ok(())`: If the approval status change is successful.
    // - `Err(GladiusNFTError)`: If the approval status change fails due to authorization issues.
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        ttl: u32,
    ) -> Result<(), GladiusNFTError> {
        // Check if caller is authorized to set approval status
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
        
        // Set or revoke approval status for all tokens
        let key = DataKey::Operator(owner.clone(), operator.clone());
        event::set_approval_for_all(&env, caller, owner, operator, approved, ttl);
        if approved {
            key.set(&env, &true);
            key.extend(&env, ttl);
            Ok(())
        } else {
            key.remove(&env);
            Ok(())
        }
    }

    // Get the address approved to manage a specific non-fungible token (NFT).
    // 
    // This function retrieves the address approved to manage a specific non-fungible token (NFT) identified by its 
    // token ID (`token_id`). If no address is approved or if the token does not exist, `None` is returned.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `token_id`: The identifier of the non-fungible token (NFT) for which to retrieve the approved address.
    // 
    // Returns:
    // - `Option<Address>`: The address approved to manage the token, or `None` if no address is approved.
    fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        DataKey::Approved(token_id).get(&env).unwrap_or(None)
    }

    // Check if an address is approved to manage all tokens owned by another address.
    // 
    // This function checks if an address (`operator`) is approved to manage all tokens owned by another address (`owner`). 
    // It returns `true` if the address is approved, and `false` otherwise.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `owner`: The address owning the tokens.
    // - `operator`: The address for which to check the approval status.
    // 
    // Returns:
    // - `bool`: A boolean value indicating whether the address is approved to manage all tokens owned by `owner`.
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        DataKey::Operator(owner, operator)
            .get(&env)
            .unwrap_or(false)
    }

}

#[contractimpl]
impl ERC721Metadata for GladiusNFTContract {
    // Get the name of the non-fungible token (NFT).
    // 
    // This function retrieves the name of the non-fungible token (NFT) as defined in the contract's metadata.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // 
    // Returns:
    // - `String`: The name of the non-fungible token (NFT).
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }

    // Get the symbol of the non-fungible token (NFT).
    // 
    // This function retrieves the symbol of the non-fungible token (NFT) as defined in the contract's metadata.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // 
    // Returns:
    // - `String`: The symbol of the non-fungible token (NFT).
    fn symbol(env: Env) -> String {
        DatakeyMetadata::Symbol.get(&env).unwrap()
    }

    // Get the URI of a specific non-fungible token (NFT).
    // 
    // This function retrieves the URI (Uniform Resource Identifier) of a specific non-fungible token (NFT) identified by its 
    // token ID (`token_id`) from the contract's metadata. If no URI is found for the token, it returns a default value.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `token_id`: The identifier of the non-fungible token (NFT) for which to retrieve the URI.
    // 
    // Returns:
    // - `String`: The URI of the specified non-fungible token (NFT).
    fn token_uri(env: Env, token_id: u32) -> String {
        DatakeyMetadata::Uri(token_id)
            .get(&env)
            .unwrap_or_else(|| String::from_str(&env, "no uri"))
    }

}


#[contractimpl]
impl ERC721Enumerable for GladiusNFTContract {
    // Get the total supply of non-fungible tokens (NFTs) in circulation.
    // 
    // This function retrieves the total supply of non-fungible tokens (NFTs) currently in circulation 
    // as stored in the contract's state and functionality (`env`).
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // 
    // Returns:
    // - `u32`: The total supply of non-fungible tokens (NFTs) in circulation.
    fn total_supply(env: Env) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .len()
    }

    // Get the token ID at a specific index in the contract's state.
    // 
    // This function retrieves the token ID at a specific index (`index`) in the list of owned token indices 
    // stored in the contract's state and functionality (`env`).
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `index`: The index of the token ID to retrieve.
    // 
    // Returns:
    // - `u32`: The token ID at the specified index.
    fn token_by_index(env: Env, index: u32) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }

    // Get the token ID of an owner's token at a specific index.
    // 
    // This function retrieves the token ID of an owner's token at a specific index (`index`) 
    // from the list of owned token IDs stored in the contract's state and functionality (`env`).
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `owner`: The address of the token owner.
    // - `index`: The index of the token ID to retrieve.
    // 
    // Returns:
    // - `u32`: The token ID of the owner's token at the specified index.
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
    // Initialize the contract with metadata and admin privileges.
    // 
    // This function initializes the contract with metadata including name and symbol, and assigns administrative 
    // privileges to the specified address (`admin`). It also sets up initial storage for token indices and mappings.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `admin`: The address to be assigned administrative privileges.
    // - `name`: The name of the non-fungible token (NFT).
    // - `symbol`: The symbol representing the non-fungible token (NFT).
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
        
        event::initialize(&env, admin, name, symbol);
    }

    // Get the address of the contract's administrator.
    // 
    // This function retrieves the address of the contract's administrator from the contract's state and functionality (`env`).
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // 
    // Returns:
    // - `Address`: The address of the contract's administrator.
    pub fn admin(env: Env) -> Address {
        get_admin(&env)
    }

    // Set the contract's administrator.
    // 
    // This function allows changing the address assigned as the contract's administrator.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `addr`: The new address to be assigned as the contract's administrator.
    pub fn set_admin(env: Env, addr: Address) {
        get_admin(&env).require_auth();
        Admin::User.set(&env, &addr);
        event::set_admin(&env, addr);
    }

    // Mint a new non-fungible token (NFT) and assign it to an owner.
    // 
    // This function mints a new non-fungible token (NFT) with the specified token ID (`token_id`) and URI (`uri`), 
    // and assigns it to the specified owner (`to`). Administrative privileges are required to execute this function.
    // 
    // Arguments:
    // - `env`: The environment containing the contract's state and functionality.
    // - `to`: The address to which the newly minted token will be assigned.
    // - `token_id`: The identifier of the newly minted token.
    // - `uri`: The URI (Uniform Resource Identifier) associated with the newly minted token.
    pub fn mint(env: Env, to: Address, token_id: u32, uri: String) {
        get_admin(&env).require_auth();

        if !DataKey::TokenOwner(token_id).has(&env) {
            DataKey::TokenOwner(token_id).set(&env, &to);
            DatakeyMetadata::Uri(token_id).set(&env, &uri);

            let mut owned_token_indices: Vec<u32> = DataKeyEnumerable::OwnedTokenIndices.get(&env).unwrap();
            let mut token_id_to_index_map: Map<u32, u32> =
                DataKeyEnumerable::TokenIdToIndex.get(&env).unwrap();

            token_id_to_index_map.set(token_id, owned_token_indices.len());
            owned_token_indices.push_back(token_id);

            let mut owner_token_indices: Vec<u32> = DataKeyEnumerable::OwnerOwnedTokenIds(to.clone())
                .get(&env)
                .unwrap_or_else(|| Vec::new(&env)); 
            owner_token_indices.push_back(token_id);

            DataKeyEnumerable::OwnedTokenIndices.set(&env, &owned_token_indices);
            DataKeyEnumerable::TokenIdToIndex.set(&env, &token_id_to_index_map);
            DataKeyEnumerable::OwnerOwnedTokenIds(to.clone()).set(&env, &owner_token_indices);

            DataKey::Balance(to.clone()).set(&env, &owner_token_indices.len());
        } else {
            panic!("Token already exists")
        }
        event::mint(&env, to, token_id, uri);
    }
}
