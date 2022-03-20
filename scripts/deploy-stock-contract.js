
const fs = require('fs');
const account = await near.account("company1.mtestaccount.testnet");
const contractName = "stockcontract.company1.mtestaccount.testnet";
const newArgs = {"ticker": "company1", 
    "total_shares": 10000,
    "price_per_share": 2, 
    "allowed_admin_caller":"multisig.company1.mtestaccount.testnet"
};

const result = account.signAndSendTransaction(
    contractName,
    [
        nearAPI.transactions.createAccount(),
        nearAPI.transactions.transfer("100000000000000000000000000"),
        nearAPI.transactions.deployContract(fs.readFileSync("contract/target/wasm32-unknown-unknown/debug/greeter.wasm")),
        nearAPI.transactions.functionCall("new", Buffer.from(JSON.stringify(newArgs)), 10000000000000, "0"),
    ]);

