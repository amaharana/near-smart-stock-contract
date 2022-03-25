const { exit } = require('process');
const fs = require('fs');

/**
 * Run from project root directory:
 *      near repl -s ./contract/deploy-stock-contract.js -- company-a.mtestaccount.testnet multisig.commpany-a.mtestaccount.testnet
 */

var arguments = process.argv ;
// console.log("# arguments: " + arguments.length);
// arguments.forEach(arg => console.log(arg));

// first arg is path to "node", second arg is path to "near" and so on...
if (arguments.length != 8) {
    console.log("USAGE: near repl -s ./contract/deploy-stock-contract.js -- <deployment_account_id> <multisig_account_id>");
}

const deployerAccountName = arguments[6]; // "company-a.mtestaccount.testnet";
const contractName = "stockcontract." + deployerAccountName; //stockcontract.company-a.mtestaccount.testnet";
const initArgs = {
    "ticker": "MYCOMPANY",
    "total_shares": 100000,
    "price_per_share": 2,
    "allowed_admin_caller": arguments[7]
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
