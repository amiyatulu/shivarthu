#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;



#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod extras;
mod functions;
pub mod types;
mod score_game;
mod share_link;

use crate::types::{
	CommitVote, Period, PhaseData, RangePoint, RevealedVote, SchellingGameType, ScoreCommitVote,
	VoteStatus, WinningDecision,
};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::sp_std::prelude::*;
use frame_support::traits::Randomness;
use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
use num_integer::Roots;
use scale_info::prelude::format;
use sortition_sum_game::types::SumTreeName;
use sortition_sum_game_link::SortitionSumGameLink;

pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;
type PhaseDataOf<T> = PhaseData<T>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		type SortitionSumGameSource: SortitionSumGameLink<
			SumTreeName = SumTreeName<Self::AccountId, Self::BlockNumber>,
			AccountId = Self::AccountId,
		>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;

		/// Handler for the unbalanced increment when rewarding (minting rewards)
		type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

		/// Handler for the unbalanced decrement when slashing (burning collateral)
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn get_period)]
	pub type PeriodName<T> = StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Period>;

	#[pallet::storage]
	#[pallet::getter(fn draws_in_round)]
	pub type DrawsInRound<T> = StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, u64, ValueQuery>; // A counter of draws made in the current round.

	#[pallet::storage]
	#[pallet::getter(fn evidence_start_time)]
	pub type EvidenceStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn staking_start_time)]
	pub type StakingStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn commit_start_time)]
	pub type CommitStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vote_start_time)]
	pub type VoteStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, BlockNumberOf<T>, ValueQuery>;

	/// Drawn jurors containing account id and stake Vec<(AccountId, Stake)>
	/// Should be stored in sorted order by AccountId
	#[pallet::storage]
	#[pallet::getter(fn  drawn_jurors)]
	pub type DrawnJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<(T::AccountId, u64)>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn unstaked_jurors)]
	pub type UnstakedJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<T::AccountId>, ValueQuery>;

	/// VoteCommits for Yes or No voting
	#[pallet::storage]
	#[pallet::getter(fn vote_commits)]
	pub type VoteCommits<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		SumTreeNameType<T>,
		Blake2_128Concat,
		T::AccountId,
		CommitVote,
	>;

	/// Vote Commits for Score Schelling  
	#[pallet::storage]
	#[pallet::getter(fn vote_commits_score)]
	pub type ScoreVoteCommits<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		SumTreeNameType<T>,
		Blake2_128Concat,
		T::AccountId,
		ScoreCommitVote,
	>;

	/// Reveal values of score schelling game as Vec<i64>
	#[pallet::storage]
	#[pallet::getter(fn reveal_score_values)]
	pub type RevealScoreValues<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<i64>, ValueQuery>;

	/// New mean from the reveal values in score schelling game
	/// Improvement: This step will not be required if all jurors incentives are distributed at one time
	#[pallet::storage]
	#[pallet::getter(fn new_mean_reveal_score)]
	pub type IncentiveMeanRevealScore<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, i64, ValueQuery>;

	/// Decision count for two choices after reveal vote:  (count for 0, count for 1)
	#[pallet::storage]
	#[pallet::getter(fn decision_count)]
	pub type DecisionCount<T> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, (u64, u64), ValueQuery>; // Count for 0, Count for 1

	#[pallet::storage]
	#[pallet::getter(fn juror_incentive_distribution)]
	pub type JurorsIncentiveDistributedAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeNameType<T>, Vec<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		PeriodExists,
		EvidencePeriodNotOver,
		StakingPeriodNotOver,
		PeriodIsNotEvidence,
		PeriodIsNotNone,
		MaxJurorNotDrawn,
		CommitPeriodNotOver,
		VotePeriodNotOver,
		PeriodDoesNotExists,
		PeriodDontMatch,
		JurorStakeLessThanMin,
		AlreadyStaked,
		MaxDrawExceeded,
		SelectedAsJuror,
		AlreadyUnstaked,
		StakeDoesNotExists,
		JurorDoesNotExists,
		VoteStatusNotCommited,
		NotValidChoice,
		CommitDoesNotMatch,
		CommitDoesNotExists,
		AlreadyGotIncentives,
		VoteNotRevealed,
		TimeForStakingOver,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
