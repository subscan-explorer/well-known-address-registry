// self
use super::*;

#[derive(Debug)]
pub struct Collectives;
impl Spec for Collectives {
	type AccountId = [u8; 32];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/collectives/collectives-polkadot/src/lib.rs#L125
	const NAME: &'static str = "collectives";
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Spec,
{
	pub ambassador_treasury: Addr<C::AccountId, C::Ss58Ver>,
	pub fellowship_treasury: Addr<C::AccountId, C::Ss58Ver>,
}
