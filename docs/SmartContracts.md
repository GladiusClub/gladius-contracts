# Gladius Smart Contracts

## The Liquidity Pool: Gladius Coin Token Emisor.
This contracts emits Gladius Coin at a rate `v` for each unit of `EURC` received.
It behaves similar to the [WETH Wrapped Contract](https://etherscan.io/address/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2#code)

Interface:
```rust
mint(amount){
    // Requieres that user sends amount/v in EURC
    // Locks the EURC
    // Mints amount units of Gladius Coins
    // Can only be called by the Gladius Contract (admin)
    // Gladius Coins are sent to the Gladius Contract
}
redeem(amount){
    // Requires that user sends amount of Gladius Coins
    // Burns amount of Gladius Coins
    // Gives back to user the locked amount/v EURC
    // This function can only be called by Sport Clubs, so they can transform the received Gladius Coins into EUR after they have sold some NFT's
}
```

This contract will also behave as the Gladius Coin token contract.

## The Gladius Subscription and Token Distribution Smart Contract
- Sport Clubs can open sport subscriptions at a price fixed in EURC
- Parents can subscribe a Student and pay the price `p+k` for a monthly subscription
- The Contract splits `p` directly to Sport Clubs and `k` to the Gladius Coin Token Emisor Contract. In exchange, the contract receives Gladius Coins.
- The contract holds Gladius Coins on behalf of Sport Clubs.
- The contract mantains a list of Sport Clubs, Parents, Students and Subscriptions with their expire date.
- Sport Clubs can distribute these Gladius Coins only to Students that have been subscribed.
- Sport Clubs can also distribute these Gladius Coins to some NFT contract so physical item redeemable NFT have a economic value

Interface:
```rust
sport_club_subscription(from: Address, sport_club: Address){
    // Gladius Admin can add new sport clubs
}
parent_subscription(from: Address, parent: Address){
    // Gladius Admin can add new parents
}
create_subscription(from: Address, price: u128, name: String, description: Sring){
    // Sport Clubs can create a Sport Monthly Subscription at a price. 
}

subscribe(from: Address, student: Address, subscription: String, periods: u128){
    // Allowed Parents can subscribe their students
    // Parents should have sent the price
    // Function splits the price p and send it direclty to the Sport Club address
    // Function split the amount k and calls the `mint` function in the Gladius Coin Token Emissor Contract
    // The contract ends holding Gladius Coins
    // Function asigns the minted Gladius Coins to the Sport Club (but does not send Gladius Coin to the Sport Club)... the contract hold them on behalf of the Club.
}

distribute(from: Address, amount; u128, student: Addres){
    // Sport Clubs can distribute Gladius Coins to Students
    // Function checks if the Sport Club has some Gladius Coin balance
    // Function checks that student has an active subscription with this Sport Club
}

mint_nft(from: Address, value; u128, student: Addres, name: String, description: String){
    // Sport Clubs can mint economic valuable NFT
    // Function checks if the Sport Club has some Gladius Coin balance
    // Function checks that student has an active subscription with this Sport Club
    // Function calls the NFT Contract to mint a NFT with a locked amount of Gladius Coins asociated 
}
```

## Economic Valuable NFT
Contract that implements the ERC27 Ethereum Standar in Soroban, but only lets Sport Clubs to mint NFTs.

Minted NFTs should have a locked amount of Gladius Coins that can be unlocked and sent back to Sport Clubs one the NFT has been redeemed by physical objects

NFTs can only be minted by the `Gladius Subscription and Token Distribution Smart Contract`