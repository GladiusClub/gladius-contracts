import * as functions from 'firebase-functions';
import { config } from './utils/env_config.js'; // Adjust the path as necessary
import { getTokenBalance } from './utils/contract.js'; // Import getTokenBalance
import { AddressBook } from './utils/address_book_api.js'; // Import AddressBook

export const getStudentBalance = functions.https.onRequest(async (req, res) => {
  try {
    // Extract 'network' and 'folder' from query parameters
    const network = req.query.network as string || 'testnet';
    const folder = req.query.folder as string || 'public';

    let addressBook: AddressBook;
    addressBook = AddressBook.loadFromFile(network, folder);

    const loadedConfig = config(network);
    let student = loadedConfig.getUser('STUDENT_SECRET');

    let balanceGLCStudent = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'),
      student.publicKey(),
      student
    );

    res.status(200).send(`GLC balance Student: ${balanceGLCStudent}`);
  } catch (error) {
    console.error('Failed to get student balance:', error);
    res.status(500).send('Internal Server Error');
  }
  
});