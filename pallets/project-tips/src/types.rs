use super::*;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum TippingName {
	SmallTipper,
	BigTipper,
	SmallSpender,
	MediumSpender,
	BigSpender,
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TippingValue<Balance> {
	pub max_tipping_value: Balance,
	pub stake_required: Balance,
}

