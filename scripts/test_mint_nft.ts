import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { invokeContract, getURI, getTotalSupplyNFT} from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import {pinFileToIPFS} from './image_to_pinata.js'

import * as fs from 'fs';
export async function testGladius(addressBook: AddressBook) {


  let gladius_admin = loadedConfig.admin;
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let student = loadedConfig.getUser('STUDENT_SECRET');


  console.log('-------------------------------------------------------');
  console.log('Minting one Gladius NFT');
  console.log('-------------------------------------------------------');

  const img_url = '/workspace/img/dragon_barge.jpg'
  const NftName = 'Gladius Cup'

  const img_uri = await pinFileToIPFS(img_url, NftName);
  console.log("pinFileToIPFS: " , img_url );
  
  const totalSupplyNFT = await getTotalSupplyNFT(
    addressBook.getContractId(network, 'gladius_nft_id'),
    sport_club
    );
  console.log("🚀 ~ testGladius ~ totalSupplyNFT:", totalSupplyNFT)

  const newIndex = Number(totalSupplyNFT) +1
  const mintNFTParams = [
    new Address(student.publicKey()).toScVal(), // to_student
    nativeToScVal(newIndex, { type: 'u32' }), // nft_index
    nativeToScVal(img_uri, { type: 'string' }),
  ];

    await invokeContract(
      'gladius_nft_id',
      addressBook,
      'mint',
      mintNFTParams,
      gladius_admin
    );

  console.log('-------------------------------------------------------');
  console.log('Getting the NFT URI');
  console.log('-------------------------------------------------------');
  

  const uri = await getURI(
    addressBook.getContractId(network, 'gladius_nft_id'),
    newIndex,
    sport_club
    );
  console.log("🚀 ~ testGladius ~ uri:", uri)

}

const network = process.argv[2] || 'testnet' ; 
const folder = process.argv[3] || 'public' ;
let addressBook: AddressBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}

const loadedConfig = config(network);


await testGladius(addressBook);
