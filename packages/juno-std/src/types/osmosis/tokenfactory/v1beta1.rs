use juno_std_derive::CosmwasmExt;
/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.DenomAuthorityMetadata")]
pub struct DenomAuthorityMetadata {
	/// Can be empty for no admin, or a valid bech32 address
	#[prost(string, tag = "1")]
	pub admin: ::prost::alloc::string::String,
}
/// Params defines the parameters for the tokenfactory module.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.Params")]
pub struct Params {
	#[prost(message, repeated, tag = "1")]
	pub denom_creation_fee:
		::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
	/// if denom_creation_fee is an empty array, then this field is used to add more gas consumption
	/// to the base cost.
	/// <https://github.com/CosmWasm/token-factory/issues/11>
	#[prost(uint64, tag = "2")]
	#[prost(optional)]
	#[serde(
		serialize_with = "crate::serde::option_as_str::serialize",
		deserialize_with = "crate::serde::option_as_str::deserialize"
	)]
	pub denom_creation_gas_consume: ::core::option::Option<u64>,
}
/// GenesisState defines the tokenfactory module's genesis state.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.GenesisState")]
pub struct GenesisState {
	/// params defines the parameters of the module.
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
	#[prost(message, repeated, tag = "2")]
	pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.GenesisDenom")]
pub struct GenesisDenom {
	#[prost(string, tag = "1")]
	pub denom: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/Params",
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
	/// params defines the parameters of the module.
	#[prost(message, optional, tag = "1")]
	pub params: ::core::option::Option<Params>,
}
/// QueryDenomAuthorityMetadataRequest defines the request structure for the
/// DenomAuthorityMetadata gRPC query.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata",
    response_type = QueryDenomAuthorityMetadataResponse
)]
pub struct QueryDenomAuthorityMetadataRequest {
	#[prost(string, tag = "1")]
	pub denom: ::prost::alloc::string::String,
}
/// QueryDenomAuthorityMetadataResponse defines the response structure for the
/// DenomAuthorityMetadata gRPC query.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataResponse")]
pub struct QueryDenomAuthorityMetadataResponse {
	#[prost(message, optional, tag = "1")]
	pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryDenomsFromCreatorRequest defines the request structure for the
/// DenomsFromCreator gRPC query.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/DenomsFromCreator",
    response_type = QueryDenomsFromCreatorResponse
)]
pub struct QueryDenomsFromCreatorRequest {
	#[prost(string, tag = "1")]
	pub creator: ::prost::alloc::string::String,
}
/// QueryDenomsFromCreatorRequest defines the response structure for the
/// DenomsFromCreator gRPC query.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse")]
pub struct QueryDenomsFromCreatorResponse {
	#[prost(string, repeated, tag = "1")]
	pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCreateDenom defines the message structure for the CreateDenom gRPC service
/// method. It allows an account to create a new denom. It requires a sender
/// address and a sub denomination. The (sender_address, sub_denomination) tuple
/// must be unique and cannot be re-used.
///
/// The resulting denom created is defined as
/// <factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is
/// originally set to be the creator, but this can be changed later. The token
/// denom does not indicate the current admin.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom")]
pub struct MsgCreateDenom {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	/// subdenom can be up to 44 "alphanumeric" characters long.
	#[prost(string, tag = "2")]
	pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenomResponse")]
pub struct MsgCreateDenomResponse {
	#[prost(string, tag = "1")]
	pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint
/// more of a token.  For now, we only support minting to the sender account
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgMint")]
pub struct MsgMint {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
	#[prost(string, tag = "3")]
	pub mint_to_address: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the response structure for an executed
/// MsgMint message.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgMintResponse")]
pub struct MsgMintResponse {}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.  For now, we only support burning from the sender account.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurn")]
pub struct MsgBurn {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
	#[prost(string, tag = "3")]
	pub burn_from_address: ::prost::alloc::string::String,
}
/// MsgBurnResponse defines the response structure for an executed
/// MsgBurn message.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurnResponse")]
pub struct MsgBurnResponse {}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdmin")]
pub struct MsgChangeAdmin {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	pub denom: ::prost::alloc::string::String,
	#[prost(string, tag = "3")]
	pub new_admin: ::prost::alloc::string::String,
}
/// MsgChangeAdminResponse defines the response structure for an executed
/// MsgChangeAdmin message.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdminResponse")]
pub struct MsgChangeAdminResponse {}
/// MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set
/// the denom's bank metadata
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata")]
pub struct MsgSetDenomMetadata {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// MsgSetDenomMetadataResponse defines the response structure for an executed
/// MsgSetDenomMetadata message.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadataResponse")]
pub struct MsgSetDenomMetadataResponse {}
/// MsgForceTransfer is the sdk.Msg type for allowing an admin account to
/// forcefully transfer a token from one account to another
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgForceTransfer")]
pub struct MsgForceTransfer {
	#[prost(string, tag = "1")]
	pub sender: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
	#[prost(string, tag = "3")]
	pub transfer_from_address: ::prost::alloc::string::String,
	#[prost(string, tag = "4")]
	pub transfer_to_address: ::prost::alloc::string::String,
}
/// MsgForceTransferResponse defines the response structure for an executed
/// MsgForceTransfer message.
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgForceTransferResponse")]
pub struct MsgForceTransferResponse {}
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgUpdateParams")]
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct TokenfactoryQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TokenfactoryQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
		QueryParamsRequest {}.query(self.querier)
	}
	pub fn denom_authority_metadata(
		&self,
		denom: ::prost::alloc::string::String,
	) -> Result<QueryDenomAuthorityMetadataResponse, cosmwasm_std::StdError> {
		QueryDenomAuthorityMetadataRequest { denom }.query(self.querier)
	}
	pub fn denoms_from_creator(
		&self,
		creator: ::prost::alloc::string::String,
	) -> Result<QueryDenomsFromCreatorResponse, cosmwasm_std::StdError> {
		QueryDenomsFromCreatorRequest { creator }.query(self.querier)
	}
}
