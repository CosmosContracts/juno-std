use juno_std_derive::CosmwasmExt;
/// FeeShare defines an instance that organizes fee distribution conditions for
/// the owner of a given smart contract
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
#[proto_message(type_url = "/juno.feeshare.v1.FeeShare")]
pub struct FeeShare {
	/// contract_address is the bech32 address of a registered contract in string
	/// form
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// deployer_address is the bech32 address of message sender. It must be the
	/// same as the contracts admin address.
	#[prost(string, tag = "2")]
	pub deployer_address: ::prost::alloc::string::String,
	/// withdrawer_address is the bech32 address of account receiving the
	/// transaction fees.
	#[prost(string, tag = "3")]
	pub withdrawer_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/juno.feeshare.v1.GenesisState")]
pub struct GenesisState {
	/// params are the feeshare module parameters
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
	/// FeeShare is a slice of active registered contracts for fee distribution
	#[prost(message, repeated, tag = "2")]
	pub fee_share: ::prost::alloc::vec::Vec<FeeShare>,
}
/// Params defines the feeshare module params
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
#[proto_message(type_url = "/juno.feeshare.v1.Params")]
pub struct Params {
	/// enable_feeshare defines a parameter to enable the feeshare module
	#[prost(bool, tag = "1")]
	pub enable_fee_share: bool,
	/// developer_shares defines the proportion of the transaction fees to be
	/// distributed to the registered contract owner
	#[prost(string, tag = "2")]
	pub developer_shares: ::prost::alloc::string::String,
	/// allowed_denoms defines the list of denoms that are allowed to be paid to
	/// the contract withdraw addresses. If said denom is not in the list, the fees
	/// will ONLY be sent to the community pool.
	/// If this list is empty, all denoms are allowed.
	#[prost(string, repeated, tag = "3")]
	pub allowed_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeeSharesRequest is the request type for the Query/FeeShares RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryFeeSharesRequest")]
#[proto_query(
    path = "/juno.feeshare.v1.Query/FeeShares",
    response_type = QueryFeeSharesResponse
)]
pub struct QueryFeeSharesRequest {
	/// pagination defines an optional pagination for the request.
	#[prost(message, optional, tag = "1")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryFeeSharesResponse is the response type for the Query/FeeShares RPC
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryFeeSharesResponse")]
pub struct QueryFeeSharesResponse {
	/// FeeShare is a slice of all stored Reveneue
	#[prost(message, repeated, tag = "1")]
	pub feeshare: ::prost::alloc::vec::Vec<FeeShare>,
	/// pagination defines the pagination in the response.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryFeeShareRequest is the request type for the Query/FeeShare RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryFeeShareRequest")]
#[proto_query(
    path = "/juno.feeshare.v1.Query/FeeShare",
    response_type = QueryFeeShareResponse
)]
pub struct QueryFeeShareRequest {
	/// contract_address of a registered contract in bech32 format
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
}
/// QueryFeeShareResponse is the response type for the Query/FeeShare RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryFeeShareResponse")]
pub struct QueryFeeShareResponse {
	/// FeeShare is a stored Reveneue for the queried contract
	#[prost(message, optional, tag = "1")]
	pub feeshare: ::core::option::Option<FeeShare>,
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryParamsRequest")]
#[proto_query(
    path = "/juno.feeshare.v1.Query/Params",
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	/// params is the returned FeeShare parameter
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// QueryDeployerFeeSharesRequest is the request type for the
/// Query/DeployerFeeShares RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryDeployerFeeSharesRequest")]
#[proto_query(
    path = "/juno.feeshare.v1.Query/DeployerFeeShares",
    response_type = QueryDeployerFeeSharesResponse
)]
pub struct QueryDeployerFeeSharesRequest {
	/// deployer_address in bech32 format
	#[prost(string, tag = "1")]
	pub deployer_address: ::prost::alloc::string::String,
	/// pagination defines an optional pagination for the request.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDeployerFeeSharesResponse is the response type for the
/// Query/DeployerFeeShares RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryDeployerFeeSharesResponse")]
pub struct QueryDeployerFeeSharesResponse {
	/// contract_addresses is the slice of registered contract addresses for a
	/// deployer
	#[prost(string, repeated, tag = "1")]
	pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
	/// pagination defines the pagination in the response.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawerFeeSharesRequest is the request type for the
/// Query/WithdrawerFeeShares RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryWithdrawerFeeSharesRequest")]
#[proto_query(
    path = "/juno.feeshare.v1.Query/WithdrawerFeeShares",
    response_type = QueryWithdrawerFeeSharesResponse
)]
pub struct QueryWithdrawerFeeSharesRequest {
	/// withdrawer_address in bech32 format
	#[prost(string, tag = "1")]
	pub withdrawer_address: ::prost::alloc::string::String,
	/// pagination defines an optional pagination for the request.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryWithdrawerFeeSharesResponse is the response type for the
/// Query/WithdrawerFeeShares RPC method.
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
#[proto_message(type_url = "/juno.feeshare.v1.QueryWithdrawerFeeSharesResponse")]
pub struct QueryWithdrawerFeeSharesResponse {
	/// contract_addresses is the slice of registered contract addresses for a
	/// withdrawer
	#[prost(string, repeated, tag = "1")]
	pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
	/// pagination defines the pagination in the response.
	#[prost(message, optional, tag = "2")]
	pub pagination:
		::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgRegisterFeeShare defines a message that registers a FeeShare
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgRegisterFeeShare")]
pub struct MsgRegisterFeeShare {
	/// contract_address in bech32 format
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// deployer_address is the bech32 address of message sender. It must be the
	/// same the contract's admin address
	#[prost(string, tag = "2")]
	pub deployer_address: ::prost::alloc::string::String,
	/// withdrawer_address is the bech32 address of account receiving the
	/// transaction fees
	#[prost(string, tag = "3")]
	pub withdrawer_address: ::prost::alloc::string::String,
}
/// MsgRegisterFeeShareResponse defines the MsgRegisterFeeShare response type
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgRegisterFeeShareResponse")]
pub struct MsgRegisterFeeShareResponse {}
/// MsgUpdateFeeShare defines a message that updates the withdrawer address for a
/// registered FeeShare
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgUpdateFeeShare")]
pub struct MsgUpdateFeeShare {
	/// contract_address in bech32 format
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// deployer_address is the bech32 address of message sender. It must be the
	/// same the contract's admin address
	#[prost(string, tag = "2")]
	pub deployer_address: ::prost::alloc::string::String,
	/// withdrawer_address is the bech32 address of account receiving the
	/// transaction fees
	#[prost(string, tag = "3")]
	pub withdrawer_address: ::prost::alloc::string::String,
}
/// MsgUpdateFeeShareResponse defines the MsgUpdateFeeShare response type
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgUpdateFeeShareResponse")]
pub struct MsgUpdateFeeShareResponse {}
/// MsgCancelFeeShare defines a message that cancels a registered FeeShare
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgCancelFeeShare")]
pub struct MsgCancelFeeShare {
	/// contract_address in bech32 format
	#[prost(string, tag = "1")]
	pub contract_address: ::prost::alloc::string::String,
	/// deployer_address is the bech32 address of message sender. It must be the
	/// same the contract's admin address
	#[prost(string, tag = "2")]
	pub deployer_address: ::prost::alloc::string::String,
}
/// MsgCancelFeeShareResponse defines the MsgCancelFeeShare response type
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgCancelFeeShareResponse")]
pub struct MsgCancelFeeShareResponse {}
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
	/// authority is the address that controls the module (defaults to x/gov unless overwritten).
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// params defines the x/feeshare parameters to update.
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
#[proto_message(type_url = "/juno.feeshare.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct FeeshareQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeeshareQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn fee_shares(
		&self,
		pagination: ::core::option::Option<
			super::super::super::cosmos::base::query::v1beta1::PageRequest,
		>,
	) -> Result<QueryFeeSharesResponse, cosmwasm_std::StdError> {
		QueryFeeSharesRequest { pagination }.query(self.querier)
	}
	pub fn fee_share(
		&self,
		contract_address: ::prost::alloc::string::String,
	) -> Result<QueryFeeShareResponse, cosmwasm_std::StdError> {
		QueryFeeShareRequest { contract_address }.query(self.querier)
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
	pub fn deployer_fee_shares(
		&self,
		deployer_address: ::prost::alloc::string::String,
		pagination: ::core::option::Option<
			super::super::super::cosmos::base::query::v1beta1::PageRequest,
		>,
	) -> Result<QueryDeployerFeeSharesResponse, cosmwasm_std::StdError> {
		QueryDeployerFeeSharesRequest {
			deployer_address,
			pagination,
		}
		.query(self.querier)
	}
	pub fn withdrawer_fee_shares(
		&self,
		withdrawer_address: ::prost::alloc::string::String,
		pagination: ::core::option::Option<
			super::super::super::cosmos::base::query::v1beta1::PageRequest,
		>,
	) -> Result<QueryWithdrawerFeeSharesResponse, cosmwasm_std::StdError> {
		QueryWithdrawerFeeSharesRequest {
			withdrawer_address,
			pagination,
		}
		.query(self.querier)
	}
}
