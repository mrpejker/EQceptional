const express = require("express");
const http = require("http");
const nacl = require("tweetnacl");
nacl.util = require("tweetnacl-util");
const cors = require("cors");
const fs = require("fs");
const nearAPI = require("near-api-js");
const getConfig = require("./config/near");

// Instantiate server with WS support
const app = express();
const port = process.env.PORT || 8080;
const server = http.createServer(app);
// const io = require("socket.io")(server, {
//   cors: true,
//   origins: ["*"],
// });

const nearConfig = getConfig(process.env.APP_ENV || "development");
const { nodeUrl, networkId, contractName } = nearConfig;
const contractMethods = {
  changeMethods: ["craft_new_hero", "update_hero_stats"],
  viewMethods: ["get_stats"],
};

const {
  keyStores: { InMemoryKeyStore },
  Near,
  Account,
  Contract,
  KeyPair,
  utils: {
    format: { parseNearAmount },
  },
} = nearAPI;

// Load credentials
console.log("Loading Credentials:\n", `./creds/${contractName}.json`);
const credentials = JSON.parse(fs.readFileSync(`./creds/${contractName}.json`));

const keyStore = new InMemoryKeyStore();
keyStore.setKey(
  networkId,
  contractName,
  KeyPair.fromString(credentials.private_key)
);
const near = new Near({
  networkId,
  nodeUrl,
  deps: { keyStore },
});
const { connection } = near;
const contractAccount = new Account(connection, contractName);
contractAccount.addAccessKey = (publicKey) =>
  contractAccount.addKey(
    publicKey,
    contractName,
    contractMethods.changeMethods,
    parseNearAmount("0.1")
  );

const contract = new Contract(contractAccount, contractName, contractMethods);

///   API   ///
app.get("/", (req, res) => {
  res.send("Hello from Murkwood Tale's contract server!");
});

// Balance of a single player or list of NFT rewards
app.get("/craft-hero", async (req, res) => {
  let result = "None";
  // const username = req.query.nearid.slice(1, -1);
  // const request = req.query.qr.slice(1, -1);
  // const gas_cost = 300000000000000;
  // const minting_cost = "100000000000000000000000";
  // console.log("Incoming action: {} {}", username, request);

  // result = await contract
  //   .checkin({
  //     args: { username, request },
  //     gas: gas_cost,
  //     amount: minting_cost,
  //   })
  //   .catch((err) => {
  //     console.log(err);
  //     res.status(200).send();
  //   });
  // console.log(result);
  res.json(result);
});

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
