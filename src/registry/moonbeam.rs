// self
use super::*;

#[derive(Debug)]
pub struct Moonbeam;
impl Spec for Moonbeam {
	type AccountId = [u8; 20];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/moonbeam-foundation/moonbeam/blob/74e484b68d56cec5febaf41441f9d74a023512c4/runtime/moonbeam/src/lib.rs#L193
	const NAME: &'static str = "moonbeam";
}
