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
      addressBook.getContractId(network, 'token_id'),
      studentPublicKey,
      parent
    );

    let balanceGLCGladiusSubscriptions = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'), // what token
      addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
      sport_club
    );

    let balanceGLCSportClub = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'), // what token
      sport_club.publicKey(), // balance of who?
      sport_club
    );
    console.log("  ");
    console.log("  ");
    console.log("« GLC  balance GladiusSubscriptions:", balanceGLCGladiusSubscriptions)
    console.log("« GLC  balance SportClub:", balanceGLCSportClub)
    console.log("« GLC  balance Student:", balanceGLCStudent)
    console.log("  ");
    console.log("  ");
    


}


  // Adjusted function to handle operations for a given student's public key
  async function handleStudentOperations(studentPublicKey: string) {
    await getAllBalances(studentPublicKey);

    
    console.log("  🕵️  | Checking and Setting Roles");

    let balanceGLCStudent = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'), // what token
      studentPublicKey, // balance of who?
      sport_club
    );
    console.log("« GLC  balance Student:", balanceGLCStudent)

    const isStudentBefore = await getIsRole(
      addressBook.getContractId(network, 'gladius_subscriptions_id'),
      'is_student',
      studentPublicKey,
      sport_club // Assuming `sport_club` is used for authentication; adjust as needed
    );
    console.log("~ handleStudentOperations ~ isStudentBefore:", isStudentBefore);
    
    const setIsStudentParams: xdr.ScVal[] = [
      new Address(studentPublicKey).toScVal(), // Using studentPublicKey
      nativeToScVal(true, { type: 'bool' }), // is
    ];
    await invokeContract('gladius_subscriptions_id', addressBook, 'set_is_student', setIsStudentParams, gladius_admin);

    const isStudentAfter = await getIsRole(
      addressBook.getContractId(network, 'gladius_subscriptions_id'),
      'is_student',
      studentPublicKey,
      sport_club // Assuming `sport_club` is used for authentication; adjust as needed
    );
    console.log("~ handleStudentOperations ~ isStudentAfter:", isStudentAfter);

    console.log("  📝  | Checking Courses");

    const totalCoursesBefore = await getTotalCourses(
      addressBook.getContractId(network, 'gladius_subscriptions_id'),
      gladius_admin
    );
    console.log(" ~ handleStudentOperations ~ totalCoursesBefore:", totalCoursesBefore);

    const courseIndex = 0;
    console.log("~ handleStudentOperations ~ courseIndex:", courseIndex);

    const getCourseParams: xdr.ScVal[] = [
      nativeToScVal(courseIndex, { type: 'u32' }), // index
    ];
    const gotCourse = await invokeContract('gladius_subscriptions_id', addressBook, 'get_course', getCourseParams, sport_club);
    const gotCourseNative = scValToNative(gotCourse.returnValue);
    console.log(" ~ handleStudentOperations ~ gotCourseNative:", gotCourseNative);

    console.log("  🎾  | Subscribing to Courses");

    const subscribeCourseParams: xdr.ScVal[] = [
      new Address(parent.publicKey()).toScVal(), // parent
      new Address(studentPublicKey).toScVal(), // student
      nativeToScVal(courseIndex, { type: 'u32' }), // course_index
    ];
    
    await invokeContract('gladius_subscriptions_id', addressBook, 'subscribe_course', subscribeCourseParams, parent);

    
    console.log("  🏆  | Gladius Coins Distribution")
    console.log("      | Sport Club will distribute 1500 units of GLC to student")
    const min_dist = 100;
    const max_dist = 150;
    const randomMDist = Math.floor(Math.random() * (min_dist - max_dist + 1)) + min_dist;
     
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
