# The Gladius Club Economic Model

Gladius Club is a Blockchain-based GameFi (Game Finance) application that aims to incentivize students to engage in physical sports activities by connecting Sport Clubs, Parents, and Students through digital points and assets.

Key Features:

- **Sport Subscriptions:** Sport Clubs offer monthly subscriptions priced in EURC (Stellar-based digital stablecoin). Parents pay for these subscriptions on behalf of their children.

- **Incentives:** Each subscription payment consists of two parts: the base subscription fee (p) and an incentive fee (k). The incentive fee is used to reward students with digital points and assets for their participation in sports activities.

- **Liquidity Pool:** The incentive fees collected are deposited into a Liquidity Pool contract, which mints Gladius Coins at a rate of v for each unit of EURC received. These Gladius Coins serve as digital rewards for students.

- **Token Distribution:** The Gladius Subscription and Token Distribution Smart Contract splits the base subscription fee (p) directly to Sport Clubs and the incentive fee (k) to the Liquidity Pool contract. In return, the contract receives Gladius Coins, which are held on behalf of the Sport Clubs.

- **Distribution of Rewards:** Sport Clubs can distribute Gladius Coins to students who have active subscriptions. These coins can be used to redeem rewards such as digital assets, NFTs (Non-Fungible Tokens), or even physical goods.

- **NFT Minting:** Sport Clubs can mint NFTs that represent economically valuable digital assets or physical goods. These NFTs can be associated with locked amounts of Gladius Coins, which can be redeemed by students upon receiving the NFT.

- **Economic Valuable NFTs:** The NFT contract implemented in the Gladius ecosystem ensures that only Sport Clubs can mint NFTs. These NFTs have locked amounts of Gladius Coins, which can be unlocked and sent back to Sport Clubs once redeemed for physical goods.

In summary, Gladius Club incentivizes student participation in physical sports by offering digital rewards in the form of Gladius Coins and NFTs. These rewards are funded through the subscription fees paid by parents and are distributed transparently through smart contracts, ensuring fair and equitable distribution within the ecosystem.

## Index
[1. General Concepts](./GeneralConcepts.md)

[2. Smart Contracts](./SmartContracts.md)