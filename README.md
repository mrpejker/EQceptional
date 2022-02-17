## EQCeptional

### Local development

1. Install dependencies
```bash
yarn
```
1. Build contract
```bash
yarn build
```
1. Deploy dev contract
```bash
yarn dev:deploy
```
or build and deploy simultaniously
```bash
yarn dev:build:deploy
```
1. Run acceptance tests
```bash
yarn test
```

### Testnet contract deployment

TBD

### API server deployment

1. Start API server locally bound to dev contract
```bash
yarn dev:server
```
1. Run acceptance tests for the API service
```bash
yarn test:server
```

### HTTPS Endpoints

1. GET / => "Hello"
1. GET /craft-hero?nearid=<> => {} Mints NFT for specified account