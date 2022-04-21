import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import BN from 'bn.js';

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  const ONE_NEAR = new BN("1000000000000000000000000");
  const [pricePerShare, setPricePerShare] = React.useState()
  const [sharesOwned, setSharesOwned] = React.useState()
  const [sharesOutstanding, setSharesOutstanding] = React.useState()

  // toggle this flag to indicate that contract data has changed 
  // and all on-screen representations should be re-rendered
  const [contractCallRefreshFlag, setContractCallRefreshFlag] = React.useState()

  // when the user has not yet interacted with the form, disable the button
  const [formsDisabled, setFormsDisabled] = React.useState(false)

  React.useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {

        // window.contract is set by initContract in index.js
        window.contract.get_price_per_share()
          .then(price => {
            setPricePerShare(price)
          })

        window.contract.get_shares_outstanding()
          .then(sharesOutstanding => {
            setSharesOutstanding(sharesOutstanding)
          })

        // TODO: This doesn't work. Contract method has a bug.
        window.contract.get_shares_owned({ account_id: window.accountId })
          .then(sharesOwned => {
            setSharesOwned(sharesOwned)
          })
      }
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render".
    // Pass a variable to specify "run when this variable state changes."
    [contractCallRefreshFlag]
  )
  
  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>Welcome to NEAR!</h1>
        <p>
          To make use of the NEAR blockchain, you need to sign in. The button
          below will sign you in using NEAR Wallet.
        </p>
        <p>
          By default, when your app runs in "development" mode, it connects
          to a test network ("testnet") wallet. This works just like the main
          network ("mainnet") wallet, but the NEAR Tokens on testnet aren't
          convertible to other currencies – they're just for testing!
        </p>
        <p>
          Go ahead and click the button below to try it out:
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          Welcome {window.accountId}!
        </h1>
        <form onSubmit={async event => {
          event.preventDefault()

          // get elements from the form using their id attribute
          const { fieldsetForBuy, sharesToBuy } = event.target.elements

          // hold onto new user-entered value from React's SynthenticEvent for use after `await` call
          const numSharesToBuy = parseInt(sharesToBuy.value)

          // disable the forms while the value gets updated on-chain
          setFormsDisabled(true)

          // TODO: Does not work :(
          try {
            const buyCost = ONE_NEAR.muln(pricePerShare).muln(numSharesToBuy);
            const gas = new BN('30000000000000');

            // make an update call to the smart contract
            await window.contract.buy_shares({
              // pass the number of shares that the user entered in the form
              num_shares: numSharesToBuy
            }, gas, buyCost)

            // Don't need to toggle 'contractCallRefreshFlag' since buy involves a apage reload
          } catch (e) {
            alert(
              'Something went wrong! ' +
              'Maybe you need to sign out and back in? ' +
              'Check your browser console for more info.'
            )
            throw e
          } finally {
            // re-enable the forms, whether the call succeeded or failed
            setFormsDisabled(false)
          }
        }}>
          <fieldset id="fieldsetForBuy">
            <label
              htmlFor="sharesToBuy"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Number of shares to buy
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                id="sharesToBuy"
                style={{ flex: 1 }}
              />
              <button
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Buy
              </button>
            </div>
          </fieldset>
        </form>

        <form onSubmit={async event => {
          event.preventDefault()

          // get elements from the form using their id attribute
          const { fieldsetForSell, sharesToSell } = event.target.elements

          // hold onto new user-entered value from React's SynthenticEvent for use after `await` call
          const numSharesToSell = parseInt(sharesToSell.value)

          // disable the forms while the value gets updated on-chain
          setFormsDisabled(true)

          // TODO: Does not work :(
          try {
            const gas = new BN('30000000000000');

            // make an update call to the smart contract
            await window.contract.sell_shares({
              // pass the number of shares that the user entered in the form
              num_shares: numSharesToSell
            }, gas, "0")

            setContractCallRefreshFlag(!contractCallRefreshFlag)
          } catch (e) {
            alert(
              'Something went wrong! ' + e
            )
            throw e
          } finally {
            // re-enable the forms, whether the call succeeded or failed
            setFormsDisabled(false)
          }
        }}>
          <fieldset id="fieldsetForSell">
            <label
              htmlFor="sharesToSell"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Number of shares to sell
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                id="sharesToSell"
                style={{ flex: 1 }}
              />
              <button
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Sell
              </button>
            </div>
          </fieldset>
        </form>


        <table>
          <tbody>
            <tr>
              <td>Shares owned by {window.accountId}</td>
              <td>{sharesOwned}</td>
            </tr>
            <tr>
              <td>Shares available to buy</td>
              <td>{sharesOutstanding}</td>
            </tr>
            <tr>
              <td>Price per share</td>
              <td>{pricePerShare}</td>
            </tr>
          </tbody>
        </table>
      </main>
    </>
  )

  function disableForms() {
    console.log("test")
  }
}
