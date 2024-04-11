import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { mintToken } from './mint_token.js';



export async function testGladius(addressBook: AddressBook) {


  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  
  async function getAllBalances() {


      let balanceGLCSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        sport_club.publicKey(), // balance of who?
        sport_club
      );


      console.log("« GLC  balance SportClub:", balanceGLCSportClub)

  }

  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  console.log(" 💰  Minting 5000 EURC to parent")
  // Minting EURC tokens to the gladius admin account


  await getAllBalances();

}

const network = process.argv[2];
const folder = process.argv[3];
let addressBook: AddressBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}

const loadedConfig = config(network);


await testGladius(addressBook);
