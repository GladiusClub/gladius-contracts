import { Address, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { airdropAccount, bumpContractCode, bumpContractInstance, deployContract, installContract, invokeContract, } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { deployToken } from './deploy_token.js';
export async function deployAndInitContracts(addressBook) {
    // Setting up public address for gladius accounts
    let gladius_admin = loadedConfig.admin;
    addressBook.setAddress(network, 'gladius_admin_public', gladius_admin.publicKey());
    if (network != 'mainnet') {
        // Gladius EURC Token
        console.log('-------------------------------------------------------');
        console.log('Because we are not in Mainnet.....');
        await airdropAccount(gladius_admin);
        await airdropAccount(loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET'));
        await airdropAccount(loadedConfig.getUser('SPORT_CLUB_SECRET'));
        await airdropAccount(loadedConfig.getUser('PARENT_SECRET'));
        await airdropAccount(loadedConfig.getUser('STUDENT_SECRET'));
        console.log('Deploying and Initializing  a Payment EURC token');
        console.log('-------------------------------------------------------');
        addressBook.setAddress(network, 'payment_token_admin_public', loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET').publicKey());
        console.log('Airdropping to all accounts');
        await deployToken('EURC Token', 'EURC', addressBook, loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET'));
    }
    console.log('-------------------------------------------------------');
    console.log('Installing Gladius Coin Emitter Contract');
    console.log('-------------------------------------------------------');
    await installContract('gladius_coin_emitter', addressBook, gladius_admin);
    await bumpContractCode('gladius_coin_emitter', addressBook, gladius_admin);
    console.log('-------------------------------------------------------');
    console.log('Installing Gladius Coin Subscriptions Contracts');
    console.log('-------------------------------------------------------');
    console.log('-------------------------------------------------------');
    await installContract('gladius_subscriptions', addressBook, gladius_admin);
    await bumpContractCode('gladius_subscriptions', addressBook, gladius_admin);
    console.log('-------------------------------------------------------');
    console.log('Deploying Gladius Coin Emitter Contract');
    console.log('-------------------------------------------------------');
    await deployContract('gladius_emitter_id', 'gladius_coin_emitter', addressBook, gladius_admin);
    await bumpContractInstance('gladius_emitter_id', addressBook, gladius_admin);
    console.log('-------------------------------------------------------');
    console.log('Deploying Gladius Subscriptions Contract');
    console.log('-------------------------------------------------------');
    await deployContract('gladius_subscriptions_id', 'gladius_subscriptions', addressBook, gladius_admin);
    await bumpContractInstance('gladius_subscriptions_id', addressBook, gladius_admin);
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
    console.log("Using emitterInitParams=[admin, pegged, ratio] : ", [gladius_subscriptions_id, token_id, ratio]);
    await invokeContract('gladius_emitter_id', addressBook, 'initialize', emitterInitParams, gladius_admin);
    console.log('-------------------------------------------------------');
    console.log('Initializing Gladius Subscriptions');
    console.log('-------------------------------------------------------');
    // Initializing Gladius Subscriptions
    let admin_public = gladius_admin.publicKey();
    let gladius_coin_emitter = addressBook.getContractId(network, 'gladius_emitter_id');
    const subscriptionsInitParams = [
        new Address(admin_public).toScVal(), // admin
        new Address(token_id).toScVal(), // payment_token
        new Address(gladius_coin_emitter).toScVal(), // gladius_coin_emitter
    ];
    console.log("Using subscriptionsInitParams=[admin, payment_token, gladius_coin_emitter] : ", [admin_public, token_id, gladius_coin_emitter]);
    await invokeContract('gladius_subscriptions_id', addressBook, 'initialize', subscriptionsInitParams, gladius_admin);
}
const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const loadedConfig = config(network);
await deployAndInitContracts(addressBook);
addressBook.writeToFile();
