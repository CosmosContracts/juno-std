use juno_std_derive::CosmwasmExt;
/// GenesisState - initial state of module
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
#[proto_message(type_url = "/juno.cwhooks.v1.GenesisState")]
pub struct GenesisState {
	/// Params of this module
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
	/// staking_contract_addresses
	#[prost(string, repeated, tag = "2")]
	pub staking_contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
	/// gov_contract_addresses
	#[prost(string, repeated, tag = "3")]
	pub gov_contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of module parameters.
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
#[proto_message(type_url = "/juno.cwhooks.v1.Params")]
pub struct Params {
	/// contract_gas_limit is the contract call gas limit
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub contract_gas_limit: u64,
}
/// QueryParams is the request type to get all module params.
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryParamsRequest")]
#[proto_query(
    path = "/juno.cwhooks.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryClockContractsResponse is the response type for the Query/ClockContracts RPC method.
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// QueryStakingContractsRequest
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryStakingContractsRequest")]
#[proto_query(
    path = "/juno.cwhooks.v1.Query/StakingContracts",
    response_type = QueryStakingContractsResponse
)]
pub struct QueryStakingContractsRequest {}
/// QueryStakingContractsResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryStakingContractsResponse")]
pub struct QueryStakingContractsResponse {
	#[prost(string, repeated, tag = "1")]
	pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryStakingContractsRequest
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryGovernanceContractsRequest")]
#[proto_query(
    path = "/juno.cwhooks.v1.Query/GovernanceContracts",
    response_type = QueryGovernanceContractsResponse
)]
pub struct QueryGovernanceContractsRequest {}
/// QueryGovernanceContractsResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.QueryGovernanceContractsResponse")]
pub struct QueryGovernanceContractsResponse {
	#[prost(string, repeated, tag = "1")]
	pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
	/// authority is the address of the governance account.
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// params defines the x/clock parameters to update.
	///
	/// NOTE: All parameters must be supplied.
	#[prost(message, optional, tag = "2")]
	pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgRegisterStaking
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgRegisterStaking")]
pub struct MsgRegisterStaking {
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub register_address: ::prost::alloc::string::String,
}
/// MsgRegisterStakingResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgRegisterStakingResponse")]
pub struct MsgRegisterStakingResponse {}
/// MsgRegisterGovernance
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgRegisterGovernance")]
pub struct MsgRegisterGovernance {
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub register_address: ::prost::alloc::string::String,
}
/// MsgRegisterGovernanceResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgRegisterGovernanceResponse")]
pub struct MsgRegisterGovernanceResponse {}
/// MsgUnregisterGovernance
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUnregisterGovernance")]
pub struct MsgUnregisterGovernance {
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub register_address: ::prost::alloc::string::String,
}
/// MsgUnregisterGovernanceResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUnregisterGovernanceResponse")]
pub struct MsgUnregisterGovernanceResponse {}
/// MsgUnregisterStaking
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUnregisterStaking")]
pub struct MsgUnregisterStaking {
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub register_address: ::prost::alloc::string::String,
}
/// MsgUnregisterStakingResponse
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
#[proto_message(type_url = "/juno.cwhooks.v1.MsgUnregisterStakingResponse")]
pub struct MsgUnregisterStakingResponse {}
pub struct CwhooksQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CwhooksQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
	pub fn staking_contracts(
		&self,
	) -> Result<QueryStakingContractsResponse, cosmwasm_std::StdError> {
		QueryStakingContractsRequest {}.query(self.querier)
	}
	pub fn governance_contracts(
		&self,
	) -> Result<QueryGovernanceContractsResponse, cosmwasm_std::StdError> {
		QueryGovernanceContractsRequest {}.query(self.querier)
	}
}
