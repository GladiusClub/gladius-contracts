import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { mintToken } from './mint_token.js';

export async function testGladius(addressBook: AddressBook) {
  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  // Minting EURC tokens to the gladius admin account
  await mintToken(
    addressBook.getContractId(network, 'token_id'),
    25000000000000,
    loadedConfig.admin.publicKey(),
    loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET')
  );

  // Checking the balance of the gladius admin account
  const balance = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    loadedConfig.admin.publicKey(),
    loadedConfig.admin
  );
  console.log('🚀 « EURC balance:', balance);
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();

const loadedConfig = config(network);

await testGladius(addressBook);
