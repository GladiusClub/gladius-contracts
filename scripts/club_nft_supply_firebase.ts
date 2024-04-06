import { db } from './firebaseAdminSetup.js';
import { AddressBook } from '../utils/address_book.js';

import { getURI, getTotalSupplyNFT, get_token_of_owner_by_index, getOwnerBalanceyNFT} from '../utils/contract.js';
import { api_config } from '../utils/api_config.js';

export async function testGladiusNFT(addressBook: AddressBook, user_stellar_secret: string, club_stellar_secret: string) {

  const student = api_config(network, user_stellar_secret);
  const studentPublicKey = student.publicKey(); 

  const sport_club = api_config(network, club_stellar_secret);
  const clubPublicKey =  sport_club.publicKey();
  
  console.log('gladius nft contract id: ', addressBook.getContractId(network, 'gladius_nft_id'));
 
  try {
    const ownerBalanceyNFT = await getOwnerBalanceyNFT(
      addressBook.getContractId(network, 'gladius_nft_id'),
      studentPublicKey,
      sport_club
    );
    console.log('🚀 ~ Student ~ balanceNFT:', ownerBalanceyNFT);
    
    let tokenIds = [];
  
    for (let i = 0; i < ownerBalanceyNFT; i++) {
      const tokenId = await get_token_of_owner_by_index(
        addressBook.getContractId(network, 'gladius_nft_id'),
        studentPublicKey,
        i,
        sport_club
      );
      tokenIds.push(Number(tokenId));
    }
  
    console.log('🚀 ~ All Token IDs:', tokenIds);
  
    let uris = [];
  
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
      console.log("UriContentData: ", UriContentData)
  
    }
  } catch (error) {
      console.error("An error occurred:", error);
}

}


const folder = 'public'; 
const network = 'testnet'; 
let addressBook: AddressBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}

  
console.log("Connecting to firebase");
  const UID = '4KKWdVfzUcUcJf9mVSVdPRXSNLI2'
  
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
          console.log(`Club wallet ${club_stellar_wallet} `);
          
          await testGladiusNFT(addressBook, user_stellar_secret, club_stellar_secret);

        }
        

      }
      else {console.log(`club with ID ${club_id} not found `);}
      
    }
  } else {
    console.log("No document found with ID 40WiH4RtOIgtJxGjwO6vadjAOem2");
  }
  
