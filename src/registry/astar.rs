// self
use super::*;

#[derive(Debug)]
pub struct Astar;
impl Spec for Astar {
	type AccountId = [u8; 32];
	type Ss58Ver = ConstU16<0>;

	// https://github.com/AstarNetwork/Astar/blob/765d44ba1e9210279088ff765aff5ee6979601d2/runtime/astar/src/lib.rs#L186
	const NAME: &'static str = "astar";
}
