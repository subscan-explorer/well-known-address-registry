// std
use std::marker::PhantomData;
// crates.io
use array_bytes::Hexify;
use polkadot_sdk_frame::{
	deps::{
		codec::Codec,
		frame_support::PalletId,
		sp_core::crypto::{AccountId32, Ss58Codec},
	},
	traits::{AccountIdConversion, Get},
};
use serde::Serialize;

pub trait Array
where
	Self: AsRef<[u8]> + Hexify,
{
}
impl<T> Array for T where T: AsRef<[u8]> + Hexify {}

#[derive(Debug, Serialize)]
pub enum Address<Ary, Ss58Ver>
where
	Ary: Array,
	Ss58Ver: Get<u16>,
{
	Pallet {
		#[serde(serialize_with = "array_bytes::ser_bytes_stringify")]
		id: [u8; 8],
		#[serde(serialize_with = "array_bytes::ser_hexify_prefixed")]
		pubkey: Ary,
		ss58: Option<String>,
		#[serde(skip)]
		_phantom: PhantomData<Ss58Ver>,
	},
}
impl<Ary, Ss58Ver> Address<Ary, Ss58Ver>
where
	Ary: Array + Codec,
	Ss58Ver: Get<u16>,
{
	pub fn pallet(id: [u8; 8]) -> Self {
		let pubkey: Ary = PalletId(id).into_account_truncating();
		// Only `AccountId32` can be converted to `SS58`.
		let ss58 = AccountId32::try_from(pubkey.as_ref())
			.ok()
			.map(|pubkey| pubkey.to_ss58check_with_version(Ss58Ver::get().into()));

		Self::Pallet { id, pubkey, ss58, _phantom: Default::default() }
	}
}
