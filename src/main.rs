// std
use std::fs;
// self
use well_known_address::{r#type::*, registry::*};

fn main() -> anyhow::Result<()> {
	let registry: Vec<UnifyRegistry> = vec![
		<Registry<Acala>>::new(vec![
			// https://github.com/AcalaNetwork/Acala/blob/9c6d516a9d7c5433b7c10230457a518936f8d9a3/runtime/acala/src/lib.rs#L156
			Addr::pallet(*b"aca/trsy"),
		])
		.into(),
		<Registry<Astar>>::new(vec![
			// https://github.com/AstarNetwork/Astar/blob/765d44ba1e9210279088ff765aff5ee6979601d2/runtime/astar/src/lib.rs#L612
			Addr::pallet(*b"py/trsry"),
		])
		.into(),
		<Registry<Collectives>>::new(vec![
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L21
			Addr::pallet(*b"py/trsry"),
		])
		.into(),
		<Registry<Darwinia>>::new(vec![
			// https://github.com/darwinia-network/darwinia/blob/8476af2e40ecad47756d783e9c3dcbef4091df87/runtime/darwinia/src/lib.rs#L84
			Addr::pallet(*b"da/trsry"),
		])
		.into(),
		<Registry<Moonbeam>>::new(vec![
			// https://github.com/moonbeam-foundation/moonbeam/blob/74e484b68d56cec5febaf41441f9d74a023512c4/runtime/moonbeam/src/lib.rs#L587
			Addr::pallet(*b"py/trsry"),
		])
		.into(),
		<Registry<Polkadot>>::new(vec![
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L37
			Addr::pallet(*b"py/ambtr"),
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L35
			Addr::pallet(*b"py/feltr"),
		])
		.into(),
	];
	let json = serde_json::to_string(&registry)?;

	println!("{json}");

	fs::write("registry.json", json)?;

	Ok(())
}
