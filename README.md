# Gladius Contracts: Setup and Deployment Guide

Welcome to the Gladius Contracts repository! This guide provides comprehensive instructions for setting up your development environment, testing, deploying, and interacting with the Gladius smart contracts. For detailed information on Gladius Economics and additional documentation, please refer to the [documentation directory](./docs/README.md).

## Getting Started

### Environment Setup

To begin, copy the provided example environment file to create your own `.env` file where you can set your secret keys and other environment variables:

```sh
cp .env.example .env
```

Fill in the `.env` file with your desired secret keys as needed.

## Running the Contracts

To work with the contracts, you'll need to start the necessary scripts in separate terminals.

- **Terminal 1:** Launch the quickstart script to set up the environment for contract interaction:

```sh
bash scripts/quickstart.sh standalone
```

- **Terminal 2:** Run the main script to start the contract processes:

```sh
bash scripts/run.sh
```

### Testing and Deployment

#### Testing Contracts

Before deploying, ensure the contracts work as expected:

1. Navigate to the contracts directory and run the tests:

```sh
cd contracts
make test
```

#### Deploying Contracts

Deployment can be performed either in a standalone mode or on a testnet. Follow these steps to deploy your contracts:

1. **Build the Contracts:**

First, compile the contracts:

```bash
cd /workspace/contracts
make build
```

2. **Build the TypeScript Scripts:**

Then, install dependencies and compile the TypeScript scripts required for deployment:

```bash
cd /workspace
yarn install
yarn build
```

3. **Deploy the Contracts:**

Choose your network (either `standalone` or `testnet`) and deploy the contracts:

```sh
cd /workspace
yarn setup <network>
```

This command generates a `.soroban/deployment.json` file. To make this deployment public within the repository:

```sh
mkdir -p public
cp .soroban/deployments.json public/
```

#### Testing on Blockchain

After deployment, you can test the contracts on the blockchain:

```bash
cd /workspace
yarn test <network>
```

**Note:** If you modify the TypeScript scripts, such as adding more tests, remember to rebuild before running. Use the following one-liner to streamline the process:

```bash
yarn build && yarn setup <network>
# or
yarn build && yarn test <network>
```

This guide should help you get started with Gladius Contracts smoothly. For further assistance or more detailed documentation, please refer to the docs directory.
