use juno_std_derive::CosmwasmExt;
/// GenesisState defines the module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.GenesisState")]
pub struct GenesisState {
	/// params are the drip module parameters
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// Params defines the drip module params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.Params")]
pub struct Params {
	/// enable_drip defines a parameter to enable the drip module
	#[prost(bool, tag = "1")]
	pub enable_drip: bool,
	/// allowed_addresses defines the list of addresses authorized to use the module
	#[prost(string, repeated, tag = "3")]
	pub allowed_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.QueryParamsRequest")]
#[proto_query(path = "/juno.drip.v1.Query/Params", response_type = QueryParamsResponse)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	/// params is the returned parameter from the module
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// MsgDistributeTokens defines a message that registers a Distribution of tokens.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.MsgDistributeTokens")]
pub struct MsgDistributeTokens {
	/// sender_address is the bech32 address of message sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// amount is the amount being airdropped to stakers
	#[prost(message, repeated, tag = "2")]
	pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDistributeTokensResponse defines the MsgDistributeTokens response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.MsgDistributeTokensResponse")]
pub struct MsgDistributeTokensResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
	/// authority is the address that controls the module (defaults to x/gov unless overwritten).
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// params defines the x/auth parameters to update.
	///
	/// NOTE: All parameters must be supplied.
	#[prost(message, optional, tag = "2")]
	pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	::prost::Message,
	::serde::Serialize,
	::serde::Deserialize,
	::schemars::JsonSchema,
	CosmwasmExt,
)]
#[proto_message(type_url = "/juno.drip.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct DripQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> DripQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
}
