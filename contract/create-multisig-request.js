/**
 * Run from project root directory:
 *      near repl -s ./contract/create-multisig-request.js
 */

const confirmerAccountName = "admin1.company1.mtestaccount.testnet";
const contractName = "multisig.company1.mtestaccount.testnet";
const functionCallArgs = JSON.stringify({
    "num_new_shares": 10
});
const requestArgs = {
    "request": {
        "receiver_id": "stockcontract2.company1.mtestaccount.testnet",
        "actions": [
            {
                "type": "FunctionCall",
                "method_name": "issue_new_shares",

                // https://stackoverflow.com/questions/69438889/near-functioncall-args-field
                // https://stackoverflow.com/questions/38134200/base64-encode-a-javascript-object
                "args": Buffer.from(functionCallArgs).toString("base64"),

                "deposit": "0",
                "gas": "3428265333836"
            }
        ]
    }
};

exports.main = async function (context) {
    const { near, nearAPI } = context;
    const account = await near.account(confirmerAccountName);

    account.signAndSendTransaction(
        contractName,
        [
            nearAPI.transactions.functionCall("add_request_and_confirm", Buffer.from(JSON.stringify(requestArgs)), 10000000000000, "0"),
        ]
    ).then(result => console.log(result));
}
