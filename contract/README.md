near-smart-stock-contract Smart Contract
==================

A [smart contract] written in [Rust] to simulate buying and selling shares of an imaginary company. Disclaimer: Learning exercise, not meant for real trading.


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]


Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
2. Tests: You can run smart contract unit tests with the using the command below. It runs standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.
   ```
   cargo test -- --nocapture
   ```

  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
