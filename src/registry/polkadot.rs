// self
use super::*;

pub struct Runtime;
impl Config for Runtime {
	type AccountId = AccountId32;
}

#[derive(Debug, Serialize)]
pub struct Registry<C>
where
	C: Config,
{
	pub treasury: Pid<C::AccountId>,
}
