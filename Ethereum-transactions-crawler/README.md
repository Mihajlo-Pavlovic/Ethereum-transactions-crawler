Ethereum Transactions Crawler
---
To run frontend you need to have trunk installed (https://trunkrs.dev/#install).
Position in ether-crawler-app\frontend and run:
```
trunk serve
```
If you get error[E0463]: can't find crate for `core`, you should run:
```
rustup target add wasm32-unknown-unknown
```
To run backend position in ether-crawler-app\backend and run:
```
cargo run
```
To access the app you should go to http://localhost:8080/