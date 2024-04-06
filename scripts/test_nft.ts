import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance, getIsRole, getTotalCourses, invokeContract, getURI, getTotalSupplyNFT, get_token_of_owner_by_index} from '../utils/contract.js';
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
  console.log('gladius nft contract id: ', addressBook.getContractId(network, 'gladius_nft_id'));

  const totalSupplyNFT = await getTotalSupplyNFT(
    addressBook.getContractId(network, 'gladius_nft_id'),
    sport_club
    );
  console.log("🚀 ~ Club ~ totalSupplyNFT:", totalSupplyNFT)

  //const newIndex = Number(totalSupplyNFT)
  //console.log("newIndex", newIndex)

  const token_of_owner = await get_token_of_owner_by_index(
    addressBook.getContractId(network, 'gladius_nft_id'),
    'GDKT3YL6QCPPJZ53R7PUN6VX7F2SFZNSYCGALC7DIUVNHEV5IJSNKFRM', //student.publicKey(),
    0,
    sport_club
  )
  console.log("🚀 ~ Student ~ token_of_owner_by_index:", token_of_owner)

  const StudentNftIndex = Number(token_of_owner);

  const uri = await getURI(
    addressBook.getContractId(network, 'gladius_nft_id'),
    StudentNftIndex,
    student
    );
  console.log("🚀 ~ Student ~ NFT uri by index", uri);

  try {
    const response = await fetch(uri);
    const responseData = await response.json();
    //const { name, img_url } = responseData;
    //console.log(`Name: ${name}, Image URL: ${img_url}`);
    console.log("responseData: ", responseData)
  } catch (error) {
      console.error("An error occurred:", error);
}

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
