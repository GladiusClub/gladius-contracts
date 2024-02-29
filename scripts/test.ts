import { Address, nativeToScVal, xdr } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance, invokeContract } from '../utils/contract.js';
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
  const balanceAdminBefore = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    loadedConfig.admin.publicKey(),
    loadedConfig.admin
  );
  console.log('🚀 « EURC balanceAdminBefore:', balanceAdminBefore);

  // Example of transfering a token with the token's transfer function from the admins account to the pegged token admin account
  // ONLY EXAMPLE SHOULD BE REMOVED AFTER
  console.log('-------------------------------------------------------');
  console.log('Example of executing a method of an smart contract');
  console.log('Making a transfer of EURC Token from gladius admin to pegged token admin');
  console.log('-------------------------------------------------------');
  const balanceTokenAdminBefore = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey(),
    loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET')
  );
  console.log('🚀 « EURC balanceTokenAdminBefore:', balanceTokenAdminBefore);

  const transferInitParams: xdr.ScVal[] = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    new Address(loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey()).toScVal(),
    nativeToScVal(1000000000, { type: 'i128' }),
  ];
  await invokeContract('token_id', addressBook, 'transfer', transferInitParams, loadedConfig.admin);
  // Balances after transfering
  const balanceAdminAfter = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    loadedConfig.admin.publicKey(),
    loadedConfig.admin
  );
  console.log('🚀 « EURC balanceAdminAfter:', balanceAdminAfter);
  const balanceTokenAdminAfter = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey(),
    loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET')
  );
  console.log('🚀 « EURC balanceTokenAdminAfter:', balanceTokenAdminAfter);
  // END OF EXAMPLE
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();

const loadedConfig = config(network);

await testGladius(addressBook);
