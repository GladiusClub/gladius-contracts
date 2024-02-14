# The Gladius Club Economic Model

## Introduction

Gladius Club is a Blockchain-based GameFi App that connects Sport Clubs, Parents, and Students in order to incentivize students to participate in physical sports through digital points and assets.

## Actors

1. **Parents**: They want to see their children participating in sports. They are willing to pay a price (_p_) for a monthly sports subscription.

2. **Sport Clubs**: They engage in economic activities and offer monthly sport subscriptions at a price (_p_).

3. **Students**: They receive the sports subscription for free (paid by parents), and their goal is to increase their profit, which is the sum of points and prizes.

## Incentives

In the Gladius Club game, there are different types of incentives that increase both the students' and the parents' profit functions. We say that when a student receives an asset valued at _k_, the profit function of the parents also increases by the same amount. Parents feel happy when their children are happy.

This means that parents will be okay to pay an extra amount on the monthly subscription if, on average, their children will receive this profit back.

Sport Clubs' ability to mint Gladius Coins or other digital prizes will be controlled by a Smart Contract to ensure that, on average, students will receive _k_ in terms of incentives.

## Payments and fee split

Monthly payments of the price _p_ and incentives _k_ are made in 3 steps:

1. Parents transform fiat EUR into Stellar-based digital stablecoin EURC (on-ramp).
2. Parents pay a monthly subscription through a Smart Contract that splits the (p+k) payment. Monthly subscription _p_ goes directly to Sport Clubs, and EURC incentives value _k_ goes to a **Liquidity Pool** Smart Contract.

This **Liquidity Pool** exists to give economic value to Gladius Coins that will be minted by Sport Clubs.

The Liquidity Pool smart contract is a `1:v` stable coin contract, where for every 1 EURC, it always mints `v` units of Gladius Coin. And for every `v` amount of Gladius Coins, it always returns 1 EURC.

Here, _v_ is the value of 1 EURC in terms of Gladius Coins. If _v=1000_, `1 EURC = 1000 GC`.

Later, Sport Clubs can withdraw the total amount of monthly subscriptions by using the EURC off-ramp to send money to their bank accounts.

## Gladius Coins minting process

On average, students should receive back incentives valued at _k_ paid by their parents. To achieve this equilibrium, after each monthly cycle, and because _k_ is being sent to the Liquidity Pool Smart Contract, Sport Clubs will be able to mint a limited amount of _v*k_ Gladius Coins for each student they have.

However, Sport Clubs are restricted to send these coins only to their own students. This means that if one Sport Club has a group A with `n` students, and another Sport Club has a group B with m students, in total, both Sport Clubs will be able to mint `n*v*k + m*v*k` Gladius Coins. However, Sport Club A will be restricted to send `n*v*k` to their students, and Sport Club B will be restricted to send `m*v*k` to their students.

Because we suppose that the Gladius Coin distribution within a group follows a standard normal distribution, the expected total amount of coins received by each student will be `v*k`, hence, they will receive on average an incentive economically valued in `k` EUR.

## Digital Prizes and NFTs

### NFTs distributed by Sport Clubs

Sport Clubs can not only distribute Gladius Coins to their students, but they can also mint some NFTs. NFTs distributed directly by Sport Clubs to their students won't have economic value associated. This is the case of digital sport cups, medalls or other prizes that students will collect just for pleasure.

### NFTs sold by Sport Clubs

Sport Clubs can offer economically valued NFTs like NFTs that can be redeemed for physical goods (training equipment). In this case, the NFT will be offered at a price in Gladius Coins. Students will be able to exchange their Gladius Coins for these NFT's.

When Sport Clubs sell these items, they will receiven these Gladius Coins that they can later exchange for EURC using the Liquidity Pool contract.

Economically valued NFTs will have a `redeemed` flag, so students can later redeem them for some physical good.
