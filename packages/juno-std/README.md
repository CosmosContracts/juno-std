# juno-std

[![juno-std on crates.io](https://img.shields.io/crates/v/juno-std.svg)](https://crates.io/crates/juno-std) [![Docs](https://docs.rs/juno-std/badge.svg)](https://docs.rs/juno-std)

Proto-generated types and helpers for interacting with Juno. Compatible with CosmWasm.
You can find all types and queriers generated from Juno's Protobuffers in their respective module in `juno_std`.

## Executing messages from CosmWasm

```rust
use cosmwasm_std::{CosmosMsg, Response, Env};
use juno_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let sender = env.contract.address.into();

    // construct message and convert them into cosmos message
    // (notice `CosmosMsg` type and `.into()`)
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

    Ok(Response::new()
        .add_message(msg_create_denom)
        .add_attribute("method", "try_create_denom"))
}
```
