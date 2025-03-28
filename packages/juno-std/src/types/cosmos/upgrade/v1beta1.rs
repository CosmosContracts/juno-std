use juno_std_derive::CosmwasmExt;
/// Plan specifies information about a planned upgrade and when it should occur.
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.Plan")]
pub struct Plan {
	/// Sets the name for the upgrade. This name will be used by the upgraded
	/// version of the software to apply any special "on-upgrade" commands during
	/// the first BeginBlock method after the upgrade is applied. It is also used
	/// to detect whether a software version can handle a given upgrade. If no
	/// upgrade handler with this name has been set in the software, it will be
	/// assumed that the software is out-of-date when the upgrade Time or Height is
	/// reached and the software will exit.
	#[prost(string, tag = "1")]
	pub name: ::prost::alloc::string::String,
	/// Deprecated: Time based upgrades have been deprecated. Time based upgrade logic
	/// has been removed from the SDK.
	/// If this field is not empty, an error will be thrown.
	#[deprecated]
	#[prost(message, optional, tag = "2")]
	pub time: ::core::option::Option<crate::shim::Timestamp>,
	/// The height at which the upgrade must be performed.
	#[prost(int64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub height: i64,
	/// Any application specific upgrade info to be included on-chain
	/// such as a git commit that validators could automatically upgrade to
	#[prost(string, tag = "4")]
	pub info: ::prost::alloc::string::String,
	/// Deprecated: UpgradedClientState field has been deprecated. IBC upgrade logic has been
	/// moved to the IBC module in the sub module 02-client.
	/// If this field is not empty, an error will be thrown.
	#[deprecated]
	#[prost(message, optional, tag = "5")]
	pub upgraded_client_state: ::core::option::Option<crate::shim::Any>,
}
/// SoftwareUpgradeProposal is a gov Content type for initiating a software
/// upgrade.
/// Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
/// proposals, see MsgSoftwareUpgrade.
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal")]
#[deprecated]
pub struct SoftwareUpgradeProposal {
	/// title of the proposal
	#[prost(string, tag = "1")]
	pub title: ::prost::alloc::string::String,
	/// description of the proposal
	#[prost(string, tag = "2")]
	pub description: ::prost::alloc::string::String,
	/// plan of the proposal
	#[prost(message, optional, tag = "3")]
	pub plan: ::core::option::Option<Plan>,
}
/// CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software
/// upgrade.
/// Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
/// proposals, see MsgCancelUpgrade.
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.CancelSoftwareUpgradeProposal")]
#[deprecated]
pub struct CancelSoftwareUpgradeProposal {
	/// title of the proposal
	#[prost(string, tag = "1")]
	pub title: ::prost::alloc::string::String,
	/// description of the proposal
	#[prost(string, tag = "2")]
	pub description: ::prost::alloc::string::String,
}
/// ModuleVersion specifies a module and its consensus version.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.ModuleVersion")]
pub struct ModuleVersion {
	/// name of the app module
	#[prost(string, tag = "1")]
	pub name: ::prost::alloc::string::String,
	/// consensus version of the app module
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub version: u64,
}
/// QueryCurrentPlanRequest is the request type for the Query/CurrentPlan RPC
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryCurrentPlanRequest")]
#[proto_query(
    path = "/cosmos.upgrade.v1beta1.Query/CurrentPlan",
    response_type = QueryCurrentPlanResponse
)]
pub struct QueryCurrentPlanRequest {}
/// QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryCurrentPlanResponse")]
pub struct QueryCurrentPlanResponse {
	/// plan is the current upgrade plan.
	#[prost(message, optional, tag = "1")]
	pub plan: ::core::option::Option<Plan>,
}
/// QueryCurrentPlanRequest is the request type for the Query/AppliedPlan RPC
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryAppliedPlanRequest")]
#[proto_query(
    path = "/cosmos.upgrade.v1beta1.Query/AppliedPlan",
    response_type = QueryAppliedPlanResponse
)]
pub struct QueryAppliedPlanRequest {
	/// name is the name of the applied plan to query for.
	#[prost(string, tag = "1")]
	pub name: ::prost::alloc::string::String,
}
/// QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryAppliedPlanResponse")]
pub struct QueryAppliedPlanResponse {
	/// height is the block height at which the plan was applied.
	#[prost(int64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub height: i64,
}
/// QueryUpgradedConsensusStateRequest is the request type for the Query/UpgradedConsensusState
/// RPC method.
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryUpgradedConsensusStateRequest")]
#[proto_query(
    path = "/cosmos.upgrade.v1beta1.Query/UpgradedConsensusState",
    response_type = QueryUpgradedConsensusStateResponse
)]
#[deprecated]
pub struct QueryUpgradedConsensusStateRequest {
	/// last height of the current chain must be sent in request
	/// as this is the height under which next consensus state is stored
	#[prost(int64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub last_height: i64,
}
/// QueryUpgradedConsensusStateResponse is the response type for the Query/UpgradedConsensusState
/// RPC method.
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryUpgradedConsensusStateResponse")]
#[deprecated]
pub struct QueryUpgradedConsensusStateResponse {
	/// Since: cosmos-sdk 0.43
	#[prost(bytes = "vec", tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub upgraded_consensus_state: ::prost::alloc::vec::Vec<u8>,
}
/// QueryModuleVersionsRequest is the request type for the Query/ModuleVersions
/// RPC method.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryModuleVersionsRequest")]
#[proto_query(
    path = "/cosmos.upgrade.v1beta1.Query/ModuleVersions",
    response_type = QueryModuleVersionsResponse
)]
pub struct QueryModuleVersionsRequest {
	/// module_name is a field to query a specific module
	/// consensus version from state. Leaving this empty will
	/// fetch the full list of module versions from state
	#[prost(string, tag = "1")]
	pub module_name: ::prost::alloc::string::String,
}
/// QueryModuleVersionsResponse is the response type for the Query/ModuleVersions
/// RPC method.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryModuleVersionsResponse")]
pub struct QueryModuleVersionsResponse {
	/// module_versions is a list of module names with their consensus versions.
	#[prost(message, repeated, tag = "1")]
	pub module_versions: ::prost::alloc::vec::Vec<ModuleVersion>,
}
/// QueryAuthorityRequest is the request type for Query/Authority
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryAuthorityRequest")]
#[proto_query(
    path = "/cosmos.upgrade.v1beta1.Query/Authority",
    response_type = QueryAuthorityResponse
)]
pub struct QueryAuthorityRequest {}
/// QueryAuthorityResponse is the response type for Query/Authority
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.QueryAuthorityResponse")]
pub struct QueryAuthorityResponse {
	#[prost(string, tag = "1")]
	pub address: ::prost::alloc::string::String,
}
/// MsgSoftwareUpgrade is the Msg/SoftwareUpgrade request type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade")]
pub struct MsgSoftwareUpgrade {
	/// authority is the address that controls the module (defaults to x/gov unless overwritten).
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
	/// plan is the upgrade plan.
	#[prost(message, optional, tag = "2")]
	pub plan: ::core::option::Option<Plan>,
}
/// MsgSoftwareUpgradeResponse is the Msg/SoftwareUpgrade response type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.MsgSoftwareUpgradeResponse")]
pub struct MsgSoftwareUpgradeResponse {}
/// MsgCancelUpgrade is the Msg/CancelUpgrade request type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.MsgCancelUpgrade")]
pub struct MsgCancelUpgrade {
	/// authority is the address that controls the module (defaults to x/gov unless overwritten).
	#[prost(string, tag = "1")]
	pub authority: ::prost::alloc::string::String,
}
/// MsgCancelUpgradeResponse is the Msg/CancelUpgrade response type.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.upgrade.v1beta1.MsgCancelUpgradeResponse")]
pub struct MsgCancelUpgradeResponse {}
pub struct UpgradeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
	querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> UpgradeQuerier<'a, Q> {
	pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
		Self { querier }
	}
	pub fn current_plan(&self) -> Result<QueryCurrentPlanResponse, cosmwasm_std::StdError> {
		QueryCurrentPlanRequest {}.query(self.querier)
	}
	pub fn applied_plan(
		&self,
		name: ::prost::alloc::string::String,
	) -> Result<QueryAppliedPlanResponse, cosmwasm_std::StdError> {
		QueryAppliedPlanRequest { name }.query(self.querier)
	}
	#[deprecated]
	pub fn upgraded_consensus_state(
		&self,
		last_height: i64,
	) -> Result<QueryUpgradedConsensusStateResponse, cosmwasm_std::StdError> {
		QueryUpgradedConsensusStateRequest { last_height }.query(self.querier)
	}
	pub fn module_versions(
		&self,
		module_name: ::prost::alloc::string::String,
	) -> Result<QueryModuleVersionsResponse, cosmwasm_std::StdError> {
		QueryModuleVersionsRequest { module_name }.query(self.querier)
	}
	pub fn authority(&self) -> Result<QueryAuthorityResponse, cosmwasm_std::StdError> {
		QueryAuthorityRequest {}.query(self.querier)
	}
}
