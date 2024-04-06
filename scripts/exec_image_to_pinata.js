import axios from 'axios';
import FormData from 'form-data';
import fs from 'fs';
//const pinataSDK = require('@pinata/sdk');
import pinataSDK from '@pinata/sdk';


const { PINATA_API_KEY, PINATA_API_SECRET } = process.env;
console.log("🚀 ~ PINATA_API_SECRET:", PINATA_API_SECRET)
console.log("🚀 ~ PINATA_API_KEY:", PINATA_API_KEY)

const filePath = '/workspace/img/gladius_club_nft.png';
const filename = path.basename(filePath);
const baseName = path.parse(filename).name;

export async function pinFileToIPFS() {

    
    const pinata = new pinataSDK(PINATA_API_KEY, PINATA_API_SECRET);
    const stream = fs.createReadStream(filePath);
    const options = {
      pinataMetadata: {
        name: `${baseName}.png`,
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
        name: "gladius nft",
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

(async () => {
    try {
      const nftUri = await pinFileToIPFS();
      console.log('NFT URI:', nftUri);
    } catch (error) {
      console.error('Error pinning file to IPFS:', error);
    }
  })();
  