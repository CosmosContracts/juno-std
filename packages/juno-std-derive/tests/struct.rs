use cosmwasm_std::CosmosMsg;
use juno_std_derive::CosmwasmExt;

#[derive(Clone, PartialEq, Eq, ::prost::Message, CosmwasmExt)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom")]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}

fn main() {
    assert_eq!(MsgCreateDenom::TYPE_URL, "/osmosis.tokenfactory.v1beta1.MsgCreateDenom");
    let msg = MsgCreateDenom {
        sender: "juno1klmlxuxaexgsedzs89jnwypq769jk90ruutrqv".to_string(),
        subdenom: "uxxx".to_string(),
    };

    let _: CosmosMsg = msg.into();
}

mod shim {
    pub struct Any {
        pub type_url: String,
        pub value: Vec<u8>,
    }
}
