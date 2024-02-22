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
bash scripts/quickstart.sh
```

In other terminal
```
bash scripts/run.sh
```

```
cd contracts
make test
```