beforeAll(async function () {
  // NOTE: nearlib and nearConfig are made available by near-cli/test_environment
  const near = await nearlib.connect(nearConfig)
  window.accountId = nearConfig.contractName
  window.contract = await near.loadContract(nearConfig.contractName, {
    viewMethods: ['get_shares_outstanding', 'get_total_shares', 'get_shares_owned', 'get_price_per_share'],
    changeMethods: ['buy_shares', 'sell_shares'],
    sender: window.accountId
  })

  window.walletConnection = {
    requestSignIn() {
    },
    signOut() {
    },
    isSignedIn() {
      return true
    },
    getAccountId() {
      return window.accountId
    }
  }
})

// TODO: add tests for other methods
test('get_shares_outstanding', async () => {
  const message = await window.contract.get_shares_outstanding({ account_id: window.accountId })
  expect(message).toEqual(1000000000)
})
