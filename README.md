# Rosetta SDK

## Repository structure

- `rosetta-types`: initially autogenerated using the openapi-generator contains the request and
response structs used by the client and server.
- `rosetta-crypto`: cryptographic primitives used by the `rosetta-client`.
- `rosetta-client`: client to interact with a rosetta server.
- `rosetta-cli`: command line interface built with the `rosetta-client`.
- `rosetta-wallet`: command line interface built with the `rosetta-client`.

## Getting started

### Install cli tools
```
cargo install --path rosetta-cli
cargo install --path rosetta-wallet
```

### Start local testnets and block explorer
```
docker compose up
```

### Block Explorer
Open in your web browser [http://127.0.0.1:3000](http://127.0.0.1:3000)

### Bitcoin
```
bitcoin-cli -regtest -rpcuser=rosetta -rpcpassword=rosetta generatetoaddress 101 YOUR_ADDRESS
```

### Ethereum
```
geth attach http://127.0.0.1:8545
> eth.sendTransaction({from: eth.coinbase, to: "0xYOUR_ADDRESS", value: web3.toWei(50, "ether")})
```

### Bitcoin example
```
docker compose up
rosetta-wallet --chain btc --keyfile /tmp/alice faucet 1000
rosetta-wallet --chain btc --keyfile /tmp/bob account
rosetta-wallet --chain btc --keyfile /tmp/alice transfer ACCOUNT 1000
rosetta-wallet --chain btc --keyfile /tmp/alice faucet 1
rosetta-wallet --chain btc --keyfile /tmp/bob balance
```

### Ethereum example
```
docker compose up
rosetta-wallet --chain eth --keyfile /tmp/alice faucet 1000
rosetta-wallet --chain eth --keyfile /tmp/bob account
rosetta-wallet --chain eth --keyfile /tmp/alice transfer ACCOUNT 1000
rosetta-wallet --chain eth --keyfile /tmp/bob balance
```

### Substrate example
```
docker compose up
rosetta-wallet --chain dot --keyfile /tmp/alice faucet 3000000000000000
rosetta-wallet --chain dot --keyfile /tmp/bob account
rosetta-wallet --chain dot --keyfile /tmp/bob balance
rosetta-wallet --chain dot --keyfile /tmp/alice transfer 15VdDXFKybRiY4Mbz3H4pucM2nzicmEcHvwzhuBXSTdaR7ay 1500000000000000
rosetta-wallet --chain dot --keyfile /tmp/bob balance
```