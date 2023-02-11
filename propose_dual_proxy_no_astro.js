// setup
const { LCDClient, MnemonicKey, MsgExecuteContract } = require('@terra-money/terra.js');
 
async function main() {
 
const terra = new LCDClient({
  URL: 'https://pisco-lcd.terra.dev',
  chainID: 'pisco-1',
});
 
const mk = new MnemonicKey({
  mnemonic: ''
});
 
const wallet = terra.wallet(mk);
console.log(wallet.key.accAddress) 
// contract addresses (testnet)
const assembly_address = 'terra195m6n5xq4rkjy47fn5y3s08tfmj3ryknj55jqvgq2y55zul9myzsgy06hk';
const xastro_address = 'terra1ctzthkc0nzseppqtqlwq9mjwy9gq8ht2534rtcj3yplerm06snmqfc5ucr';
const generator_address = 'terra1gc4d4v82vjgkz0ag28lrmlxx3tf6sq69tmaujjpe7jwmnqakkx0qm28j2l';
const ex_proxy_addr = 'terra1648m2znznvcdadem5dherx4z8zle8yuyxz37khtdkv7hra93s02snfnylk';
const ex_lp_addr = 'terra18pvm8tmgyutmlykhtwt48lqgvak4afkl4uuu7dryh8xt9g4lfn4sc0u3xt';
 
// custom functions 
async function queryContract(terra, contractAddress, query) {
   return await terra.wasm.contractQuery(contractAddress, query);
}
 
function toEncodedBinary(object) {
  return Buffer.from(JSON.stringify(object)).toString('base64');
}
 
// update generator config
// let config = await queryContract(terra, generator_address, {config: {}});
 
// config.allowed_reward_proxies.push(ex_proxy_addr);
 
// first msg
 
 
let msg = {
   "move_to_proxy": {
       "lp_token": ex_lp_addr,
       "proxy": ex_proxy_addr,
   }
}
 
let binary = toEncodedBinary(msg)
 
let move_to_proxy_msg = {
   order: "1",
   msg: {
       wasm: {
           execute: {
               contract_addr: generator_address,
               msg: binary,
               funds: []
           }
       }
   }
}
 
// proposal msg
let proposal_msg = {
   "submit_proposal": {
       "title": "Add Generator Proxy to Sayve",
       "description": "Sayve would like to integrate Proxy Contracts we would like a Generator Proxy to be added for Sayve Luna LP so testing can stake LP via Astroport Generator to Sayve LP staking contract",
       "link": null,
       "messages": [move_to_proxy_msg]
   }
}
 
 
let proposal_binary = toEncodedBinary(proposal_msg)
 
let execute_msg = {
   "send": {
       "contract": assembly_address,
       "amount": "15000000000",
       "msg": proposal_binary
   }
}
 
// execute and broadcast proposal
let execute = new MsgExecuteContract(
   wallet.key.accAddress,
   xastro_address,
   execute_msg,
);

let executeTx = await wallet.createAndSignTx({
    msgs: [execute]
})
.then(tx => terra.tx.broadcast(tx))
.then((result) => console.log(result.txhash));
 
}
 
main().catch(console.error)
