import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from './utils/address_book_api.js';

import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from './utils/contract.js';
import { api_config } from './utils/api_config.js';

const network = process.argv[2] || 'testnet';     
const folder = 'public'; 
let addressBook: AddressBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}

export async function testGladius(addressBook: AddressBook) {
  const club_stellar_secret = 'SCOS74PSM3TD7APT2BTTNGASW4EHCPN75UFYDS6KGBDHG24NR6J2XYXQ';
  const sport_club = api_config(network, club_stellar_secret);

  
  async function getAllBalances() {


      let balanceGLCSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        sport_club.publicKey(), // balance of who?
        sport_club
      );


      console.log("« GLC  balance SportClub:", balanceGLCSportClub)

  }



  await getAllBalances();

}


await testGladius(addressBook);
