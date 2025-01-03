// std
use std::fs;
// self
use well_known_address::{r#type::*, registry::*};

fn main() -> anyhow::Result<()> {
	let registry = vec![
		UnifyRegistry::from(<Registry<Polkadot>>::new(vec![
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L37
			Address::pallet(*b"py/ambtr"),
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L35
			Address::pallet(*b"py/feltr"),
		])),
		<Registry<Collectives>>::new(vec![
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L21
			Address::pallet(*b"py/trsry"),
		])
		.into(),
	];
	let json = serde_json::to_string(&registry)?;

	println!("{json}");

	fs::write("registry.json", json)?;

	Ok(())
}
