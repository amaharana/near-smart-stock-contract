#!/bin/bash

let $# || { echo "Usage: create-test-users.sh <master_account> <admin_account_prefix> <investor_account_prefix>"; exit 1; } 

master_account=$1
admin_account=$2.$1
investor_account=$3.$1

# Demo admin users who can issue new shares, buy-back existing shares, and perform other "admin" functions
near create-account $admin_account --masterAccount $master_account
near send $master_account $admin_account 400
near create-account admin1.$admin_account --masterAccount $admin_account
near create-account admin2.$admin_account --masterAccount $admin_account
near create-account admin3.$admin_account --masterAccount $admin_account

# Demo investor users who can buy and sell shares
near create-account $investor_account --masterAccount $master_account
near send $master_account $investor_account 300
near create-account user1.$investor_account --masterAccount $investor_account
near create-account user2.$investor_account --masterAccount $investor_account
near create-account user3.$investor_account --masterAccount $investor_account
