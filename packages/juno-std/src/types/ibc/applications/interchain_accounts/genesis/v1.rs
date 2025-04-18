use juno_std_derive::CosmwasmExt;
/// GenesisState defines the interchain accounts genesis state
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.genesis.v1.GenesisState")]
pub struct GenesisState {
	#[prost(message, optional, tag = "1")]
	pub controller_genesis_state: ::core::option::Option<ControllerGenesisState>,
	#[prost(message, optional, tag = "2")]
	pub host_genesis_state: ::core::option::Option<HostGenesisState>,
}
/// ControllerGenesisState defines the interchain accounts controller genesis state
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
#[proto_message(
	type_url = "/ibc.applications.interchain_accounts.genesis.v1.ControllerGenesisState"
)]
pub struct ControllerGenesisState {
	#[prost(message, repeated, tag = "1")]
	pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
	#[prost(message, repeated, tag = "2")]
	pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
	#[prost(string, repeated, tag = "3")]
	pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
	#[prost(message, optional, tag = "4")]
	pub params: ::core::option::Option<super::super::controller::v1::Params>,
}
/// HostGenesisState defines the interchain accounts host genesis state
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.genesis.v1.HostGenesisState")]
pub struct HostGenesisState {
	#[prost(message, repeated, tag = "1")]
	pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
	#[prost(message, repeated, tag = "2")]
	pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
	#[prost(string, tag = "3")]
	pub port: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "4")]
	pub params: ::core::option::Option<super::super::host::v1::Params>,
}
/// ActiveChannel contains a connection ID, port ID and associated active channel ID, as well as a boolean flag to
/// indicate if the channel is middleware enabled
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.genesis.v1.ActiveChannel")]
pub struct ActiveChannel {
	#[prost(string, tag = "1")]
	#[serde(alias = "connectionID")]
	pub connection_id: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	#[serde(alias = "portID")]
	pub port_id: ::prost::alloc::string::String,
	#[prost(string, tag = "3")]
	#[serde(alias = "channelID")]
	pub channel_id: ::prost::alloc::string::String,
	#[prost(bool, tag = "4")]
	pub is_middleware_enabled: bool,
}
/// RegisteredInterchainAccount contains a connection ID, port ID and associated interchain account address
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
#[proto_message(
	type_url = "/ibc.applications.interchain_accounts.genesis.v1.RegisteredInterchainAccount"
)]
pub struct RegisteredInterchainAccount {
	#[prost(string, tag = "1")]
	#[serde(alias = "connectionID")]
	pub connection_id: ::prost::alloc::string::String,
	#[prost(string, tag = "2")]
	#[serde(alias = "portID")]
	pub port_id: ::prost::alloc::string::String,
	#[prost(string, tag = "3")]
	pub account_address: ::prost::alloc::string::String,
}
