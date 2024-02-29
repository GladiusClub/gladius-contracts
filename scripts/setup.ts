import { Address, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import {
  airdropAccount,
  bumpContractCode,
  bumpContractInstance,
  deployContract,
  installContract,
  invokeContract,
} from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { deployToken } from './deploy_token.js';

export async function deployAndInitContracts(addressBook: AddressBook) {
  // Setting up public address for gladius accounts
  addressBook.setContractId(network, 'gladius_admin_public', loadedConfig.admin.publicKey());
  await airdropAccount(loadedConfig.admin);

  if (network != 'mainnet') {
    // Gladius EURC Token
    console.log('-------------------------------------------------------');
    console.log('Deploying and Initializing Pegged EURC token');
    console.log('-------------------------------------------------------');
    addressBook.setContractId(
      network,
      'pegged_token_admin_public',
      loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey()
    );
    await airdropAccount(loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET'));
    await deployToken(
      'EURC Token',
      'EURC',
      addressBook,
      loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET')
    );
  }

  console.log('-------------------------------------------------------');
  console.log('Installing Gladius Contracts');
  console.log('-------------------------------------------------------');
  // Gladius Coin Emitter
  await installContract('gladius_coin_emitter', addressBook, loadedConfig.admin);
  await bumpContractCode('gladius_coin_emitter', addressBook, loadedConfig.admin);
  // Gladius Subscriptions
  // console.log('-------------------------------------------------------');
  // await installContract('gladius_subscriptions', addressBook, loadedConfig.admin);
  // await bumpContractCode('gladius_subscriptions', addressBook, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying and Initializing Gladius Emitter');
  console.log('-------------------------------------------------------');
  await deployContract(
    'gladius_emitter_id',
    'gladius_coin_emitter',
    addressBook,
    loadedConfig.admin
  );
  await bumpContractInstance('gladius_emitter_id', addressBook, loadedConfig.admin);

  const ratio = 1000;

  // Initializing Gladius Emitter
  const emitterInitParams = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    new Address(addressBook.getContractId(network, 'token_id')).toScVal(),
    nativeToScVal(ratio, { type: 'u32' }),
  ];
  await invokeContract(
    'gladius_emitter_id',
    addressBook,
    'initialize',
    emitterInitParams,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();

const loadedConfig = config(network);

await deployAndInitContracts(addressBook);
addressBook.writeToFile();
