import { Address, Contract, Keypair, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { bumpContractCode, deploySorobanToken, installContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { invoke } from '../utils/tx.js';

const network = process.argv[2];

/**
 * Deploy a token contract and initialize it
 * @param name Name of the token
 * @param symbol Symbol of the token
 * @param addressBook AddressBook instance
 * @param source Keypair of the source account
 */
export async function deployToken(
  name: string,
  symbol: string,
  addressBook: AddressBook,
  source: Keypair
) {
  // Instaling token contract
  await installContract('token', addressBook, source);
  await bumpContractCode('token', addressBook, source);

  try {
    const contractId = await deploySorobanToken('token', addressBook, source);

    // Initializing Token
    const tokenInitParams = [
      new Address(source.publicKey()).toScVal(),
      nativeToScVal(7, { type: 'u32' }),
      nativeToScVal(name, { type: 'string' }),
      nativeToScVal(symbol, { type: 'string' }),
    ];

    const contractInstance = new Contract(contractId!);
    const contractOperation = contractInstance.call('initialize', ...tokenInitParams);
    const result = await invoke(contractOperation, source, false);

    if (result.status === 'SUCCESS') {
      addressBook.setContractId(network, 'token_id', contractId!);
      console.log(`Token ${symbol} deployed successfully with contractId: ${contractId}!`);
    }
    addressBook.writeToFile();
  } catch (error) {
    console.log('🚀 « error:', error);
  }
}
