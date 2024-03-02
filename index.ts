import express from 'express';
import { GladiusContracts } from './scripts/contractsfunction.js'; // Adjust the path as necessary

const app = express();
app.use(express.json());

// Define the route that uses your GladiusContracts function
app.post('/gladius-contracts', GladiusContracts);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`Server running on port ${PORT}`));
