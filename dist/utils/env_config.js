import dotenv from 'dotenv';
import * as fs from 'fs';
import path from 'path';
import { Keypair, SorobanRpc } from 'stellar-sdk';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
dotenv.config({ path: path.join(__dirname, '../../.env') });
class EnvConfig {
    rpc;
    passphrase;
    friendbot;
    admin;
    constructor(rpc, passphrase, friendbot, admin) {
        this.rpc = rpc;
        this.passphrase = passphrase;
        this.friendbot = friendbot;
        this.admin = admin;
    }
    /**
     * Load the environment config from the .env file
     * @returns Environment config
     */
    static loadFromFile(network) {
        const fileContents = fs.readFileSync(path.join(__dirname, '../../config.json'), 'utf8');
        const configs = JSON.parse(fileContents);
        let rpc_url, friendbot_url, passphrase;
        const networkConfig = configs.networkConfig.find((config) => config.network === network);
        if (!networkConfig) {
            throw new Error(`Network configuration for '${network}' not found`);
        }
        rpc_url = networkConfig.soroban_rpc_url;
        friendbot_url = networkConfig.friendbot_url;
        passphrase = networkConfig.soroban_network_passphrase;
        if (rpc_url === undefined ||
            (network != 'mainnet' && friendbot_url === undefined) ||
            passphrase === undefined ||
            process.env.GLADIUS_ADMIN_SECRET === undefined) {
            console.log("🚀 ~ EnvConfig ~ loadFromFile ~ (network != 'mainnet' && friendbot_url === undefined):", (network != 'mainnet' && friendbot_url === undefined));
            console.log("🚀 ~ EnvConfig ~ loadFromFile ~ rpc_url === undefined:", rpc_url === undefined);
            console.log("🚀 ~ EnvConfig ~ loadFromFile ~ passphrase === undefined:", passphrase === undefined);
            console.log("🚀 ~ EnvConfig ~ loadFromFile ~ process.env.GLADIUS_ADMIN_SECRET === undefined:", process.env.GLADIUS_ADMIN_SECRET === undefined);
            throw new Error('Error: Configuration is missing required fields, include <network>');
        }
        const admin = process.env.GLADIUS_ADMIN_SECRET;
        const allowHttp = network === 'standalone';
        return new EnvConfig(new SorobanRpc.Server(rpc_url, { allowHttp }), passphrase, friendbot_url, Keypair.fromSecret(admin));
    }
    /**
     * Get the Keypair for a user from the env file
     * @param userKey - The name of the user in the env file
     * @returns Keypair for the user
     */
    getUser(userKey) {
        const userSecretKey = process.env[userKey];
        if (userSecretKey != undefined) {
            return Keypair.fromSecret(userSecretKey);
        }
        else {
            throw new Error(`${userKey} secret key not found in .env`);
        }
    }
}
export const config = (network) => {
    return EnvConfig.loadFromFile(network);
};
