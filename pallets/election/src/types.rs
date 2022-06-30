use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec};
use scale_info::TypeInfo;
use frame_support::traits::ConstU32;


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct DepartmentDetails {
    pub name: Vec<u8>,
    pub locationid: u128,
    pub details: Vec<u8>,
    pub departmentid: u128,
}

/// A holder of a seat as either a member or a runner-up.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, TypeInfo)]
pub struct SeatHolder<AccountId, Balance> {
	/// The holder.
	pub who: AccountId,
	/// The total backing stake.
	pub stake: Balance,
	/// The amount of deposit held on-chain.
	///
	/// To be unreserved upon renouncing, or slashed upon being a loser.
	pub deposit: Balance,
}


/// An active voter.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Voter<AccountId> {
	/// The members being backed.
	pub votes: Vec<AccountId>,
	/// The amount of stake placed on this vote.
	pub score: u64,
}