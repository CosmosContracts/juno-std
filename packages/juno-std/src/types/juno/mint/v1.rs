use juno_std_derive::CosmwasmExt;
/// Minter represents the minting state.
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
#[proto_message(type_url = "/juno.mint.v1.Minter")]
pub struct Minter {
	/// current annual inflation rate
	#[prost(string, tag = "1")]
	pub inflation: ::prost::alloc::string::String,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub phase: u64,
	#[prost(uint64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub start_phase_block: u64,
	/// current annual expected provisions
	#[prost(string, tag = "4")]
	pub annual_provisions: ::prost::alloc::string::String,
	#[prost(string, tag = "5")]
	pub target_supply: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
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
#[proto_message(type_url = "/juno.mint.v1.Params")]
pub struct Params {
	/// type of coin to mint
	#[prost(string, tag = "1")]
	pub mint_denom: ::prost::alloc::string::String,
	/// expected blocks per year
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub blocks_per_year: u64,
}
/// GenesisState defines the mint module's genesis state.
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
#[proto_message(type_url = "/juno.mint.v1.GenesisState")]
pub struct GenesisState {
	/// minter is a space for holding current inflation information.
	#[prost(message, optional, tag = "1")]
	pub minter: ::core::option::Option<Minter>,
	/// params defines all the parameters of the module.
	#[prost(message, optional, tag = "2")]
	pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/juno.mint.v1.QueryParamsRequest")]
#[proto_query(path = "/juno.mint.v1.Query/Params", response_type = QueryParamsResponse)]
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
#[proto_message(type_url = "/juno.mint.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	/// params defines the parameters of the module.
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// QueryInflationRequest is the request type for the Query/Inflation RPC method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryInflationRequest")]
#[proto_query(
    path = "/juno.mint.v1.Query/Inflation",
    response_type = QueryInflationResponse
)]
pub struct QueryInflationRequest {}
/// QueryInflationResponse is the response type for the Query/Inflation RPC
/// method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryInflationResponse")]
pub struct QueryInflationResponse {
	/// inflation is the current minting inflation value.
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub inflation: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAnnualProvisionsRequest is the request type for the
/// Query/AnnualProvisions RPC method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryAnnualProvisionsRequest")]
#[proto_query(
    path = "/juno.mint.v1.Query/AnnualProvisions",
    response_type = QueryAnnualProvisionsResponse
)]
pub struct QueryAnnualProvisionsRequest {}
/// QueryAnnualProvisionsResponse is the response type for the
/// Query/AnnualProvisions RPC method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryAnnualProvisionsResponse")]
pub struct QueryAnnualProvisionsResponse {
	/// annual_provisions is the current minting annual provisions value.
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub annual_provisions: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTargetSupplyRequest is the request type for the
/// Query/TargetSupply RPC method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryTargetSupplyRequest")]
#[proto_query(
    path = "/juno.mint.v1.Query/TargetSupply",
    response_type = QueryTargetSupplyResponse
)]
pub struct QueryTargetSupplyRequest {}
/// QueryTargetSupplyResponse is the response type for the
/// Query/TargetSupply RPC method.
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
#[proto_message(type_url = "/juno.mint.v1.QueryTargetSupplyResponse")]
pub struct QueryTargetSupplyResponse {
	/// target_supply is the target supply for this phase value.
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub target_supply: ::prost::alloc::vec::Vec<u8>,
}
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
#[proto_message(type_url = "/juno.mint.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
	/// authority is the address of the governance account.
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// params defines the x/mint parameters to update.
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
#[proto_message(type_url = "/juno.mint.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct MintQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MintQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
	pub fn inflation(&self) -> Result<QueryInflationResponse, cosmwasm_std::StdError> {
		QueryInflationRequest {}.query(self.querier)
	}
	pub fn annual_provisions(
		&self,
	) -> Result<QueryAnnualProvisionsResponse, cosmwasm_std::StdError> {
		QueryAnnualProvisionsRequest {}.query(self.querier)
	}
	pub fn target_supply(&self) -> Result<QueryTargetSupplyResponse, cosmwasm_std::StdError> {
		QueryTargetSupplyRequest {}.query(self.querier)
	}
}
