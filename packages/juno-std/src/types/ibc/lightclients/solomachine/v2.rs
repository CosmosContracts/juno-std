use juno_std_derive::CosmwasmExt;
/// ClientState defines a solo machine client that tracks the current consensus
/// state and if the client is frozen.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ClientState")]
pub struct ClientState {
	/// latest sequence of the client state
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub sequence: u64,
	/// frozen sequence of the solo machine
	#[prost(bool, tag = "2")]
	pub is_frozen: bool,
	#[prost(message, optional, tag = "3")]
	pub consensus_state: ::core::option::Option<ConsensusState>,
	/// when set to true, will allow governance to update a solo machine client.
	/// The client will be unfrozen if it is frozen.
	#[prost(bool, tag = "4")]
	pub allow_update_after_proposal: bool,
}
/// ConsensusState defines a solo machine consensus state. The sequence of a
/// consensus state is contained in the "height" key used in storing the
/// consensus state.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ConsensusState")]
pub struct ConsensusState {
	/// public key of the solo machine
	#[prost(message, optional, tag = "1")]
	pub public_key: ::core::option::Option<crate::shim::Any>,
	/// diversifier allows the same public key to be re-used across different solo
	/// machine clients (potentially on different chains) without being considered
	/// misbehaviour.
	#[prost(string, tag = "2")]
	pub diversifier: ::prost::alloc::string::String,
	#[prost(uint64, tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub timestamp: u64,
}
/// Header defines a solo machine consensus header
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.Header")]
pub struct Header {
	/// sequence to update solo machine public key at
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub sequence: u64,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub timestamp: u64,
	#[prost(bytes = "vec", tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub signature: ::prost::alloc::vec::Vec<u8>,
	#[prost(message, optional, tag = "4")]
	pub new_public_key: ::core::option::Option<crate::shim::Any>,
	#[prost(string, tag = "5")]
	pub new_diversifier: ::prost::alloc::string::String,
}
/// Misbehaviour defines misbehaviour for a solo machine which consists
/// of a sequence and two signatures over different messages at that sequence.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.Misbehaviour")]
pub struct Misbehaviour {
	#[prost(string, tag = "1")]
	#[serde(alias = "clientID")]
	pub client_id: ::prost::alloc::string::String,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub sequence: u64,
	#[prost(message, optional, tag = "3")]
	pub signature_one: ::core::option::Option<SignatureAndData>,
	#[prost(message, optional, tag = "4")]
	pub signature_two: ::core::option::Option<SignatureAndData>,
}
/// SignatureAndData contains a signature and the data signed over to create that
/// signature.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.SignatureAndData")]
pub struct SignatureAndData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub signature: ::prost::alloc::vec::Vec<u8>,
	#[prost(enumeration = "DataType", tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub data_type: i32,
	#[prost(bytes = "vec", tag = "3")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub data: ::prost::alloc::vec::Vec<u8>,
	#[prost(uint64, tag = "4")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub timestamp: u64,
}
/// TimestampedSignatureData contains the signature data and the timestamp of the
/// signature.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.TimestampedSignatureData")]
pub struct TimestampedSignatureData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub signature_data: ::prost::alloc::vec::Vec<u8>,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub timestamp: u64,
}
/// SignBytes defines the signed bytes used for signature verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.SignBytes")]
pub struct SignBytes {
	#[prost(uint64, tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub sequence: u64,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub timestamp: u64,
	#[prost(string, tag = "3")]
	pub diversifier: ::prost::alloc::string::String,
	/// type of the data used
	#[prost(enumeration = "DataType", tag = "4")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub data_type: i32,
	/// marshaled data
	#[prost(bytes = "vec", tag = "5")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub data: ::prost::alloc::vec::Vec<u8>,
}
/// HeaderData returns the SignBytes data for update verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.HeaderData")]
pub struct HeaderData {
	/// header public key
	#[prost(message, optional, tag = "1")]
	pub new_pub_key: ::core::option::Option<crate::shim::Any>,
	/// header diversifier
	#[prost(string, tag = "2")]
	pub new_diversifier: ::prost::alloc::string::String,
}
/// ClientStateData returns the SignBytes data for client state verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ClientStateData")]
pub struct ClientStateData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(message, optional, tag = "2")]
	pub client_state: ::core::option::Option<crate::shim::Any>,
}
/// ConsensusStateData returns the SignBytes data for consensus state
/// verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ConsensusStateData")]
pub struct ConsensusStateData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(message, optional, tag = "2")]
	pub consensus_state: ::core::option::Option<crate::shim::Any>,
}
/// ConnectionStateData returns the SignBytes data for connection state
/// verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ConnectionStateData")]
pub struct ConnectionStateData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(message, optional, tag = "2")]
	pub connection:
		::core::option::Option<super::super::super::core::connection::v1::ConnectionEnd>,
}
/// ChannelStateData returns the SignBytes data for channel state
/// verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.ChannelStateData")]
pub struct ChannelStateData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(message, optional, tag = "2")]
	pub channel: ::core::option::Option<super::super::super::core::channel::v1::Channel>,
}
/// PacketCommitmentData returns the SignBytes data for packet commitment
/// verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.PacketCommitmentData")]
pub struct PacketCommitmentData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(bytes = "vec", tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub commitment: ::prost::alloc::vec::Vec<u8>,
}
/// PacketAcknowledgementData returns the SignBytes data for acknowledgement
/// verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.PacketAcknowledgementData")]
pub struct PacketAcknowledgementData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(bytes = "vec", tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
}
/// PacketReceiptAbsenceData returns the SignBytes data for
/// packet receipt absence verification.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.PacketReceiptAbsenceData")]
pub struct PacketReceiptAbsenceData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
}
/// NextSequenceRecvData returns the SignBytes data for verification of the next
/// sequence to be received.
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
#[proto_message(type_url = "/ibc.lightclients.solomachine.v2.NextSequenceRecvData")]
pub struct NextSequenceRecvData {
	#[prost(bytes = "vec", tag = "1")]
	#[serde(
		serialize_with = "crate::serde::as_base64_encoded_string::serialize",
		deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
	)]
	pub path: ::prost::alloc::vec::Vec<u8>,
	#[prost(uint64, tag = "2")]
	#[serde(
		serialize_with = "crate::serde::as_str::serialize",
		deserialize_with = "crate::serde::as_str::deserialize"
	)]
	pub next_seq_recv: u64,
}
/// DataType defines the type of solo machine proof being created. This is done
/// to preserve uniqueness of different data sign byte encodings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum DataType {
	/// Default State
	UninitializedUnspecified = 0,
	/// Data type for client state verification
	ClientState = 1,
	/// Data type for consensus state verification
	ConsensusState = 2,
	/// Data type for connection state verification
	ConnectionState = 3,
	/// Data type for channel state verification
	ChannelState = 4,
	/// Data type for packet commitment verification
	PacketCommitment = 5,
	/// Data type for packet acknowledgement verification
	PacketAcknowledgement = 6,
	/// Data type for packet receipt absence verification
	PacketReceiptAbsence = 7,
	/// Data type for next sequence recv verification
	NextSequenceRecv = 8,
	/// Data type for header verification
	Header = 9,
}
impl DataType {
	/// String value of the enum field names used in the ProtoBuf definition.
	///
	/// The values are not transformed in any way and thus are considered stable
	/// (if the ProtoBuf definition does not change) and safe for programmatic use.
	pub fn as_str_name(&self) -> &'static str {
		match self {
			DataType::UninitializedUnspecified => "DATA_TYPE_UNINITIALIZED_UNSPECIFIED",
			DataType::ClientState => "DATA_TYPE_CLIENT_STATE",
			DataType::ConsensusState => "DATA_TYPE_CONSENSUS_STATE",
			DataType::ConnectionState => "DATA_TYPE_CONNECTION_STATE",
			DataType::ChannelState => "DATA_TYPE_CHANNEL_STATE",
			DataType::PacketCommitment => "DATA_TYPE_PACKET_COMMITMENT",
			DataType::PacketAcknowledgement => "DATA_TYPE_PACKET_ACKNOWLEDGEMENT",
			DataType::PacketReceiptAbsence => "DATA_TYPE_PACKET_RECEIPT_ABSENCE",
			DataType::NextSequenceRecv => "DATA_TYPE_NEXT_SEQUENCE_RECV",
			DataType::Header => "DATA_TYPE_HEADER",
		}
	}
	/// Creates an enum from field names used in the ProtoBuf definition.
	pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
		match value {
			"DATA_TYPE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
			"DATA_TYPE_CLIENT_STATE" => Some(Self::ClientState),
			"DATA_TYPE_CONSENSUS_STATE" => Some(Self::ConsensusState),
			"DATA_TYPE_CONNECTION_STATE" => Some(Self::ConnectionState),
			"DATA_TYPE_CHANNEL_STATE" => Some(Self::ChannelState),
			"DATA_TYPE_PACKET_COMMITMENT" => Some(Self::PacketCommitment),
			"DATA_TYPE_PACKET_ACKNOWLEDGEMENT" => Some(Self::PacketAcknowledgement),
			"DATA_TYPE_PACKET_RECEIPT_ABSENCE" => Some(Self::PacketReceiptAbsence),
			"DATA_TYPE_NEXT_SEQUENCE_RECV" => Some(Self::NextSequenceRecv),
			"DATA_TYPE_HEADER" => Some(Self::Header),
			_ => None,
		}
	}
}
