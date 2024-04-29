# D1: Gladius and Clubs Coins Tokenomics Model: 
Formulate a sustainable economic model wherein parents contribute payments to clubs for student fees, with a portion allocated to a pool. Clubs can mint ClubCoins and NFT’s to students. Students can subsequently trade their NFTs or Coins for physical goods or for other valuable NFT’s. This deliverable includes the technical description of every smart contract that will be developed.

**Reviewer Instructions**: The Tokenomics Model and Techincal report will be shared on the Gladius GitHub repository and on the Stellar Discord Channel.

**Status**: Done


# D2: Token Distribution Smart Contract
Develop a contract that handles monthly parental payments, divides the payment between clubs and the Gladius Coin (points) Pool, and enables Clubs to mint  the Club Coins and NFT prizes for students. This deliverable encompasses coding, along with the creation of unit and integration tests. Contract Specifications:
- [x] Linked to a specific coin, defaulting to the Gladius Coin
- [x] Facilitates monthly parental payments
- [x] Allocates payments to Clubs and the Gladius Coin pool
- [x] Enables the minting of Gladius Coins for students
- [x] Facilitates the minting of NFTs
      
**Reviewer Instructions**:  Code will be available in Gladius GitHub as OpenSource.

**Status**: Done


# D3: NFT smart contract and Pinata implementation
- [x] Creation of the Gladius Prizes NFT Smart Contract in Soroban 
- [x] implement Pinata for IPFS descentralize image storage. 
- [] Implementation of a lazy minting technique in Soroban

**Reviewer Instructions**:  Code will be available in Gladius GitHub as OpenSource

**Status**: In Progress


# D4: Token Distribution Factory Contract
The Factory Contract facilitates Clubs in generating personalized Token Distribution Smart Contracts, enabling them to issue branded coins and NFTs to students, receiving premium payments and deploying both a Premium Branded Coin and Token Distribution Smart Contract.

**Reviewer Instructions**: Code will be available in Gladius GitHub as OpenSource

**Status**: In Progress

# D5:  User Wallets creations
- [x] Development of the automatization of the Parents,Clubs and Students  
- [x] wallet creation and secure private key storage when they log in into our platform. 

**Reviewer Instructions**: Reviewer will be able to login to our platform and see his/hers wallet address, Gladius Tokens, NFT gallery, but won’t be able to extract the private key.

**Status**: Done


# D6: Backend
Adapting the already existing backend
- [x] converting Web3 elements to Soroban
- [x] creating cloud functions that interact with Soroban RPC, and setting up new wallets
Cloud functions for communicating with the frontend have already been developed, but may need some adjustment to accommodate Soroban.
- [x] Successful integration with Soroban
- [x] Successful integration with cloud functions
- [x] back-end can create wallets, inistate transactions on wallets and general interaction with the smart contracts.

**Reviewer Instructions**:

**Status**: Done


# D7: Frontend
The club user Frontend has largely been built and deployed in a PoC design. We need to adjust the Club frontend, making it to work with Soroban, updating APIs. We also need to deploy the Student frontend to code, and attach it to the backend APIs.
- [x] User-friendly frontend 
- [x] allows clubs to Login and automatically create a waller
- [x]  Transfer Gladius Coins to students
- [x]  Mint and transfer NFTs as rewards.

**Reviewer Instructions**:  Students will be able to login, see their rewards, and transfer their tokens to receive physical awards at the club

**Status**: In Progress