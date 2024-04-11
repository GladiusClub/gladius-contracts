import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import {
  getOwnerBalanceyNFT,
  getURI,
  getTotalSupplyNFT,
  get_token_of_owner_by_index,
} from '../utils/contract.js';
import { config } from '../utils/env_config.js';
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
  console.log('🚀 ~ Club ~ totalSupplyNFT:', totalSupplyNFT);

  const ownerBalanceyNFT = await getOwnerBalanceyNFT(
    addressBook.getContractId(network, 'gladius_nft_id'),
    'GDKT3YL6QCPPJZ53R7PUN6VX7F2SFZNSYCGALC7DIUVNHEV5IJSNKFRM',
    sport_club
  );
  console.log('🚀 ~ Student ~ balanceNFT:', ownerBalanceyNFT);

  //const newIndex = Number(totalSupplyNFT)
  //console.log("newIndex", newIndex)

  let tokenIds = [];

  for (let i = 0; i < ownerBalanceyNFT; i++) {
    const tokenId = await get_token_of_owner_by_index(
      addressBook.getContractId(network, 'gladius_nft_id'),
      'GDKT3YL6QCPPJZ53R7PUN6VX7F2SFZNSYCGALC7DIUVNHEV5IJSNKFRM',
      i,
      sport_club
    );
    tokenIds.push(Number(tokenId));
  }

  console.log('🚀 ~ All Token IDs:', tokenIds);

  let uris = [];
  let allUriContentData = [];


  for (let tokenId of tokenIds) {
    const uri = await getURI(
      addressBook.getContractId(network, 'gladius_nft_id'),
      tokenId,
      student
    );
    console.log('🚀 ~ Token ID:', tokenId, 'URI:', uri);
    uris.push(uri);
    const UriContent = await fetch(uri);
    const UriContentData = await UriContent.json();
    //console.log("UriContentData: ", UriContentData)
    allUriContentData.push(UriContentData);
  }

  //console.log('🚀 ~ All URIs:', uris);
  const combinedJsonObject = { nfts: allUriContentData };
  console.log(combinedJsonObject);

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


