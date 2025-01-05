pub mod acala;
pub use acala::Acala;

pub mod astar;
pub use astar::Astar;

pub mod collectives;
pub use collectives::Collectives;

pub mod darwinia;
pub use darwinia::Darwinia;

pub mod moonbeam;
pub use moonbeam::Moonbeam;

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
	Acala(Registry<Acala>),
	Astar(Registry<Astar>),
	Collectives(Registry<Collectives>),
	Darwinia(Registry<Darwinia>),
	Moonbeam(Registry<Moonbeam>),
	Polkadot(Registry<Polkadot>),
}
macro_rules! impl_registry {
	($($spec:ident,)+) => {
		$(
			impl From<Registry<$spec>> for UnifyRegistry {
				fn from(registry: Registry<$spec>) -> Self {
					Self::$spec(registry)
				}
			}
		)+
	};
}
impl_registry! {
	Acala,
	Astar,
	Collectives,
	Darwinia,
	Moonbeam,
	Polkadot,
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Spec,
{
	spec_name: &'static str,
	address: Vec<Addr<C::AccountId, C::Ss58Ver>>,
}
impl<C> Registry<C>
where
	C: Spec,
{
	pub fn new(address: Vec<Addr<C::AccountId, C::Ss58Ver>>) -> Self {
		let spec_name = C::NAME;

		Self { spec_name, address }
	}
}
