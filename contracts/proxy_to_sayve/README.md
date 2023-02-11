# Generator Proxy to SAYVE LP Staking Rewards

The generator proxy contract interacts with the SAYVE LP staking contract (the Astroport dual rewards feature).

SAYVE LP Staking contract implementation: https://github.com/sayveprotocol/sayve-terra2/tree/main/contracts/lp_staking

The staking via proxy guide is [here](https://miro.medium.com/max/1400/0*8hn2NSnZJZTa9YGV).

---
## Code ID
Testnet: 6673
```
https://finder.terra.money/testnet/tx/4eb34fc3b4a4a809ac3dc60de7a77a07af62ac7a2ee8c4dda9d7898968f80e1d
```
Mainnet: 959
```
https://finder.terra.money/mainnet/tx/1147eba0f054ddb406b796d4867ceb0f847acd8845205802e09ecd09cb659db4
```


## InstantiateMsg

Inits with required contract addresses for depositing and reward distribution.

```json
{
  "generator_contract_addr": "terra...",
  "pair_addr": "terra...",
  "lp_token_addr": "terra...",
  "reward_contract_addr": "terra...",
  "reward_token_addr": "terra..."
}
```

## ExecuteMsg

### `receive`

CW20 receive msg.

```json
{
  "receive": {
    "sender": "terra...",
    "amount": "123",
    "msg": "<base64_encoded_json_string>"
  }
}
```

### `update_rewards`

Updates token proxy rewards.

```json
{
  "update_rewards": {}
}
```

### `send_rewards`

Sends token rewards amount for given address.

```json
{
  "send_rewards": {
    "account": "terra...",
    "amount": "123"
  }
}
```

### `withdraw`

Withdraws token rewards amount for given address.

```json
{
  "withdraw": {
    "account": "terra...",
    "amount": "123"
  }
}
```

### `emergency_withdraw`

Withdraws token rewards amount for given address.

```json
{
  "emergency_withdraw": {
    "account": "terra...",
    "amount": "123"
  }
}
```

## QueryMsg

All query messages are described below. A custom struct is defined for each query response.

### `deposit`

Returns deposited/staked token amount.

```json
{
  "deposit": {}
}
```

### `reward`

Gives token proxy reward amount.

```json
{
  "reward": {}
}
```

### `pending_token`

Gives token proxy reward pending amount.

```json
{
  "pending_token": {}
}
```
