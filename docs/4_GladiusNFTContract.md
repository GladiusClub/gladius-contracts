## Economic Valuable NFT Contrat
Contract that implements the ERC721 Ethereum Standard in Soroban but only allow Sport Clubs to mint NFTs.

Minted NFTs should have a locked amount of Gladius Coins that can be unlocked and sent back to Sport Clubs once the NFT has been redeemed for physical objects.

NFTs can only be minted by the Gladius Subscription and Token Distribution Smart Contract.

Interface:
```rust
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
fn balance_of(env: Env, owner: Address) -> u32;

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
fn owner_of(env: Env, token_id: u32) -> Result<Address, GladiusNFTError>;


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
    token_id: u32) -> Result<(), GladiusNFTError>;

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
    ttl: u32) -> Result<(), GladiusNFTError>;

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
) -> Result<(), GladiusNFTError>;

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
fn get_approved(env: Env, token_id: u32) -> Option<Address>;

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
fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool;

// Get the name of the non-fungible token (NFT).
// 
// This function retrieves the name of the non-fungible token (NFT) as defined in the contract's metadata.
// 
// Arguments:
// - `env`: The environment containing the contract's state and functionality.
// 
// Returns:
// - `String`: The name of the non-fungible token (NFT).
fn name(env: Env) -> String;

// Get the symbol of the non-fungible token (NFT).
// 
// This function retrieves the symbol of the non-fungible token (NFT) as defined in the contract's metadata.
// 
// Arguments:
// - `env`: The environment containing the contract's state and functionality.
// 
// Returns:
// - `String`: The symbol of the non-fungible token (NFT).
fn symbol(env: Env) -> String;

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
fn token_uri(env: Env, token_id: u32) -> String;

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
fn total_supply(env: Env) -> u32;

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
fn token_by_index(env: Env, index: u32) -> u32;

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
fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32;

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
);

// Get the address of the contract's administrator.
// 
// This function retrieves the address of the contract's administrator from the contract's state and functionality (`env`).
// 
// Arguments:
// - `env`: The environment containing the contract's state and functionality.
// 
// Returns:
// - `Address`: The address of the contract's administrator.
pub fn admin(env: Env) -> Address;

// Set the contract's administrator.
// 
// This function allows changing the address assigned as the contract's administrator.
// 
// Arguments:
// - `env`: The environment containing the contract's state and functionality.
// - `addr`: The new address to be assigned as the contract's administrator.
pub fn set_admin(env: Env, addr: Address);

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
pub fn mint(env: Env, to: Address, token_id: u32, uri: String);
```