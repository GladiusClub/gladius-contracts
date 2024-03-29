import { initializeApp, applicationDefault } from 'firebase-admin/app';
import { getFirestore } from 'firebase-admin/firestore';


initializeApp({
  credential: applicationDefault(),
  projectId: 'wallet-login-45c1c',
});

const db = getFirestore();
const usersCollectionRef = db.collection('users');


async function GetStellarWallets() {
  const snapshot = await usersCollectionRef.get();

  snapshot.forEach(doc => {
    const userData = doc.data();
    console.log(`User ID: ${doc.id}, Stellar Wallet: ${userData.stellar_wallet}`);
  });
}

GetStellarWallets().catch(console.error);
