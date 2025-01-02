pub use polkadot_sdk_frame::deps::sp_core::crypto::AccountId32;

// std
use std::marker::PhantomData;
// crates.io
use array_bytes::Hexify;
use polkadot_sdk_frame::{
	deps::{codec::FullCodec, frame_support::PalletId},
	traits::AccountIdConversion,
};
use serde::{Serialize, Serializer};

// TODO: adaptation for `AccountId20`.
#[derive(Debug)]
pub struct Pid<Id>([u8; 8], PhantomData<Id>);
impl<Id> Pid<Id> {
	pub fn new(pid: [u8; 8]) -> Self {
		Self(pid, Default::default())
	}
}
impl<Id> Serialize for Pid<Id>
where
	Id: AsRef<[u8]> + FullCodec,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		<PalletId as AccountIdConversion<Id>>::into_account_truncating(&PalletId(self.0))
			.as_ref()
			.hexify_prefixed()
			.serialize(serializer)
	}
}
