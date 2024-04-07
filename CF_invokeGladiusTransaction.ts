import * as functions from 'firebase-functions';
import { Address, nativeToScVal, xdr } from 'stellar-sdk';
import { db } from './scripts/firebaseAdminSetup.js';
import { AddressBook } from './utils/address_book.js';
import { getTokenBalance, invokeContract } from './utils/contract.js';
import { api_config } from './utils/api_config.js';

export const invokeGladiusTransaction = functions.https.onRequest(async (request, response) => {
  // Set CORS headers for preflight requests
  response.set('Access-Control-Allow-Origin', 'http://localhost:3000');
  response.set('Access-Control-Allow-Methods', 'GET, POST');
  response.set('Access-Control-Allow-Headers', 'Content-Type');
  response.set('Access-Control-Max-Age', '3600');

  if (request.method === 'OPTIONS') {
    response.status(204).send('');
    return;
}

  const network = process.argv[2] || 'testnet';     

  const UID: string = request.body.UID; // Extract UID from the request body
  const amount: number = parseInt(request.body.amount, 10); // Extract and parse the amount from the request body

  if (!UID || isNaN(amount)) {
    response.status(400).send('Missing or invalid parameters.');
    return;
  }

  let addressBook: AddressBook;
  const folder = 'public'; 

  if (folder === 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
  } else {
    addressBook = AddressBook.loadFromFile(network);
  }

 
  
 
  async function handleStudentOperations(user_stellar_secret: string, club_stellar_secret: string) {
    const student = api_config(network, user_stellar_secret);
    const studentPublicKey = student.publicKey(); 

    const sport_club = api_config(network, club_stellar_secret);
    const clubPublicKey =  sport_club.publicKey();
    console.log("ContractId: ", addressBook.getContractId(network, 'gladius_emitter_id'))
    const sender_balance = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'), // what token
      studentPublicKey, // balance of who?
      sport_club
    );

    console.log("Sender balance ", sender_balance)
    const courseIndex = 0;
    console.log("~ handleStudentOperations ~ courseIndex:", courseIndex);
    console.log("  ↔️   | Gladius Coins Transaction")
    
    //const min_dist = 10;
    //const max_dist = 15;
    //const amount = Math.floor(Math.random() * (min_dist - max_dist + 1)) + min_dist;
    
    console.log("      | Student Sends GLC to Sport Club: ", amount)

    const transactionParams: xdr.ScVal[] = [
      new Address(studentPublicKey).toScVal(), // from
      new Address(clubPublicKey).toScVal(), // to
      nativeToScVal(amount, { type: 'i128' }), // amount
    ];

    if (amount < 0) {
      console.log('ERROR: Amount should be a positive number')
      response.status(400).send({ to_address: studentPublicKey, error: 'Amount should be a positive number' });;
      return;

    } else if (amount === 0) {
      console.log('ERROR: Not allowed to send 0 coins')
      response.status(400).send({ to_address: studentPublicKey, error: 'Not allowed to send 0 coins' });;
      return;

    } else if (clubPublicKey === studentPublicKey) {
      console.log('ERROR: Not allowed to send coins to yourself')
      response.status(400).send({ to_address: studentPublicKey, error: 'Not allowed to send coins to yourself' });;
      return;
      
    } else if (sender_balance < amount) {
      console.log('ERROR: Transfer amount exceeds balance')
      response.status(400).send({ to_address: studentPublicKey, error: 'Transfer amount exceeds balance' });
      return;
      
    } else {
      // If all checks passed, perform the transaction
      try {
      console.log("invokeContract");
      await invokeContract('gladius_emitter_id', addressBook, 'transfer', transactionParams, student);
      console.log("Success");
      const sender_balance_after = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        studentPublicKey, // balance of who?
        sport_club
      );
      console.log("Sender balance after", sender_balance_after)

     return response.status(200).json({ to_address: studentPublicKey, sent: amount }); // Indicating success
      //message: `GLC sent form ${stellar_wallet} to ${club_stellar_wallet} in the amount of ${amount}`
      
    } catch (error) {
      const errorMessage = (error as Error).message;
      console.error(`Error invoking contract for address ${studentPublicKey}: ${errorMessage}`);
      
      response.status(400).send({ to_address: studentPublicKey, error: `Contract invocation failed: ${errorMessage}` });
      return // Indicating a client-side error
    }
  
  
  }
   
}

  console.log("Connecting to firebase");
  
  const docRef = db.collection('users').doc(UID);
  const club_id = '2';
  const clubRef = db.collection('clubs').doc(club_id);
  
  const docSnap = await docRef.get();
  const clubSnap = await clubRef.get();

  if (docSnap.exists) { 
    const userData = docSnap.data();
    if (userData && userData.stellar_wallet && userData.email) { // Check if userData is truthy before accessing its properties
      console.log(`Document with ID ${UID} found. User email: `, userData.email);
      
      const stellar_wallet = userData.stellar_wallet
      const user_stellar_secret = userData.stellar_secret
      console.log("proccess stellar wallet:", stellar_wallet);

      if (clubSnap.exists) {
        const clubData = clubSnap.data();
        if (clubData && clubData.name && clubData.club_stellar_secret && clubData.club_stellar_wallet) {
          console.log(`Club ID ${club_id} was found. It's ${clubData.name} `);
          
          const club_stellar_wallet = clubData.club_stellar_wallet
          const club_stellar_secret = clubData.club_stellar_secret
          console.log(`Destination wallet ${club_stellar_wallet} `);
          await handleStudentOperations(user_stellar_secret, club_stellar_secret)
          
          response.status(200).json({
            message: `GLC sent form ${stellar_wallet} to ${club_stellar_wallet} in the amount of ${amount}`
        })
        }
        

      }
      else {console.log(`club with ID ${club_id} not found `);}
      
    }
  } else {
    console.log(`No document found with ID ${UID}`);
  }
  
  
});

