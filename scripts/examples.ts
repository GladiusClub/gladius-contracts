import { config } from '../utils/env_config.js';
import { getTokenBalance } from '../utils/contract.js'; // Import getTokenBalance
import { AddressBook } from '../utils/address_book.js'; // Import AddressBook
import { Keypair } from 'stellar-sdk';

// Retrieves the network argument from the command line
const network = process.argv[2];
const folder = process.argv[3];

if (!network) {
  console.error('Network argument is missing. Usage: node script.js [network]');
  process.exit(1);
}

let addressBook: AddressBook;

if (folder == 'public') {
  addressBook = AddressBook.loadFromFile(network, folder);
} else {
  addressBook = AddressBook.loadFromFile(network);
}

const loadedConfig = config(network);

// Declare these variables in a scope accessible by both functions

export async function outputSecrets() {
  let gladius_admin = loadedConfig.admin;
  let payment_token_admin = loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET');
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let parent = loadedConfig.getUser('PARENT_SECRET');
  let student = loadedConfig.getUser('STUDENT_SECRET');

  console.log('Gladius Admin Secret:', gladius_admin.publicKey());
  console.log('Payment Token Admin Secret:', payment_token_admin.publicKey());
  console.log('Sport Club Secret:', sport_club.publicKey());
  console.log('Parent Secret:', parent.publicKey());
  console.log('Student Secret:', student.publicKey());
}

async function getStudentBalance() {
  let student = loadedConfig.getUser('STUDENT_SECRET');

  const randomPair = Keypair.random();

  let balanceGLCStudent = await getTokenBalance(
    addressBook.getContractId(network, 'gladius_emitter_id'),
    student.publicKey(),
    student
  );

  console.log('« GLC balance Student:', balanceGLCStudent);
}

await outputSecrets();
await getStudentBalance();
