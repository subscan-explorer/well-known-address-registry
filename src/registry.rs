pub mod collectives;
pub mod polkadot;

// crates.io
use polkadot_sdk_frame::deps::codec::FullCodec;
use serde::Serialize;
// self
use crate::r#type::*;

pub trait Config {
	type AccountId: AsRef<[u8]> + FullCodec;
}

#[derive(Debug, Serialize)]
pub struct Registry {
	pub collectives: collectives::Registry<collectives::Runtime>,
	pub polkadot: polkadot::Registry<collectives::Runtime>,
}
