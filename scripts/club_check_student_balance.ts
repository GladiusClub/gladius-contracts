// Import Firestore database instance
import { db } from './firebaseAdminSetup.js';

import { AddressBook } from '../utils/address_book.js';
import { getTokenBalance } from '../utils/contract.js';
import { config } from '../utils/env_config.js';

export async function testGladius(addressBook: AddressBook) {
  const network = 'testnet'; 
  const loadedConfig = config(network); 
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET'); 
 // let student = loadedConfig.getUser('STUDENT_SECRET');

  // Function to get balances for a student
  async function getStudentBalances(studentPublicKey: string) {
    // Example token balance fetch using `getTokenBalance`
    let balanceGLCStudent = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'),
      studentPublicKey,
      sport_club
    );
    console.log("GLC balance for Student:", balanceGLCStudent);
  }

  // Fetch users from Firestore
  const usersSnapshot = await db.collection('users').get();
  
  // Iterate over each document
  for (let doc of usersSnapshot.docs) {
    const userData = doc.data();
    console.log("Document ID:", doc.id); 

    if (userData.stellar_wallet) {
      console.log("firebase stellar_wallet: ", userData.stellar_wallet); 
     // console.log("firebase publicKey type: ", typeof userData.stellar_wallet); 
     // console.log("student.publicKey: ", student.publicKey()); 
     // console.log("student.publicKey type: ", typeof student.publicKey()); 
      
      // Await the balance fetch to ensure synchronous logging
      //await getStudentBalances(userData.stellar_wallet);
      
      try {
        // Attempt to fetch the balance, if it fails, catch the error and continue
        await getStudentBalances(userData.stellar_wallet);
      } catch (error) {
        console.error("Failed to get balance for:", userData.stellar_wallet, error);
        // Optionally log the error and continue with the next iteration
        continue;
      }
    }
  }
}

// Example usage
(async () => {
  const network = process.argv[2] || 'testnet';
  const folder = process.argv[3] || 'public';
  let addressBook;

  if (folder === 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
  } else {
    addressBook = AddressBook.loadFromFile(network);
  }

  await testGladius(addressBook);
})();