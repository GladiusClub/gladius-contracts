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
  console.log('-------------------------------------------------------');
  await installContract('gladius_subscriptions', addressBook, loadedConfig.admin);
  await bumpContractCode('gladius_subscriptions', addressBook, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying Gladius Contrats');
  console.log('-------------------------------------------------------');
  // Gladius Coin Emitter
  await deployContract(
    'gladius_emitter_id',
    'gladius_coin_emitter',
    addressBook,
    loadedConfig.admin
  );
  await bumpContractInstance('gladius_emitter_id', addressBook, loadedConfig.admin);
  // Gladius Subscriptions
  await deployContract(
    'gladius_subscriptions_id',
    'gladius_subscriptions',
    addressBook,
    loadedConfig.admin
  );
  await bumpContractInstance('gladius_subscriptions_id', addressBook, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Initializing Gladius Emitter');
  console.log('-------------------------------------------------------');
  // Initializing Gladius Emitter
  const ratio = 1000;
  const gladius_subscriptions_id = addressBook.getContractId(network, 'gladius_subscriptions_id');
  const token_id = addressBook.getContractId(network, 'token_id');
  const emitterInitParams = [
    new Address(gladius_subscriptions_id).toScVal(), // admin
    new Address(token_id).toScVal(), // pegged
    nativeToScVal(ratio, { type: 'u32' }), // ratio
  ];
  console.log("Using emitterInitParams=[admin, pegged, ratio] : ", 
  [gladius_subscriptions_id,token_id, ratio])

  await invokeContract(
    'gladius_emitter_id',
    addressBook,
    'initialize',
    emitterInitParams,
    loadedConfig.admin
  );

  console.log('-------------------------------------------------------');
  console.log('Initializing Gladius Subscriptions');
  console.log('-------------------------------------------------------');
  
  // Initializing Gladius Subscriptions
  let admin_public = loadedConfig.admin.publicKey();
  let gladius_coin_emitter = addressBook.getContractId(network, 'gladius_emitter_id');

  const subscriptionsInitParams = [
    new Address(admin_public).toScVal(), // admin
    new Address(token_id).toScVal(), // payment_token
    new Address(gladius_coin_emitter).toScVal(), // gladius_coin_emitter
  ];

  console.log("Using subscriptionsInitParams=[admin, payment_token, gladius_coin_emitter] : ", 
  [admin_public,token_id, gladius_coin_emitter])

  await invokeContract(
    'gladius_subscriptions_id',
    addressBook,
    'initialize',
    subscriptionsInitParams,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();

const loadedConfig = config(network);

await deployAndInitContracts(addressBook);
addressBook.writeToFile();
