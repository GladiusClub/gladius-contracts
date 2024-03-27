import { readFile } from 'fs/promises';

async function loadSecrets() {
  try {
    const data = await readFile('./testnet_keys.json', 'utf8');
    const secrets = JSON.parse(data);

    const payment_token_admin = secrets.PAYMENT_TOKEN_ADMIN_SECRET;
    const gladius_admin = secrets.GLADIUS_ADMIN_SECRET;
    const sport_club = secrets.SPORT_CLUB_SECRET;
    const parent = secrets.PARENT_SECRET;
    const student = secrets.STUDENT_SECRET;

    console.log(`Payment Token Admin: ${payment_token_admin}`);
    console.log(`Gladius Admin: ${gladius_admin}`);
    console.log(`Sport Club: ${sport_club}`);
    console.log(`Parent: ${parent}`);
    console.log(`Student: ${student}`);
  } catch (error) {
    console.error('Error reading the secrets file:', error);
  }
}

loadSecrets();
