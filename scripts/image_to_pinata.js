import fs from 'fs';
import path from 'path';
// import pinataSDK from '@pinata/sdk';

const pinataSDK = require('@pinata/sdk');

const PINATA_API_KEY: string | undefined = process.env.PINATA_API_KEY;
const PINATA_API_SECRET: string | undefined = process.env.PINATA_API_SECRET;

console.log("🚀 ~ PINATA_API_SECRET:", PINATA_API_SECRET);
console.log("🚀 ~ PINATA_API_KEY:", PINATA_API_KEY);

export async function pinFileToIPFS(filePath: string, NftName:string ): Promise<string> {
  //const filePath: string = '/workspace/img/golden_badge.jpg';
  
  const filename: string = path.basename(filePath);
  const baseName: string = path.parse(filename).name;
  const fileExt = path.extname(filePath);
  console.log(`upload to IPFS ${NftName} from `,  filePath);


    if (!PINATA_API_KEY || !PINATA_API_SECRET) {
        console.error("Pinata API Key or Secret is undefined.");
        return "Pinata API Key or Secret is undefined";
    }
  
    const pinata = new pinataSDK(PINATA_API_KEY, PINATA_API_SECRET);
    const stream = fs.createReadStream(filePath);
    const options = {
        pinataMetadata: {
            name: `${baseName}${fileExt}`,
        },
    };

    // Pin the file to IPFS
    const result = await pinata.pinFileToIPFS(stream, options);
    console.log(result);
    const ipfsHash = result.IpfsHash;

    const jsonOptions = {
      pinataMetadata: {
        name: `${baseName}.json`,
      },
    };

    const jsonContent = {
        name: NftName,
        img_url: `https://gateway.pinata.cloud/ipfs/${ipfsHash}`
    };

    // Pin the JSON content to IPFS
    const jsonResult = await pinata.pinJSONToIPFS(jsonContent, jsonOptions);
    const nft_uri = `https://gateway.pinata.cloud/ipfs/${jsonResult.IpfsHash}`;
    console.log("NFT URI:", nft_uri);

    const nftUriFilePath = '/workspace/.soroban/nft_uri';

    // Write the NFT URI to a file
    await fs.promises.writeFile(nftUriFilePath, nft_uri);
    console.log('NFT URI has been written to the file successfully.');
    return nft_uri;
}
