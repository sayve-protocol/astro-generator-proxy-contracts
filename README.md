# Astroport Generator Proxy Contracts

[![codecov](https://codecov.io/gh/astroport-fi/astro-generator-proxy-contracts/branch/main/graph/badge.svg?token=ZCO1D3AGSM)](https://codecov.io/gh/astroport-fi/astro-generator-proxy-contracts)

This repo contains the proxy contracts for 3rd party LP staking incentives.
These are needed for allowing dual incentives on the Astro LP Tokens via the generator contract.

## Contracts

| Name                           | Description                      |
| ------------------------------ | -------------------------------- |
| [`proxy_to_sayve`](contracts/proxy_to_sayve) | Generator Proxy to Sayve Protocol |

## Building Contracts

You will need Rust 1.64.0+ with wasm32-unknown-unknown target installed.

### You can compile each contract:
Go to contract directory and run 
    
```
cargo wasm
cp ../../target/wasm32-unknown-unknown/release/astroport_token.wasm .
ls -l astroport_token.wasm
sha256sum astroport_token.wasm
```

### You can run tests for all contracts
Run the following from the repository root

```
cargo test
```

### For a production-ready (compressed) build:
Run the following from the repository root

```
./scripts/build_release.sh
```

The optimized contracts are generated in the artifacts/ directory.

## Branches

We use [main](https://github.com/astroport-fi/astro-generator-proxy-contracts/tree/main) branch for new feature development and [release](https://github.com/astroport-fi/astro-generator-proxy-contracts/tree/release) one for collecting features which are ready for deployment. You can find the version and commit for actually deployed contracts [here](https://github.com/astroport-fi/astroport-changelog).

## Docs

Docs can be generated using `cargo doc --no-deps`

## Contracts
Testnet: Code ID 7141
Mainnet: Code ID 1028

### Init message
#### Testnet
```

{
  "generator_contract_addr": "terra1gc4d4v82vjgkz0ag28lrmlxx3tf6sq69tmaujjpe7jwmnqakkx0qm28j2l",
  "lp_token_addr": "terra18pvm8tmgyutmlykhtwt48lqgvak4afkl4uuu7dryh8xt9g4lfn4sc0u3xt",
  "pair_addr": "terra1d9058pntz4rzut00hcara77hc9hrm3p8c0c88sgelnz8t5w5q6ds0jt2ju",
  "reward_contract_addr": "terra1gv5852s3nme2q7tj6qhmkve8hsku5d58pvxt5yt3m2zpjg3l29gqtzcerj",
  "reward_token_addr": "terra16q6kqp0p2hlfgj52sadv5p7qh2hgkkjdfske48vklz94tkxg4jwq8k8myj"
}
```

The generator contract address is from "generator_address"
```
https://github.com/astroport-fi/astroport-changelog/blob/main/terra-2/pisco-1/core_pisco.json
```
The reward_contract_addr is your LP staking contract <br>
The reward_token_addr is the token you will be giving as reward <br>
lp_token and pair_addr are created when you create pool on astroport <br>
For example looking at VKR
```
https://terrasco.pe/mainnet/tx/C69F9A70DE209EFFC473ACD40522480E02F8F0706076B33CD7C58ECA23CA45B7
``` 
We can see the lp_token_addr and pair_addr
```
liquidity_token_addr    terra18mcmlf4v23ehukkh7qxgpf5tznzg6893fxmf9ffmdt9phgf365zqvmlug6
pair_contract_addr  terra1alzkrc6hkvs8g5a064cukfxnv0jj4l3l8vhgfypfxvysk78v6dgqsymgmv
```
ex_proxy_addr from initiation
```
terra1648m2znznvcdadem5dherx4z8zle8yuyxz37khtdkv7hra93s02snfnylk
```


## Astro Proposal
### Install dependencies
npm init -y
npm install @terra-money/terra.js



### Governance
Messages to add a proxy to your own staking contract (optional):
1. Build your proxy contract cloning and instantiate it.
2. Update paramaters in propose_dual_proxy_no_astro.js 
```
ex_proxy_addr: "terra1648m2znznvcdadem5dherx4z8zle8yuyxz37khtdkv7hra93s02snfnylk"
```
and
assembly_address = "astral_assembly_address" <br>
xastro_address = "xastro_token_address" <br>
generator_address = "generator_address" <br>
from 
```
https://github.com/astroport-fi/astroport-changelog/blob/main/terra-2/pisco-1/core_pisco.json
```
3. Make a message to move lp tokens to the proxy via the generator's move_to_proxy endpoint.


### Submit Proposal
```
node propose_dual_proxy_no_astro.js
```
