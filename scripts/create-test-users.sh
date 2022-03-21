near create-account company1.mtestaccount.testnet --masterAccount mtestaccount.testnet
near send mtestaccount.testnet company1.mtestaccount.testnet 300
near create-account admin1.company1.mtestaccount.testnet --masterAccount company1.mtestaccount.testnet
near create-account admin2.company1.mtestaccount.testnet --masterAccount company1.mtestaccount.testnet
near create-account admin3.company1.mtestaccount.testnet --masterAccount company1.mtestaccount.testnet

near create-account retailuser.mtestaccount.testnet --masterAccount mtestaccount.testnet
near send mtestaccount.testnet retailuser.mtestaccount.testnet 300
near create-account user1.retailuser.mtestaccount.testnet --masterAccount retailuser.mtestaccount.testnet
near create-account user2.retailuser.mtestaccount.testnet --masterAccount retailuser.mtestaccount.testnet
near create-account user3.retailuser.mtestaccount.testnet --masterAccount retailuser.mtestaccount.testnet
