use juno_std_derive::CosmwasmExt;
/// Config is the config object of the x/auth/tx package.
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
#[proto_message(type_url = "/cosmos.tx.config.v1.Config")]
pub struct Config {
	/// skip_ante_handler defines whether the ante handler registration should be skipped in case an app wants to override
	/// this functionality.
	#[prost(bool, tag = "1")]
	pub skip_ante_handler: bool,
	/// skip_post_handler defines whether the post handler registration should be skipped in case an app wants to override
	/// this functionality.
	#[prost(bool, tag = "2")]
	pub skip_post_handler: bool,
}
