import { existsSync, readFileSync, writeFileSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export interface ContractNames {
  pegged_token_admin_public: string;
  gladius_admin_public: string;
  token_id: string;
  gladius_emitter_id: string;
}

export interface WasmKeys {
  gladius_coin_emitter: string;
  gladius_subscriptions: string;
  token: string;
}

interface NetworkContracts {
  network: string;
  ids: ContractNames;
  hashes: WasmKeys;
}

export class AddressBook {
  private networks: NetworkContracts[];
  private fileName: string;

  constructor(networks: NetworkContracts[], fileName: string) {
    this.networks = networks;
    this.fileName = fileName;
  }

  static loadFromFile(fileName: string = 'deployments.json') {
    const filePath = path.join(__dirname, '../../.soroban/', fileName);
    let networks: NetworkContracts[];

    if (existsSync(filePath)) {
      const fileContent = readFileSync(filePath, { encoding: 'utf-8' });
      networks = JSON.parse(fileContent);
    } else {
      // If the file doesn't exist, create a new empty array for networks
      networks = [
        {
          network: 'standalone',
          ids: {
            pegged_token_admin_public: '',
            gladius_admin_public: '',
            token_id: '',
            gladius_emitter_id: '',
          },
          hashes: {
            gladius_coin_emitter: '',
            gladius_subscriptions: '',
            token: '',
          },
        },
        {
          network: 'testnet',
          ids: {
            pegged_token_admin_public: '',
            gladius_admin_public: '',
            token_id: '',
            gladius_emitter_id: '',
          },
          hashes: {
            gladius_coin_emitter: '',
            gladius_subscriptions: '',
            token: '',
          },
        },
        {
          network: 'mainnet',
          ids: {
            pegged_token_admin_public: '',
            gladius_admin_public: '',
            token_id: '',
            gladius_emitter_id: '',
          },
          hashes: {
            gladius_coin_emitter: '',
            gladius_subscriptions: '',
            token: '',
          },
        },
      ];
    }

    return new AddressBook(networks, fileName);
  }

  writeToFile() {
    const filePath = path.join(__dirname, '../../.soroban/', this.fileName);
    const fileContent = JSON.stringify(this.networks, null, 2);
    writeFileSync(filePath, fileContent);
  }

  setContractId(networkName: string, contractKey: keyof ContractNames, contractId: string) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      network.ids[contractKey] = contractId;
    } else {
      throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
    }
  }

  getContractId(networkName: string, contractKey: keyof ContractNames) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      return network.ids[contractKey];
    } else {
      throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
    }
  }

  setWasmHash(networkName: string, contractKey: keyof WasmKeys, wasmHash: string) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      network.hashes[contractKey] = wasmHash;
    } else {
      throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
    }
  }

  getWasmHash(networkName: string, contractKey: keyof WasmKeys) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      return network.hashes[contractKey];
    } else {
      throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
    }
  }

  getContractsByNetwork(networkName: string): NetworkContracts | undefined {
    const network = this.networks.find((n) => n.network === networkName);
    return network;
  }
}
