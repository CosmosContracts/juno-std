use juno_std_derive::CosmwasmExt;
/// Module is the config object of the crisis module.
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
#[proto_message(type_url = "/cosmos.crisis.module.v1.Module")]
pub struct Module {
	/// fee_collector_name is the name of the FeeCollector ModuleAccount.
	#[prost(string, tag = "1")]
	pub fee_collector_name: ::prost::alloc::string::String,
	/// authority defines the custom module authority. If not set, defaults to the governance module.
	#[prost(string, tag = "2")]
	pub authority: ::prost::alloc::string::String,
}
