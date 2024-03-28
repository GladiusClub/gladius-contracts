import { config } from '../utils/env_config.js';
import { getTokenBalance } from '../utils/contract.js'; // Import getTokenBalance
import { AddressBook } from '../utils/address_book.js'; // Import AddressBook

// Retrieves the network argument from the command line
const network = 'testnet';
const folder = 'public';

let addressBook: AddressBook;
addressBook = AddressBook.loadFromFile(network, folder);

const loadedConfig = config(network);

export async function getStudentBalance() {
  let student = loadedConfig.getUser('STUDENT_SECRET');

  let balanceGLCStudent = await getTokenBalance(
    addressBook.getContractId(network, 'gladius_emitter_id'),
    student.publicKey(),
    student
  );

  console.log('« GLC balance Student:', balanceGLCStudent);
}
