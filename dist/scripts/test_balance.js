import { AddressBook } from '../utils/address_book.js';
import { getTokenBalance } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
export async function testGladius(addressBook) {
    const balanceCheck = await getTokenBalance(addressBook.getContractId(network, 'token_id'), loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey(), loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET'));
    console.log('🚀 « EURC balanceCheck:', balanceCheck);
}
const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();
const loadedConfig = config(network);
await testGladius(addressBook);
