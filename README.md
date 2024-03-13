# Gladius Contracts: Setup and Deployment Guide

Welcome to the Gladius Contracts repository! This guide provides comprehensive instructions for setting up your development environment, testing, deploying, and interacting with the Gladius smart contracts. For detailed information on Gladius Economics and additional documentation, please refer to the [documentation directory](./docs/README.md).

This guide should help you get started with Gladius Contracts smoothly. For further assistance or more detailed documentation, please refer to the docs directory.

## Getting Started

### Environment Setup

To begin, copy the provided example environment file to create your own `.env` file where you can set your secret keys and other environment variables:

1.- 
```sh
cp .env.example .env
```
2.- 
Fill in the `.env` file with your desired secret keys as needed.

### Open Doker Containers:

To correctly work with the contracts, you'll need to start the following scripts in separate terminals.

- **Terminal 1:** Launch the quickstart script to set up the environment for contract interaction:

```sh
bash scripts/quickstart.sh standalone
```

- **Terminal 2:** Run the main script to start the `soroban-preview` container with the corret `soroban-cli` version:

```sh
bash scripts/run.sh
```

### Set up your addresses

Inside the you can run the following inside the `soroban-preview` docker container to generate secret keys and find them later on in `.soroban/identity` directory

```bash
bash scripts/setup.sh <network>
```

To show a secret of an account and the list of name you can use you can run

```bash
# this will show the list of available accounts
soroban config identity ls
# this will show the secret key of an account
soroban config identity show <name of account>
```

## Running the Contracts

### Testing and Building Contracts

Before deploying, ensure the contracts work as expected:

Navigate to the contracts directory and run the tests:

```sh
cd contracts
make test
```

#### Deploying Contracts

Deployment can be performed either in any of the **networks** configured in the `config.json` file. These are `standalone`, `futurenet` or `testnet`- Follow these steps to deploy your contracts:

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

This command generates a `.soroban/deployments.json` file. To make this deployment public within the repository:

```sh
mkdir -p public
cp .soroban/deployments.json public/
```

#### Testing on Blockchain (./soroban)

After deployment, you can test the contracts on the blockchain:

```bash
cd /workspace
yarn test <network>
```

This, by default will read the contrat addresses in `.soroban`, which is an ignored folder.

#### Testing on Blockchain (public)

If you want to test with the published addresss available in the `public` folder, please do:
```bash
yarn test testnet public
```
