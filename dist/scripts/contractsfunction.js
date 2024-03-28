import { AddressBook } from '../utils/address_book.js';
import { getTokenBalance } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
export async function GladiusContracts(req, res) {
    // Use 'testnet' as the default value for 'network' if it's not provided
    const network = req.query.network || req.body.network || 'testnet';
    const loadedConfig = config(network);
    // Use provided 'public_key' or default to 'PEGGED_TOKEN_ADMIN_SECRET' public key if not provided
    const publicKey = req.query.public_key || req.body.public_key || loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET').publicKey();
    const addressBook = AddressBook.loadFromFile();
    try {
        const balanceCheck = await getTokenBalance(addressBook.getContractId(network, 'token_id'), publicKey, // Use the resolved publicKey (either provided or default)
        loadedConfig.getUser('PEGGED_TOKEN_ADMIN_SECRET'));
        console.log('🚀 « EURC balanceCheck:', balanceCheck);
        // Responding with the balance check result
        res.status(200).send(`EURC balanceCheck: ${balanceCheck}`);
    }
    catch (error) {
        console.error('Failed to check balance:', error);
        // Sending error response if something goes wrong
        res.status(500).send('Failed to check balance due to an error.');
    }
}
