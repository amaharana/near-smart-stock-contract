/**
 * Run from project root directory:
 *      near repl -s ./contract/deploy-stock-contract.js
 */

const fs = require('fs');
const deployerAccountName = "company1.mtestaccount.testnet";
const contractName = "stockcontract2.company1.mtestaccount.testnet";
const initArgs = {
    "ticker": "company1",
    "total_shares": 10000,
    "price_per_share": 2,
    "allowed_admin_caller": "multisig.company1.mtestaccount.testnet"
};

exports.main = async function(context) {
    const { near, nearAPI } = context;
    const account = await near.account(deployerAccountName);
    account.signAndSendTransaction(
        contractName,
        [
            nearAPI.transactions.createAccount(),
            nearAPI.transactions.transfer("100000000000000000000000000"),
            nearAPI.transactions.deployContract(fs.readFileSync("contract/target/wasm32-unknown-unknown/debug/greeter.wasm")),
            nearAPI.transactions.functionCall("new", Buffer.from(JSON.stringify(initArgs)), 10000000000000, "0"),
        ]
    ).then(result => console.log(result));
}
