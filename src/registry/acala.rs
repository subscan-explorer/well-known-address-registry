// self
use super::*;

#[derive(Debug)]
pub struct Acala;
impl Spec for Acala {
	type AccountId = [u8; 32];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/AcalaNetwork/Acala/blob/9c6d516a9d7c5433b7c10230457a518936f8d9a3/runtime/acala/src/lib.rs#L128
	const NAME: &'static str = "acala";
}
