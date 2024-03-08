# Gladius Smart Contracts

## The Liquidity Pool: Gladius Coin Token Emitter
This contract emits Gladius Coins at a rate of `v` for each unit of `EURC` received. It functions similarly to the [WETH Wrapped Contract](https://etherscan.io/address/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2#code).

### Interface:
```rust
/// Initializes the Gladius Coin smart contract with provided parameters.
    ///
    /// This function sets up the Gladius Coin smart contract by initializing its metadata, including
    /// name, symbol, and decimal places. It also sets the administrator, pegged token address, and
    /// the ratio. If the contract has already been initialized, it returns an error indicating that
    /// initialization has already occurred.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `admin` - The address of the administrator to be set.
    /// * `pegged` - The address of the pegged token.
    /// * `ratio` - The ratio of conversion between Gladius Coin and the pegged token.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the initialization is successful, otherwise returns an error of type
    /// `GladiusCoinEmitterError`.
    fn initialize(e: Env,
        admin: Address, 
        pegged: Address,
        ratio: u32) -> Result<(), GladiusCoinEmitterError>;

    /// Wraps and mints Gladius Coins for the specified recipient.
    ///
    /// This function wraps a specified amount of the pegged token into Gladius Coins and mints them
    /// for the designated recipient. It first checks if the contract has been initialized and ensures
    /// that the wrap amount is non-negative. It then transfers the specified amount of the pegged
    /// token from the minter to the contract, locking it. After that, it calculates the amount of Gladius
    /// Coins to mint based on the wrapping ratio and mints them for the recipient. Finally, it emits
    /// a `wrap` event to notify listeners about the wrapping and minting operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `to` - The address of the recipient to whom the minted Gladius Coins will be sent.
    /// * `wrap_amount` - The amount of the pegged token to wrap and mint into Gladius Coins.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the wrapping and minting operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
    fn wrap_and_mint(
        e: Env, 
        to: Address, 
        amount: i128) -> Result<(), GladiusCoinEmitterError>;

    /// Unwraps and burns Gladius Coins, converting them back to the pegged token.
    ///
    /// This function unwraps a specified amount of Gladius Coins, converting them back to the
    /// pegged token, and burns them. It first checks if the contract has been initialized and ensures
    /// that the unwrap amount is non-negative. It then verifies the caller's authentication and sends
    /// back the unwrapped amount of the pegged token to the caller. After that, it calculates the
    /// amount of Gladius Coins to burn based on the wrapping ratio and burns them. Finally, it emits
    /// an `unwrap` event to notify listeners about the unwrapping and burning operation.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    /// * `from` - The address of the sender who wants to unwrap and burn Gladius Coins.
    /// * `unwrap_amount` - The amount of Gladius Coins to unwrap and burn.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the unwrapping and burning operation is successful, otherwise returns an
    /// error of type `GladiusCoinEmitterError`.
    fn unwrap_and_burn(
        e: Env, 
        from: Address, 
        amount: i128) -> Result<(), GladiusCoinEmitterError>;

    /// Retrieves the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// This function reads and returns the current wrapping ratio of Gladius Coins to the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the current wrapping ratio as a `u32`.
    fn ratio(e: Env)  -> u32;

    /// Retrieves the address of the pegged token.
    ///
    /// This function reads and returns the address of the pegged token.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the pegged token as an `Address`.
    fn pegged(e: Env)  -> Address;

    /// Retrieves the address of the minter (administrator) of Gladius Coins.
    ///
    /// This function reads and returns the address of the minter (administrator) of Gladius Coins.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment instance providing access to the blockchain state.
    ///
    /// # Returns
    ///
    /// Returns the address of the minter (administrator) as an `Address`.
    fn minter(e: Env)  -> Address;
```

This contract also behaves as the Gladius Coin token contract.

## The Gladius Subscription and Token Distribution Smart Contract

- Sport Clubs can open sport subscriptions at a fixed price in EURC.
Parents can subscribe a Student and pay the price `p+k` for a monthly subscription.
- The Contract splits p directly to Sport Clubs and `k` to the Gladius Coin Token Emitter Contract. In exchange, the contract receives Gladius Coins.
- The contract holds Gladius Coins on behalf of Sport Clubs.
- The contract maintains a list of Sport Clubs, Parents, Students, and Subscriptions with their expiration date.
- Sport Clubs can distribute these Gladius Coins only to Students who have been subscribed.
- Sport Clubs can also distribute these Gladius Coins to some NFT contract so physically redeemable NFTs have economic value.

Interface:
```rust
sport_club_subscription(from: Address, sport_club: Address):
    // Gladius Admin can add new sport clubs

parent_subscription(from: Address, parent: Address):
    // Gladius Admin can add new parents

create_subscription(from: Address, price: u128, name: String, description: String):
    // Sport Clubs can create a Sport Monthly Subscription at a price.

subscribe(from: Address, student: Address, subscription: String, periods: u128):
    // Allowed Parents can subscribe their students
    // Parents should have sent the price
    // Function splits the price p and sends it directly to the Sport Club address
    // Function splits the amount k and calls the `mint` function in the Gladius Coin Token Emitter Contract
    // The contract ends up holding Gladius Coins
    // Function assigns the minted Gladius Coins to the Sport Club (but does not send Gladius Coin to the Sport Club)... the contract holds them on behalf of the Club.

distribute(from: Address, amount: u128, student: Address):
    // Sport Clubs can distribute Gladius Coins to Students
    // Function checks if the Sport Club has some Gladius Coin balance
    // Function checks that the student has an active subscription with this Sport Club

mint_nft(from: Address, value: u128, student: Address, name: String, description: String):
    // Sport Clubs can mint economically valuable NFTs
    // Function checks if the Sport Club has some Gladius Coin balance
    // Function checks that the student has an active subscription with this Sport Club
    // Function calls the NFT Contract to mint an NFT with a locked amount of Gladius Coins associated

```

## Economic Valuable NFT
Contract that implements the ERC721 Ethereum Standard in Soroban but only allow Sport Clubs to mint NFTs.

Minted NFTs should have a locked amount of Gladius Coins that can be unlocked and sent back to Sport Clubs once the NFT has been redeemed for physical objects.

NFTs can only be minted by the Gladius Subscription and Token Distribution Smart Contract.

Interface:
```rust
fn mint(from: Address, to: Address, value: u64){
    // Function that can only be called by the Gladius Subscription contract
    // The caller should send value amount of Gladius Coins
    // NFT is minted to the destination user.
    // The NFT contract locks the Gladius Coins until the final user redeems it for physical goods
}
fn redeem(from: Address, nft: u64){
    // Final user (Students) can redeem a minted NFT for a physical good
    // Contract checks that the user has the NFT
    // Contract checks if the NFT has already been redeemed (boolean redeemed flag)
    // Contract changes redeemed to true and sends the locked amount of Gladius Coins to the Sport Club that emitted the NFT.
}

```