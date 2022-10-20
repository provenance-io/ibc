# Exchange

This is a CosmWasm smart contract that allows users to exchange collateral denom for a native denom,
and vice versa at a 1:1 exchange ratio. The smart contract uses the supplied marker address, and 
manipulates the marker byminting, burning, and withdrawing the marker's currencies.

When collateral denom is exchanged for native denom, the contract will send the collateral to the marker,
mint new native denom, and then withdraw it to the sender's account. When the user exchanges native denom,
the contract sends the native denom to the marker, burns the received native denom, and withdraws the collateral
to the sender.

## Init

In order to create the contract you must pass in the InstantiateMsg with the following args. The marker specified
by the `marker_address` must have an equal amount of `native_denom` and `collateral_denom` in its account.

```rust
pub struct InstantiateMsg {
    pub native_denom: String,
    pub collateral_denom: String,
    pub marker_address: String,
}
```

The `native_denom` is the local currency of the private chain. It will be minted and burned as needed. The `collateral_denom`
is the currency that is traded and stored to receive `native_denom`. It is never minted or burned. The exchange ratio is `1:1`.

## Messages

The following messages can be used to interact with the contract.

`Trade {}` - Trades one coin for another using the `--amount` flag. Trading `native_denom` will result in the sender receiving 
stored `collateral_denom`, and trading `collateral_denom` will result in the sender receiving newly minted `native_denom`.

The following queries can be used to inspect the contract.

`GetExchangeInfo {}` - Returns the native_denom, collateral_denom, and marker_address.