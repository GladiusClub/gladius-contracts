# The Gladius Club Economic Model
## Introduction
Gladius Club is an Blockchain based GameFi App that connects Sport Clubs, Parents and Students in order to incentivize students to do phisical sports through digital points and assets. 

## Actors
1.- **Parents**. They want to see their children doing sports. They are willing to pay a price _p_ for a monthly sport subscription.

2.- **Sport Clubs**. They have an economic activity, and they offer sport monthly subscriptions at a price _p_. 

3.- **Students**. They receive the sport subscription for free (paid by parents) and their goal is to increase their profit which is the sum of points and prizes.

## Incentives
In the Gladius Club game there are different types of incentives that increase the students as well as the parents profit functions. We say that when a student receives an asset valued in _k_, the profit function of the parents also increase in the same amount. Parents feel happy when children are happy.

This means that parents will be ok to pay an extra amount on the monthly subscription if on average their children will receive this profit back.

Sport Clubs  ability to mint Gladius Coins or other digital prizes will be controlled by a Smart Contract in order to make sure that in average, students will receive _k_ in terms of incentives

## Payments and fee split
Montly paymnts of the price _p_ and incentives _k_ are made in 3 steps:

1.- Parents transform fiat EUR into Stellar based digital stablecoin EURC (onramp)
2.- Parents pay a monthly subscrition thrugh a Smart Contract that split the (p+k) payment. Mothly subsription _p_ goes directly to Sport Clubs and EURC incentives value _k_ goes to a **Liquidity Pool** Smart Contract.

This **Liquidity Pool** exists in order to give economcic value to Gladius Coins that will be minted by Sport Clubs.

The Liquidity Pool smart contract 

Later, Sport Clubs can withdraw the total amount of monthly subscriptions by using the EURC off ramp to send money to their bank accounts.

## Galdius Coins minting process.
In average, students should receive back incentives valued in _k_ payed by their parents. In order to achieve this in the equilibrum, after each month cycle, Sport Clubs will be able to mint a limited amount of _v*k_ Gladius Coins.
Here, _v_ is the value of 1 EUR in terms of Gladius Coins. If _v=1000_, `1 EUR = 1000 GC`