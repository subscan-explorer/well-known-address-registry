// self
use super::*;

#[derive(Debug)]
pub struct Polkadot;
impl Spec for Polkadot {
	type AccountId = [u8; 32];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/relay/polkadot/src/lib.rs#L164
	const NAME: &'static str = "polkadot";
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Spec,
{
	pub treasury: Addr<C::AccountId, C::Ss58Ver>,
}
