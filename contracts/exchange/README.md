# Exchange

This is a CosmWasm smart contract that acts as an exchange between a denomination
and a marker denom on the chain. The native and private (restricted) denominations
along with their exchange rate can be configured at initialization.

The contract only allows a `denom <-> restricted marker` transfer. We plan to upgrade this and allow
`any <-> restricted marker` transfer.

## Init

In order to create the contract you must pass in the InstantiateMsg with the following args.

```rust
pub struct InstantiateMsg {
    pub native_denom: String,
    pub private_denom: String,
    pub exchange_rate: Decimal,
    pub marker_address: String,
}
```

The native_denom refers to the denomination that is being exchanged for the private_denom. The private_denom represents the restricted
marker denom on the chain, and the marker_address is the address for this marker. Lastly, the exchange_rate is the amount of native_denom
to private_denom. A 1:1 ratio is an exchange_rate of "1.0".

## Messages

The following messages can be used to interact with the contract.

`Trade {coin: Coin}` - Takes in a coin and exchanges it for either the opposite one. The --amount flag MUST be included if the native_denom
is the coin being sent in.
`SetExchangeRate {exchange_rate: Decimal}` - Changes the exchange rate on the contract. It can only be set by the owner.
`SetOwner {owner: String}` - Changes the owner of the contract. It can only be set by the owner.


The following queries can be used to inspect the contract.

`GetExchangeInfo {}` - Returns the native_denom, private_denom, exchange_rate, and marker_address.
`GetOwner {}` - Returns the current owner of the contract.