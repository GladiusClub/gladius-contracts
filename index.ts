import { Request, Response } from 'express';

import { AddressBook } from './utils/address_book.js';
import { getTokenBalance } from './utils/contract.js';
import { config } from './utils/env_config.js';

export async function GladiusContracts(req: Request, res: Response): Promise<void> {

  const network = process.argv[2];
  const addressBook = AddressBook.loadFromFile();
  const loadedConfig = config(network);

  try {
      const balanceCheck = await getTokenBalance(
          addressBook.getContractId(network, 'token_id'),
          loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET').publicKey(),
          loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET')
      );
      console.log('🚀 « EURC balanceCheck:', balanceCheck);

      // Responding with the balance check result
      res.status(200).send(`EURC balanceCheck: ${balanceCheck}`);

  } catch (error) {
      console.error('Failed to check balance:', error);
      // Sending error response if something goes wrong
      res.status(500).send('Failed to check balance due to an error.');
  }
};
