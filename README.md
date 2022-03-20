## EQCeptional
Hello and welcome to the backend repository for EQceptional app! 
Our application is built with Unity and serves as emotional interlligence trainers. 
This part of the project supports the NFT rewards issuance for achivements through app interaction.  The demo version requires signing in through NEAR testnet account on which users can receive their prizes. 

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
yarn start
### run the server

### HTTPS Endpoints

1. GET / => "Hello"
1. GET /craft-hero?nearid=<> => {} Mints NFT for specified account

### Firebase
1. Set up:
```bash
firebase init
```
2. Deploy static files:
```bash
firebase deploy
```
3. Hosting: 
https://eqceptional-eaeab.web.app

