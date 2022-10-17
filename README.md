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
