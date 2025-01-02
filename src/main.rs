// std
use std::fs;
// self
use well_known_address::{r#type::*, registry::*};

fn main() -> anyhow::Result<()> {
	let registry = Registry {
		collectives: collectives::Registry {
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L37C63-L37C75
			ambassador_treasury: Pid::new(*b"py/ambtr"),
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L35
			fellowship_treasury: Pid::new(*b"py/feltr"),
		},
		polkadot: polkadot::Registry {
			// https://github.com/polkadot-fellows/runtimes/blob/2e73a6c90159b723c741b1a5b5898ba002c5e87d/system-parachains/constants/src/polkadot.rs#L21
			treasury: Pid::new(*b"py/trsry"),
		},
	};
	let json = serde_json::to_string(&registry)?;

	println!("{json}");

	fs::write("registry.json", json)?;

	Ok(())
}
