use juno_std_derive::CosmwasmExt;
/// This object is used to store the contract address and the
/// jail status of the contract.
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
#[proto_message(type_url = "/juno.clock.v1.ClockContract")]
pub struct ClockContract {
	/// The address of the contract.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// The jail status of the contract.
	#[prost(bool, tag = "2")]
	pub is_jailed: bool,
}
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
#[proto_message(type_url = "/juno.clock.v1.GenesisState")]
pub struct GenesisState {
	/// Params of this module
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/juno.clock.v1.Params")]
pub struct Params {
	/// contract_gas_limit defines the maximum amount of gas that can be used by a contract.
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub contract_gas_limit: u64,
}
/// QueryClockContracts is the request type to get all contracts.
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
#[proto_message(type_url = "/juno.clock.v1.QueryClockContractsRequest")]
#[proto_query(
    path = "/juno.clock.v1.Query/ClockContracts",
    response_type = QueryClockContractsResponse
)]
pub struct QueryClockContractsRequest {
	/// pagination defines an optional pagination for the request.
	#[prost(message, optional, tag = "1")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/juno.clock.v1.QueryClockContractsResponse")]
pub struct QueryClockContractsResponse {
	/// clock_contracts are the clock contracts.
	#[prost(message, repeated, tag = "1")]
	pub clock_contracts: ::prost::alloc::vec::Vec<ClockContract>,
	/// pagination defines the pagination in the response.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClockContract is the request type to get a single contract.
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
#[proto_message(type_url = "/juno.clock.v1.QueryClockContractRequest")]
#[proto_query(
    path = "/juno.clock.v1.Query/ClockContract",
    response_type = QueryClockContractResponse
)]
pub struct QueryClockContractRequest {
	/// contract_address is the address of the contract to query.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
}
/// QueryClockContractResponse is the response type for the Query/ClockContract RPC method.
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
#[proto_message(type_url = "/juno.clock.v1.QueryClockContractResponse")]
pub struct QueryClockContractResponse {
	/// contract is the clock contract.
	#[prost(message, optional, tag = "1")]
	pub clock_contract: ::core::option::Option<ClockContract>,
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
#[proto_message(type_url = "/juno.clock.v1.QueryParamsRequest")]
#[proto_query(path = "/juno.clock.v1.Query/Params", response_type = QueryParamsResponse)]
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
#[proto_message(type_url = "/juno.clock.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// MsgRegisterClockContract is the Msg/RegisterClockContract request type.
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
#[proto_message(type_url = "/juno.clock.v1.MsgRegisterClockContract")]
pub struct MsgRegisterClockContract {
	/// The address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The address of the contract to register.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
}
/// MsgRegisterClockContractResponse defines the response structure for executing a
/// MsgRegisterClockContract message.
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
#[proto_message(type_url = "/juno.clock.v1.MsgRegisterClockContractResponse")]
pub struct MsgRegisterClockContractResponse {}
/// MsgUnregisterClockContract is the Msg/UnregisterClockContract request type.
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
#[proto_message(type_url = "/juno.clock.v1.MsgUnregisterClockContract")]
pub struct MsgUnregisterClockContract {
	/// The address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The address of the contract to unregister.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
}
/// MsgUnregisterClockContractResponse defines the response structure for executing a
/// MsgUnregisterClockContract message.
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
#[proto_message(type_url = "/juno.clock.v1.MsgUnregisterClockContractResponse")]
pub struct MsgUnregisterClockContractResponse {}
/// MsgUnjailClockContract is the Msg/UnjailClockContract request type.
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
#[proto_message(type_url = "/juno.clock.v1.MsgUnjailClockContract")]
pub struct MsgUnjailClockContract {
	/// The address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The address of the contract to unjail.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
}
/// MsgUnjailClockContractResponse defines the response structure for executing a
/// MsgUnjailClockContract message.
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
#[proto_message(type_url = "/juno.clock.v1.MsgUnjailClockContractResponse")]
pub struct MsgUnjailClockContractResponse {}
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
#[proto_message(type_url = "/juno.clock.v1.MsgUpdateParams")]
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
#[proto_message(type_url = "/juno.clock.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct ClockQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ClockQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn clock_contracts(
		&self,
		pagination: ::core::option::Option<
			super::super::super::cosmos::base::query::v1beta1::PageRequest,
		>,
	) -> Result<QueryClockContractsResponse, cosmwasm_std::StdError> {
		QueryClockContractsRequest { pagination }.query(self.querier)
	}
	pub fn clock_contract(
		&self,
		contract_address: ::prost::alloc::string::String,
	) -> Result<QueryClockContractResponse, cosmwasm_std::StdError> {
		QueryClockContractRequest { contract_address }.query(self.querier)
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
}
