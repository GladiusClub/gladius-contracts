import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance, getIsRole, getTotalCourses, invokeContract, getURI, getTotalSupplyNFT} from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { mintToken } from './mint_token.js';
import * as fs from 'fs';



export async function testGladius(addressBook: AddressBook) {


  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let parent = loadedConfig.getUser('PARENT_SECRET');
  let student = loadedConfig.getUser('STUDENT_SECRET');

  

  console.log('-------------------------------------------------------');
  console.log('Getting the NFT URI');
  console.log('-------------------------------------------------------');
  console.log('gladius_nft_id: ', addressBook.getContractId(network, 'gladius_nft_id'));

  const totalSupplyNFT = await getTotalSupplyNFT(
    addressBook.getContractId(network, 'gladius_nft_id'),
    sport_club
    );
  console.log("🚀 ~ testGladius ~ totalSupplyNFT:", totalSupplyNFT)

  const newIndex = Number(totalSupplyNFT)
  console.log("newIndex", newIndex)

  const uri = await getURI(
    addressBook.getContractId(network, 'gladius_nft_id'),
    newIndex,
    sport_club
    );
  console.log("🚀 ~ testGladius ~ uri:", uri)


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
