import { db } from './firebaseAdminSetup.js';
import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { mintToken } from './mint_token.js';



export async function testGladius(addressBook: AddressBook) {
  const network = process.argv[2] || 'testnet'; 
  const loadedConfig = config(network);
  let gladius_admin = loadedConfig.admin;
  let payment_token_admin = loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET');
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let parent = loadedConfig.getUser('PARENT_SECRET');

  async function getAllBalances(studentPublicKey: string)  {

    let balanceGLCStudent = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'),
      studentPublicKey,
      sport_club
    );

  
    console.log("« GLC  balance Student:", balanceGLCStudent)
    console.log("  ");
    

}


  async function handleStudentOperations(studentPublicKey: string) {
    await getAllBalances(studentPublicKey);

    const courseIndex = 0;
    console.log("~ handleStudentOperations ~ courseIndex:", courseIndex);
    await getAllBalances(studentPublicKey);
    
    console.log("  🏆  | Gladius Coins Distribution")
    const min_dist = 200;
    const max_dist = 250;
    const randomMDist = Math.floor(Math.random() * (min_dist - max_dist + 1)) + min_dist;
    
    console.log("      | Sport Club will distribute GLC to student: ", randomMDist)

    const distributeParams: xdr.ScVal[] = [
      nativeToScVal(courseIndex, { type: 'u32' }), // course_index
      new Address(studentPublicKey).toScVal(), // student
      nativeToScVal(randomMDist, { type: 'i128' }), // amount
    ];
    await invokeContract('gladius_subscriptions_id', addressBook, 'distribute_gladius_coins', distributeParams, sport_club);
    await getAllBalances(studentPublicKey);
  }
const usersSnapshot = await db.collection('users').get();
  
  // Iterate over each document
  for (let doc of usersSnapshot.docs) {
    const userData = doc.data();
    console.log("Document ID:", doc.id); 

    if (userData.stellar_wallet) {
      console.log("firebase stellar_wallet: ", userData.stellar_wallet); 
     
      try {
        // Attempt to fetch the balance, if it fails, catch the error and continue
        await handleStudentOperations(userData.stellar_wallet);
      } catch (error) {
        console.error("Failed to get balance for:", userData.stellar_wallet, error);
        // Optionally log the error and continue with the next iteration
        continue;
      }
    }
  }
}



(async () => {
  const folder = 'public'; // Default value, adjust as needed
  const network = 'testnet'; // Default value, adjust as needed
  let addressBook: AddressBook;

  // Load the AddressBook based on the specified network and folder
  if (folder === 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
  } else {
    addressBook = AddressBook.loadFromFile(network);
  }

  // Execute the main function with the loaded AddressBook
  await testGladius(addressBook);
})();
