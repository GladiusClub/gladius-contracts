import { initializeApp, applicationDefault } from 'firebase-admin/app';
import { getFirestore } from 'firebase-admin/firestore';


initializeApp({
  credential: applicationDefault(),
  projectId: 'wallet-login-45c1c',
});

export const db = getFirestore();
