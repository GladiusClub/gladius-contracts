# gladius-contracts
See [./docs](./docs/README.md) for the Gladius Economics and Documentation

## Setup
```
cp .env.example .env
```
Fill with your desired secret keys. If you don't set any, the scripts will create new accounts for you

## Contracts
In one terminal
```
bash scripts/quickstart.sh standalone
```

In other terminal
```
bash scripts/run.sh
```

1.- Test the contracts

```
cd contracts
make test
```

2.- Deploy the contracts
Here you can choose to deploy in standalone or in testnet
```
cd /workspace
bash scripts/deploy.sh standalone 
```

This will create the `.soroban/deployment.json` object that will be hidden.
If you want to publish it into the repo, just do

```
mkdir -p public
cp .soroban/deployments.json public/
```