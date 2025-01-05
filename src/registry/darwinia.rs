
// self
use super::*;

#[derive(Debug)]
pub struct Darwinia;
impl Spec for Darwinia {
	type AccountId = [u8; 20];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/darwinia-network/darwinia/blob/8476af2e40ecad47756d783e9c3dcbef4091df87/runtime/darwinia/src/lib.rs#L84
	const NAME: &'static str = "Darwinia2";
}
