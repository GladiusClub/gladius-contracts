import { config } from '../utils/env_config.js';

// Retrieves the network argument from the command line
const network = process.argv[2];

if (!network) {
  console.error('Network argument is missing. Usage: node script.js [network]');
  process.exit(1);
}

export async function outputSecrets() {
  const loadedConfig = config(network); // Pass the network argument to the config function

  let gladius_admin = loadedConfig.admin;
  let payment_token_admin = loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET');
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let parent = loadedConfig.getUser('PARENT_SECRET');
  let student = loadedConfig.getUser('STUDENT_SECRET');

  console.log('Gladius Admin Secret:', gladius_admin.publicKey());
  console.log('Payment Token Admin Secret:', payment_token_admin.publicKey());
  console.log('Sport Club Secret:', sport_club.publicKey());
  console.log('Parent Secret:', parent.publicKey());
  console.log('Student Secret:', student.publicKey());
}

await outputSecrets();
