import { Address, nativeToScVal } from 'stellar-sdk';
import { invokeCustomContract } from '../utils/contract.js';
/**
 * Mint tokens to a user
 * @param contractId Contract ID of the token
 * @param amount Amount to mint to the user
 * @param to Public key of the user
 * @param signer Keypair of the signer
 */
export async function mintToken(contractId, amount, to, signer) {
    try {
        const mintTokensParams = [
            new Address(to).toScVal(),
            nativeToScVal(amount, { type: 'i128' }),
        ];
        return await invokeCustomContract(contractId, 'mint', mintTokensParams, signer);
    }
    catch (error) {
        console.log('🚀 « error:', error);
    }
}
