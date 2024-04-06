// import { randomBytes } from 'crypto';
import { randomBytes } from 'crypto';
import { readFileSync } from 'fs';
import path from 'path';
import {
  Address,
  Asset,
  Contract,
  Keypair,
  Operation,
  StrKey,
  hash,
  scValToNative,
  xdr,
  nativeToScVal
} from 'stellar-sdk';
import { fileURLToPath } from 'url';
import { AddressBook, ContractNames, WasmKeys } from './address_book.js';
import { config } from './env_config.js';
import { createTxBuilder, invoke, invokeTransaction } from './tx.js';

// Relative paths from __dirname
const CONTRACT_REL_PATH: object = {
  gladius_coin_emitter:
    '../../contracts/gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.optimized.wasm',
  gladius_subscriptions:
    '../../contracts/gladius-subscriptions/target/wasm32-unknown-unknown/release/gladius_subscriptions.optimized.wasm',
  token:
    '../../contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm',
  gladius_nft:
  '../../contracts/gladius-nft/target/wasm32-unknown-unknown/release/gladius_nft.optimized.wasm',
};

const network = process.argv[2];
const loadedConfig = config(network);

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export async function installContract(
  wasmKey: keyof WasmKeys,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractWasm = readFileSync(
    path.join(__dirname, CONTRACT_REL_PATH[wasmKey as keyof object])
  );
  const wasmHash = hash(contractWasm);
  addressBook.setWasmHash(network, wasmKey, wasmHash.toString('hex'));
  console.log('Installing:', wasmKey, wasmHash.toString('hex'));
  const op = Operation.invokeHostFunction({
    func: xdr.HostFunction.hostFunctionTypeUploadContractWasm(contractWasm),
    auth: [],
  });
  addressBook.writeToFile();
  await invoke(op, source, false);
}

export async function deployContract(
  contractKey: keyof ContractNames,
  wasmKey: keyof WasmKeys,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractIdSalt = randomBytes(32);
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const contractIdPreimage = xdr.ContractIdPreimage.contractIdPreimageFromAddress(
    new xdr.ContractIdPreimageFromAddress({
      address: Address.fromString(source.publicKey()).toScAddress(),
      salt: contractIdSalt,
    })
  );

  const hashIdPreimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: contractIdPreimage,
    })
  );
  console.log('Deploying WASM', wasmKey, 'for', contractKey);
  const contractId = StrKey.encodeContract(hash(hashIdPreimage.toXDR()));
  addressBook.setAddress(network, contractKey, contractId);
  const wasmHash = Buffer.from(addressBook.getWasmHash(network, wasmKey), 'hex');

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: contractIdPreimage,
      executable: xdr.ContractExecutable.contractExecutableWasm(wasmHash),
    })
  );

  addressBook.writeToFile();
  await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );
}

export async function invokeContract(
  contractKey: keyof ContractNames,
  addressBook: AddressBook,
  method: string,
  params: xdr.ScVal[],
  source: Keypair
) {
  console.log('Invoking contract: ', contractKey, ' with method: ', method);
  const contractAddress = addressBook.getContractId(network, contractKey);
  const contractInstance = new Contract(contractAddress);

  const contractOperation = contractInstance.call(method, ...params);
  return await invoke(contractOperation, source, false);
}

export async function invokeCustomContract(
  contractId: string,
  method: string,
  params: xdr.ScVal[],
  source: Keypair
) {
  console.log('Invoking contract: ', contractId, ' with method: ', method);
  const contractInstance = new Contract(contractId);

  const contractOperation = contractInstance.call(method, ...params);
  return await invoke(contractOperation, source, false);
}

export async function deployStellarAsset(asset: Asset, source: Keypair) {
  const xdrAsset = asset.toXDRObject();
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const preimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
    })
  );
  const contractId = StrKey.encodeContract(hash(preimage.toXDR()));
  console.log('🚀 « deployed Stellar Asset:', contractId);

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: xdr.ContractIdPreimage.contractIdPreimageFromAsset(xdrAsset),
      executable: xdr.ContractExecutable.contractExecutableStellarAsset(),
    })
  );
  return await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );
}

export async function bumpContractInstance(
  contractKey: keyof ContractNames,
  addressBook: AddressBook,
  source: Keypair
) {
  const address = Address.fromString(addressBook.getContractId(network, contractKey));
  console.log('bumping contract instance: ', address.toString());
  const contractInstanceXDR = xdr.LedgerKey.contractData(
    new xdr.LedgerKeyContractData({
      contract: address.toScAddress(),
      key: xdr.ScVal.scvLedgerKeyContractInstance(),
      durability: xdr.ContractDataDurability.persistent(),
    })
  );
  const bumpTransactionData = new xdr.SorobanTransactionData({
    resources: new xdr.SorobanResources({
      footprint: new xdr.LedgerFootprint({
        readOnly: [contractInstanceXDR],
        readWrite: [],
      }),
      instructions: 0,
      readBytes: 0,
      writeBytes: 0,
    }),
    resourceFee: xdr.Int64.fromString('0'),
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    ext: new xdr.ExtensionPoint(0),
  });

  const txBuilder = await createTxBuilder(source);
  txBuilder.addOperation(Operation.extendFootprintTtl({ extendTo: 535670 })); // 1 year
  txBuilder.setSorobanData(bumpTransactionData);
  const result = await invokeTransaction(txBuilder.build(), source, false);
  // @ts-ignore
  console.log(result.status, '\n');
}

export async function bumpContractCode(
  wasmKey: keyof WasmKeys,
  addressBook: AddressBook,
  source: Keypair
) {
  console.log('bumping contract code: ', wasmKey);
  const wasmHash = Buffer.from(addressBook.getWasmHash(network, wasmKey), 'hex');
  const contractCodeXDR = xdr.LedgerKey.contractCode(
    new xdr.LedgerKeyContractCode({
      hash: wasmHash,
    })
  );
  const bumpTransactionData = new xdr.SorobanTransactionData({
    resources: new xdr.SorobanResources({
      footprint: new xdr.LedgerFootprint({
        readOnly: [contractCodeXDR],
        readWrite: [],
      }),
      instructions: 0,
      readBytes: 0,
      writeBytes: 0,
    }),
    resourceFee: xdr.Int64.fromString('0'),
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    ext: new xdr.ExtensionPoint(0),
  });

  const txBuilder = await createTxBuilder(source);
  txBuilder.addOperation(Operation.extendFootprintTtl({ extendTo: 535670 })); // 1 year
  txBuilder.setSorobanData(bumpTransactionData);
  const result = await invokeTransaction(txBuilder.build(), source, false);
  // @ts-ignore
  console.log(result.status, '\n');
}

export async function airdropAccount(user: Keypair) {
  try {
    console.log('Start funding');
    await loadedConfig.rpc.requestAirdrop(user.publicKey(), loadedConfig.friendbot);
    console.log('Funded: ', user.publicKey());
  } catch (e) {
    console.log(user.publicKey(), ' already funded');
  }
}

export async function deploySorobanToken(
  wasmKey: keyof WasmKeys,
  addressBook: AddressBook,
  source: Keypair
) {
  const contractIdSalt = randomBytes(32);
  const networkId = hash(Buffer.from(loadedConfig.passphrase));
  const contractIdPreimage = xdr.ContractIdPreimage.contractIdPreimageFromAddress(
    new xdr.ContractIdPreimageFromAddress({
      address: Address.fromString(source.publicKey()).toScAddress(),
      salt: contractIdSalt,
    })
  );

  const hashIdPreimage = xdr.HashIdPreimage.envelopeTypeContractId(
    new xdr.HashIdPreimageContractId({
      networkId: networkId,
      contractIdPreimage: contractIdPreimage,
    })
  );
  const contractId = StrKey.encodeContract(hash(hashIdPreimage.toXDR()));
  const wasmHash = Buffer.from(addressBook.getWasmHash(network, wasmKey), 'hex');

  const deployFunction = xdr.HostFunction.hostFunctionTypeCreateContract(
    new xdr.CreateContractArgs({
      contractIdPreimage: contractIdPreimage,
      executable: xdr.ContractExecutable.contractExecutableWasm(wasmHash),
    })
  );

  // addressBook.writeToFile();
  const result = await invoke(
    Operation.invokeHostFunction({
      func: deployFunction,
      auth: [],
    }),
    source,
    false
  );

  if (result) {
    return contractId;
  }
}

export async function getTokenBalance(contractId: string, from: string, source: Keypair) {
  const tokenContract = new Contract(contractId);
  const op = tokenContract.call('balance', new Address(from).toScVal());

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  if (parsedResult == 0) {
    return parsedResult;
  }
  const resultNumber = parseInt(parsedResult);
  return resultNumber;
}


export async function getIsRole(
  contractId: string,
  function_name: string,
  user: string,
  source: Keypair) {

  const subscriptionContract = new Contract(contractId);
  const op = subscriptionContract.call(function_name, new Address(user).toScVal());

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  return parsedResult;
}

export async function getTotalCourses(contractId: string, source: Keypair) {

  const subscriptionContract = new Contract(contractId);
  const op = subscriptionContract.call('get_total_courses');

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  return parsedResult;
}


export async function getURI(
  contractId: string,
  id: number,
  source: Keypair) {

  const nftContract = new Contract(contractId);
  const op = nftContract.call('token_uri', nativeToScVal(id, { type: 'u32' }));

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  return parsedResult;
}


export async function getTotalSupplyNFT(
  contractId: string,
  source: Keypair) {

  const nftContract = new Contract(contractId);
  const op = nftContract.call('total_supply');

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  return parsedResult;
}



export async function getNFTbyOwner(
  contractId: string,
  owner: string,
  index: number,
  source: Keypair) {

  const nftContract = new Contract(contractId);
  const op = nftContract.call('token_of_owner_by_index', new Address(owner).toScVal(),  nativeToScVal(index, { type: 'u32' }));

  const result = await invoke(op, source, true);
  const parsedResult = scValToNative(result.result.retval).toString();

  if (!parsedResult) {
    throw new Error('The operation has no result.');
  }
  return parsedResult;
}



