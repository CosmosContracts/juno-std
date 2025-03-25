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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.GenesisState")]
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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.Params")]
pub struct Params {
	/// Minimum stores the minimum gas price(s) for all TX on the chain.
	/// When multiple coins are defined then they are accepted alternatively.
	/// The list must be sorted by denoms asc. No duplicate denoms or zero amount
	/// values allowed. For more information see
	/// <https://docs.cosmos.network/main/modules/auth#concepts>
	#[prost(message, repeated, tag = "1")]
	pub minimum_gas_prices:
		::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryMinimumGasPricesRequest is the request type for the
/// Query/MinimumGasPrices RPC method.
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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.QueryMinimumGasPricesRequest")]
#[proto_query(
    path = "/gaia.globalfee.v1beta1.Query/MinimumGasPrices",
    response_type = QueryMinimumGasPricesResponse
)]
pub struct QueryMinimumGasPricesRequest {}
/// QueryMinimumGasPricesResponse is the response type for the
/// Query/MinimumGasPrices RPC method.
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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.QueryMinimumGasPricesResponse")]
pub struct QueryMinimumGasPricesResponse {
	#[prost(message, repeated, tag = "1")]
	pub minimum_gas_prices:
		::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.MsgUpdateParams")]
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
#[proto_message(type_url = "/gaia.globalfee.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct GlobalfeeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> GlobalfeeQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn minimum_gas_prices(
		&self,
	) -> Result<QueryMinimumGasPricesResponse, cosmwasm_std::StdError> {
		QueryMinimumGasPricesRequest {}.query(self.querier)
	}
}
