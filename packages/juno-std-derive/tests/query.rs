use cosmwasm_std::{ Empty, QueryRequest };
use juno_std_derive::CosmwasmExt;

#[derive(Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/DenomsFromCreator",
    response_type = QueryDenomsFromCreatorResponse
)]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse")]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

fn main() {
    let _: QueryRequest<Empty> = (QueryDenomsFromCreatorRequest {
        creator: "juno1klmlxuxaexgsedzs89jnwypq769jk90ruutrqv".to_string(),
    }).into();
}

mod shim {
    pub struct Any {
        pub type_url: String,
        pub value: Vec<u8>,
    }
}
