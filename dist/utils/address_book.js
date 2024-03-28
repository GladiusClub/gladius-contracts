import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
export class AddressBook {
    networks;
    fileName;
    constructor(networks, fileName) {
        this.networks = networks;
        this.fileName = fileName;
    }
    static loadFromFile(network, folder = '.soroban') {
        const fileName = `deployments.json`;
        const filePath = path.join(__dirname, `../../${folder}/`, fileName);
        console.log("🚀 ~ AddressBook ~ loadFromFile ~ filePath:", filePath);
        let networks;
        if (existsSync(filePath)) {
            const fileContent = readFileSync(filePath, { encoding: 'utf-8' });
            networks = JSON.parse(fileContent);
        }
        else {
            // If the file doesn't exist, create a new empty array for networks
            networks = [
                {
                    network: 'standalone',
                    ids: {
                        payment_token_admin_public: '',
                        gladius_admin_public: '',
                        token_id: '',
                        gladius_emitter_id: '',
                        gladius_subscriptions_id: '',
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
                        payment_token_admin_public: '',
                        gladius_admin_public: '',
                        token_id: '',
                        gladius_emitter_id: '',
                        gladius_subscriptions_id: '',
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
                        payment_token_admin_public: '',
                        gladius_admin_public: '',
                        token_id: '',
                        gladius_emitter_id: '',
                        gladius_subscriptions_id: '',
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
        const dirPath = path.join(__dirname, '../../.soroban/');
        const filePath = path.join(dirPath, this.fileName);
        if (!existsSync(dirPath)) {
            console.log(".soroban does not exist, will create dir");
            mkdirSync(dirPath, { recursive: true });
        }
        const fileContent = JSON.stringify(this.networks, null, 2);
        writeFileSync(filePath, fileContent);
    }
    setAddress(networkName, contractKey, contractId) {
        const network = this.networks.find((n) => n.network === networkName);
        if (network) {
            network.ids[contractKey] = contractId;
        }
        else {
            throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
        }
    }
    getContractId(networkName, contractKey) {
        const network = this.networks.find((n) => n.network === networkName);
        if (network) {
            return network.ids[contractKey];
        }
        else {
            throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
        }
    }
    setWasmHash(networkName, contractKey, wasmHash) {
        const network = this.networks.find((n) => n.network === networkName);
        if (network) {
            network.hashes[contractKey] = wasmHash;
        }
        else {
            throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
        }
    }
    getWasmHash(networkName, contractKey) {
        const network = this.networks.find((n) => n.network === networkName);
        if (network) {
            return network.hashes[contractKey];
        }
        else {
            throw new Error(`Network ${networkName} not found`); // Error if network doesn't exist
        }
    }
    getContractsByNetwork(networkName) {
        const network = this.networks.find((n) => n.network === networkName);
        return network;
    }
}
