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
- [ ] Implementation of a lazy minting technique in Soroban

**Reviewer Instructions**:  Code will be available in Gladius GitHub as OpenSource

**Status**: Done


# D4: Token Distribution Factory Contract
The Factory Contract facilitates Clubs in generating personalized Token Distribution Smart Contracts, enabling them to issue branded coins and NFTs to students, receiving premium payments and deploying both a Premium Branded Coin and Token Distribution Smart Contract.

**Reviewer Instructions**: Code will be available in Gladius GitHub as OpenSource

**Status**: In Progress

# D5:  User Wallets creations
- [x] Development of the automatization of the Parents,Clubs and Students  
- [x] wallet creation and secure private key storage when they log in into our platform.

**Creating Stellar wallets** for every new user in Firebase
- [create new user cloud function](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/singup_function/main.py)

Cloud functions **creating users for Parents, Clubs and Students** in Stellar/Soroban
- [SignupGladiusClub](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/SignupGladiusClub/index.ts)
- [Signup Gladius Parent](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/SignupGladiusParent/index.ts)

Assigning **special roles** to new users in Soroban contract
- [Signup Gladius Club Course](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/SignupGladiusClubCourse/index.ts)

Display GLC token **balance** in user wallet 
- [get balance](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/getStudentBalanceByID/index.ts)

**Reviewer Instructions**: Reviewer will be able to login to our platform and see his/hers wallet address, Gladius Tokens, NFT gallery, but won’t be able to extract the private key.
- Follow instructions described in D7: Frontend

**Status**: Done


# D6: Backend
Adapting the already existing Firebase backend, creating cloud functions that interact with Soroban RPC, and setting up new wallets. 
Cloud functions for communicating with the frontend have already been developed, but may need some adjustment to accommodate Soroban.
- [x] Adding Soroban to [existing Web3 elements](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/singup_function/main.py)
- [x] Successful integration or Soroban with cloud functions on Gladius back-end (all Gladius cloud functions were developed open-source and convered by MIT licenced

**Soroban token transfer**
- [transfer GLC tokens](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/transferGLCauth/index.ts)
- [burn Gladius tokens](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/burnGLC/index.ts)


**Gladius NFT contract powered by Soroban** (implementation of D3 in Gladius back-end)
- [mint Gladius NFT](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/mintGladiusNFT/index.ts)
- [fetch minted Gladius NFT](https://github.com/GladiusClub/gladius-backend/blob/main/gcp_cloud_functions/fetchGladiusNFT/index.ts)
  
  
**Reviewer Instructions**: 
- Review backend cloud functions logic described in [Gladius Knowledge Base](https://gladiusclub.gitbook.io/docs/v/backend)
- Review in Explorer that all transactions intiated in Frontend (see D7 review instructions) were executed and succeeded in Soroban network (Stellar.expert link will be provided in Frontend)



**Status**: Done


# D7: Frontend
The club user Frontend has largely been built and deployed in a PoC design. We need to adjust the Club frontend, making it to work with Soroban, updating APIs. We also need to deploy the Student frontend to code, and attach it to the backend APIs.
- [x] User-friendly frontend 
- [x] allows clubs to Login and automatically create a waller
- [x]  Transfer Gladius Coins to students
- [ ]  Mint and transfer NFTs as rewards.

**Reviewer Instructions**:  Students will be able to login, see their rewards, and transfer their tokens to receive physical awards at the club
1. Club signup in Soroban contract 
  - go to  https://gladius-club.web.app/
  - create new club
  - create new course (set course fee and incentive)
Go to https://gladius-frontend.web.app/
  - sign up as guardian
  - Enroll student to your new club and new course
  - click Simulate payment
  - click Expand to see created wallets
    - verify transactions on Club wallet
    - verify transactions on Parent  wallet
IMPORTANT: write down student login and password, logout and login to https://gladius-frontend.web.app/ with Student account
  - Go back to Club Admin and refresh the page
  - Verify that new student has been added
  - Verify that Club received EURC according to course fee 
2. Club rewards distribution
  - go to calendar page and click any event
  - select an event, assign a reward to a student, make a transfer
Go back to the main page and under Club Summary click "All transactions" (review rewards distribution)
3. Student app
  - login to https://gladius-frontend.web.app/ with Student account (password was generated during Parent singup)
  - Go to Profile and check the Student balance (it should match with distributed GLC from the club admin calendar)
  - Click Send, select Club name and click Send (wait a bit )
  - Click Receive to view the Wallet address and link to Explorer
Go back to Club admin and verify (refresh) that GLC payment from Student was received (use case: student bought some equipment)
4. NFT
Go to https://gladius-club.web.app/
- Find Marketshare page (NFT Rewards)
- Click Send and select a user that will receive the NFT
- CLick confirm
Go to https://gladius-frontend.web.app/
- Click Profile
- Go to badges and verify that NFT badge was received by a user

**Status**: Ready
