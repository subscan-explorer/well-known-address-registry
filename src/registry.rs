pub mod collectives;
pub use collectives::Collectives;

pub mod polkadot;
pub use polkadot::Polkadot;

// crates.io
use polkadot_sdk_frame::traits::{ConstU16, Get};
use serde::Serialize;
// self
use crate::r#type::*;

pub trait Spec {
	// https://github.com/rust-lang/rust/issues/60551
	// const Ss58Ver: u16 = 0;

	const NAME: &'static str;

	type AccountId: Array;
	type Ss58Ver: Get<u16>;
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum UnifyRegistry {
	Polkadot(Registry<Polkadot>),
	Collectives(Registry<Collectives>),
}
impl From<Registry<Polkadot>> for UnifyRegistry {
	fn from(registry: Registry<Polkadot>) -> Self {
		Self::Polkadot(registry)
	}
}
impl From<Registry<Collectives>> for UnifyRegistry {
	fn from(registry: Registry<Collectives>) -> Self {
		Self::Collectives(registry)
	}
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Spec,
{
	spec_name: &'static str,
	address: Vec<Address<C::AccountId, C::Ss58Ver>>,
}
impl<C> Registry<C>
where
	C: Spec,
{
	pub fn new(address: Vec<Address<C::AccountId, C::Ss58Ver>>) -> Self {
		let spec_name = C::NAME;

		Self { spec_name, address }
	}
}
