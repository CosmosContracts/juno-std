use juno_std_derive::CosmwasmExt;
/// This defines the address, balance, and wallet limit
/// of a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.FeePayContract")]
pub struct FeePayContract {
	/// The address of the contract.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// The ledger balance of the contract.
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub balance: u64,
	/// The number of times a wallet may interact with the contract.
	#[prost(uint64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub wallet_limit: u64,
}
/// This object is used to store the number of times a wallet has
/// interacted with a contract.
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
#[proto_message(type_url = "/juno.feepay.v1.FeePayWalletUsage")]
pub struct FeePayWalletUsage {
	/// The contract address.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// The wallet address.
	#[prost(string, tag = "2")]
	pub wallet_address: ::prost::alloc::string::String,
	/// The number of uses corresponding to a wallet.
	#[prost(uint64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub uses: u64,
}
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
#[proto_message(type_url = "/juno.feepay.v1.GenesisState")]
pub struct GenesisState {
	/// params are the feepay module parameters
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
	/// fee_pay_contracts are the feepay module contracts
	#[prost(message, repeated, tag = "2")]
	pub fee_pay_contracts: ::prost::alloc::vec::Vec<FeePayContract>,
}
/// Params defines the feepay module params
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
#[proto_message(type_url = "/juno.feepay.v1.Params")]
pub struct Params {
	/// enable_feepay defines a parameter to enable the feepay module
	#[prost(bool, tag = "1")]
	pub enable_feepay: bool,
}
/// QueryFeePayContractRequest retrieves a single fee pay contract
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractRequest")]
#[proto_query(
    path = "/juno.feepay.v1.Query/FeePayContract",
    response_type = QueryFeePayContractResponse
)]
pub struct QueryFeePayContractRequest {
	/// contract_address defines the address of the fee pay contract
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
}
/// QueryFeePayContractResponse defines the response for retrieving a single fee pay contract
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractResponse")]
pub struct QueryFeePayContractResponse {
	/// contract defines the fee pay contract
	#[prost(message, optional, tag = "1")]
	pub fee_pay_contract: ::core::option::Option<FeePayContract>,
}
/// Message for querying a list of fee pay contracts
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractsRequest")]
#[proto_query(
    path = "/juno.feepay.v1.Query/FeePayContracts",
    response_type = QueryFeePayContractsResponse
)]
pub struct QueryFeePayContractsRequest {
	/// Pagination defines an optional pagination for the request.
	#[prost(message, optional, tag = "1")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// The response for querying all fee pay contracts
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractsResponse")]
pub struct QueryFeePayContractsResponse {
	/// A slice of all the stored fee pay contracts
	#[prost(message, repeated, tag = "1")]
	pub fee_pay_contracts: ::prost::alloc::vec::Vec<FeePayContract>,
	/// pagination defines the pagination in the response.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Message for querying the number of uses on a fee pay contract by wallet
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractUsesRequest")]
#[proto_query(
    path = "/juno.feepay.v1.Query/FeePayContractUses",
    response_type = QueryFeePayContractUsesResponse
)]
pub struct QueryFeePayContractUsesRequest {
	/// The contract address.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// The wallet address.
	#[prost(string, tag = "2")]
	pub wallet_address: ::prost::alloc::string::String,
}
/// The response for querying the number of uses on a fee pay contract by wallet
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayContractUsesResponse")]
pub struct QueryFeePayContractUsesResponse {
	/// The number of uses on the fee pay contract by wallet
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub uses: u64,
}
/// Message for querying if a wallet is eligible for fee pay contract interactions
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayWalletIsEligibleRequest")]
#[proto_query(
    path = "/juno.feepay.v1.Query/FeePayWalletIsEligible",
    response_type = QueryFeePayWalletIsEligibleResponse
)]
pub struct QueryFeePayWalletIsEligibleRequest {
	/// The contract address.
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// The wallet address.
	#[prost(string, tag = "2")]
	pub wallet_address: ::prost::alloc::string::String,
}
/// The response for querying if a wallet is eligible for fee pay contract interactions
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
#[proto_message(type_url = "/juno.feepay.v1.QueryFeePayWalletIsEligibleResponse")]
pub struct QueryFeePayWalletIsEligibleResponse {
	/// The eligibility of the wallet for fee pay contract interactions
	#[prost(bool, tag = "1")]
	pub eligible: bool,
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
#[proto_message(type_url = "/juno.feepay.v1.QueryParamsRequest")]
#[proto_query(
    path = "/juno.feepay.v1.Query/Params",
    response_type = QueryParamsResponse
)]
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
#[proto_message(type_url = "/juno.feepay.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	/// params is the returned Feepay parameter
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// The message to register a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgRegisterFeePayContract")]
pub struct MsgRegisterFeePayContract {
	/// The wallet address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The fee pay contract to register.
	#[prost(message, optional, tag = "2")]
	pub fee_pay_contract: ::core::option::Option<FeePayContract>,
}
/// The response message for registering a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgRegisterFeePayContractResponse")]
pub struct MsgRegisterFeePayContractResponse {}
/// The message to unregister a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUnregisterFeePayContract")]
pub struct MsgUnregisterFeePayContract {
	/// The wallet address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The fee pay contract address.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
}
/// The response message for unregistering a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUnregisterFeePayContractResponse")]
pub struct MsgUnregisterFeePayContractResponse {}
/// The message to fund a fee pay contract
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
#[proto_message(type_url = "/juno.feepay.v1.MsgFundFeePayContract")]
pub struct MsgFundFeePayContract {
	/// The wallet address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The fee pay contract to fund.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
	/// The coins to fund the contract with.
	#[prost(message, repeated, tag = "3")]
	pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// The response message for funding a fee pay contract.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgFundFeePayContractResponse")]
pub struct MsgFundFeePayContractResponse {}
/// The message to update a fee pay contract wallet limit.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUpdateFeePayContractWalletLimit")]
pub struct MsgUpdateFeePayContractWalletLimit {
	/// The wallet address of the sender.
	#[prost(string, tag = "1")]
	pub sender_address: ::prost::alloc::string::String,
	/// The fee pay contract to fund.
	#[prost(string, tag = "2")]
	pub contract_address: ::prost::alloc::string::String,
	/// The new wallet limit.
	#[prost(uint64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub wallet_limit: u64,
}
/// The response message for updating a fee pay contract wallet limit.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUpdateFeePayContractWalletLimitResponse")]
pub struct MsgUpdateFeePayContractWalletLimitResponse {}
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
	/// authority is the address that controls the module (defaults to x/gov unless overwritten).
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// params defines the x/feepay parameters to update.
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
#[proto_message(type_url = "/juno.feepay.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct FeepayQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeepayQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn fee_pay_contract(
		&self,
		contract_address: ::prost::alloc::string::String,
	) -> Result<QueryFeePayContractResponse, cosmwasm_std::StdError> {
		QueryFeePayContractRequest { contract_address }.query(self.querier)
	}
	pub fn fee_pay_contracts(
		&self,
		pagination: ::core::option::Option<
			super::super::super::cosmos::base::query::v1beta1::PageRequest,
		>,
	) -> Result<QueryFeePayContractsResponse, cosmwasm_std::StdError> {
		QueryFeePayContractsRequest { pagination }.query(self.querier)
	}
	pub fn fee_pay_contract_uses(
		&self,
		contract_address: ::prost::alloc::string::String,
		wallet_address: ::prost::alloc::string::String,
	) -> Result<QueryFeePayContractUsesResponse, cosmwasm_std::StdError> {
		QueryFeePayContractUsesRequest {
			contract_address,
			wallet_address,
		}
		.query(self.querier)
	}
	pub fn fee_pay_wallet_is_eligible(
		&self,
		contract_address: ::prost::alloc::string::String,
		wallet_address: ::prost::alloc::string::String,
	) -> Result<QueryFeePayWalletIsEligibleResponse, cosmwasm_std::StdError> {
		QueryFeePayWalletIsEligibleRequest {
			contract_address,
			wallet_address,
		}
		.query(self.querier)
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
}
