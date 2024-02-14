# Gladius Smart Contracts

# Gladius Smart Contracts

## The Liquidity Pool: Gladius Coin Token Emitter
This contract emits Gladius Coins at a rate of `v` for each unit of `EURC` received. It functions similarly to the [WETH Wrapped Contract](https://etherscan.io/address/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2#code).

### Interface:
```rust
mint(amount):
    // Requires that the user sends amount/v in EURC
    // Locks the EURC
    // Mints amount units of Gladius Coins
    // Can only be called by the Gladius Contract (admin)
    // Gladius Coins are sent to the Gladius Contract

redeem(amount):
    // Requires that the user sends amount of Gladius Coins
    // Burns amount of Gladius Coins
    // Gives back to the user the locked amount/v EURC
    // This function can only be called by Sport Clubs, so they can transform the received Gladius Coins into EUR after they have sold some NFTs
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