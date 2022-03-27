near-smart-stock-contract
==================

This repo consists of 2 parts: A smart contract that can be deployed on the NEAR blockchain, and a REACT webapp to interact with the smart contract.

Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn
   run test`.


High-Level Solution Architecture
================================

There are 3 main components of the solution (2 in this package and 1 external):
1. Smart Stock Contract (SSC): This smart contract appears in the /contract folder and respresents the shares of a single company. This contract's storage keeps track of the ticker symbol, total shares issues, total outstanding shares available to buy, number of shares owned by shareholders (NEAR wallet owners). It also provides methods that can be invoked by 2 class of users:
    
    1.1 Retail investors - can buy and sell shares.

    1.1 Privileged or "admin" users - can issue new shares, initiate share buy-backs etc. These are authorized users with the ability to performa these actions on behalf of the company. The authorization part is enforced using a multi-sig contract described below.

2. [Multi-Sig contract] (MSC) from NEAR core contracts: This smart contract enforces K of N confirmations to enable privileged actions in SSC. During initialization, MSC is supplied with the public keys (or account IDs) of the N authorized users, and how many confirmations (K) are needed to execute privileged actions. When SSC is initialized, it is supplied with the deployed contract address of corresponding MSC. After this initiatilization, only the MSC can invoke privileged actions in SSC, after obtaining K confirmations.

3. dApp: This React web application provides the UI for interacting with SSC. In future, it can be extended to provide admin operations that involve MSC.


In the diagram below, boxes in dotted lines are planned for future and have not been built at this time.

![High Level Architecture Diagram](https://raw.githubusercontent.com/amaharana/near-smart-stock-contract/master/diagrams/HighLevelArchitecture.drawio.png)

Pre-requisites for building
===========================
1. Make sure you've installed [Node.js] ≥ v17.6.0 and [Rust] compiler ≥ 1.59.0
2. Install NEAR CLI ≥ 3.2.0 `yarn global add near-cli && near --version`

Build Multisig
==============
```
git clone https://github.com/near/core-contracts
cd core-contracts/multisig2
./build.sh
```

Build SSC and dApp (this repo)
==============================
```
https://github.com/amaharana/near-smart-stock-contract
cd near-smart-stock-contract
yarn install
yarn build
```

Create test accounts
====================
1. Create a testnet "master" account at https://wallet.testnet.near.org/ (let's call it mtestaccount.testnet, ***replace your testnet account name in all commands below***)
2. Save the credentials to use this account from cli: `near login` and login using the wallet created above
3. Run script to create the demo users
```
cd near-smart-stock-contract/scripts
./create-test-accounts.sh mtestaccount.testnet company-a investorpool-a
```

Deploy and Initialize Contracts
===============================

Multisig Contract
-----------------

Update the script below with your testnet account name and paste it at the repl prompt. It will create a new subaccount to hold the multisig contract, deploy the wasm to this account, and initialize it to allow confirmation by 2 out of 3 test admin accounts created in steps above.

```
cd core-contracts/multisig2
near repl
```

```
// TODO: Parameterize this script to eliminate copy/paste monkey business
const fs = require('fs');
const account = await near.account("company-a.mtestaccount.testnet");
const contractName = "multisig.company-a.mtestaccount.testnet";
const methodNames = ["add_request","delete_request","confirm"];
const newArgs = {"num_confirmations": 2, "members": [
        { "account_id": "admin1.company-a.mtestaccount.testnet" },
        { "account_id": "admin2.company-a.mtestaccount.testnet" },
        { "account_id": "admin3.company-a.mtestaccount.testnet" },
    ]};
const result = account.signAndSendTransaction(
    contractName,
    [
        nearAPI.transactions.createAccount(),
        nearAPI.transactions.transfer("100000000000000000000000000"),
        nearAPI.transactions.deployContract(fs.readFileSync("res/multisig2.wasm")),
        nearAPI.transactions.functionCall("new", Buffer.from(JSON.stringify(newArgs)), 10000000000000, "0"),
    ]);
```

Stock Contract
--------------
This script will deploy the stock contract to `stockcontract.company-a.mtestaccount.testnet` and initialize it with defaults. 
Privileged calls will only be accepted from `multisig.company-a.mtestaccount.testnet`.

```
cd near-smart-stock-contract
near repl -s ./contract/deploy-stock-contract.js -- company-a.mtestaccount.testnet multisig.company-a.mtestaccount.testnet
```
Edit `near-smart-stock-contract/neardev/dev-account` and `near-smart-stock-contract/dev-account.env` and replace the contract name with `stockcontract.company-a.mtestaccount.testnet`.

Run
===

Contract
--------
Sample NEAR CLI commands to check that the contract are working as expected:

```
// see if total shares is set correctly
near view stockcontract2.company-a.mtestaccount.testnet get_total_shares

// try increasing total shares by direct call - should fail
near call stockcontract2.company-a.mtestaccount.testnet issue_new_shares '{"num_new_shares": 10}' --accountId admin2.company-a.mtestaccount.testnet

// see open request IDs in multisig contract
near view multisig.company-a.mtestaccount.testnet list_request_ids

// create a new multisig request to increase total shares (and provide the first confirmation)
near repl -s ./contract/create-multisig-request.js

// verify request
near view multisig.company-a.mtestaccount.testnet list_request_ids
near view multisig.company-a.mtestaccount.testnet get_confirmations '{"request_id":2}'


// try to confirm confirm the request as second user who is not in the list of confirmers provided during init - should fail
near call multisig.company-a.mtestaccount.testnet confirm '{"request_id":2}' --account_id admin2.company1.mtestaccount.testnet

// confirm second request correctly
near call multisig.company-a.mtestaccount.testnet confirm '{"request_id":2}' --account_id admin2.company-a.mtestaccount.testnet

// verify that multisig request successfully updated total shares
near view stockcontract2.company-a.mtestaccount.testnet get_total_shares
```

UI
--

Start the local development server: `yarn start` (see `package.json` for a
   full list of `scripts` you can run with `yarn`). You can test buying and selling shares using the UI.
   
Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [React]: https://reactjs.org/
  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
  [Multi-Sig contract]: (https://github.com/near/core-contracts/tree/master/multisig2)
  [Rust]: (https://www.rust-lang.org)
