use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(
	PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo,
)]
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

impl<AccountId> Default for Voter<AccountId> {
	fn default() -> Self {
		Self { votes: vec![], score: Default::default() }
	}
}

/// An indication that the renouncing account currently has which of the below roles.
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
pub enum Renouncing {
	/// A member is renouncing.
	Member,
	/// A runner-up is renouncing.
	RunnerUp,
	/// A candidate is renouncing, while the given total number of candidates exists.
	Candidate(#[codec(compact)] u32),
}
