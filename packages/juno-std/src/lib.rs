#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

/// The version (commit hash) used when generating this library.
pub const JUNOD_VERSION: &str = include_str!("types/JUNO_COMMIT");

mod serde;
pub mod shim;

#[allow(deprecated)]
pub mod types;

pub use shim::{ cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins };
