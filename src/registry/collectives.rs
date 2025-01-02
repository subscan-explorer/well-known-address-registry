// self
use super::*;

#[derive(Debug)]
pub struct Runtime;
impl Config for Runtime {
	type AccountId = AccountId32;
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Config,
{
	pub ambassador_treasury: Pid<C::AccountId>,
	pub fellowship_treasury: Pid<C::AccountId>,
}
