#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

// To dos:

// Unstake jurors in one go
// Give incentives in one go 
// Tests

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod extras;
pub mod types;
mod score_game;
mod share_link;

use crate::types::{
	CommitVote, DrawJurorsLimit, Period, SchellingGameType, StakingTime, VoteStatus, RevealedVote, WinningDecision, ScoreCommitVote, RangePoint
};
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::sp_std::prelude::*;
use frame_support::traits::Randomness;
use frame_support::{
	traits::{
		Currency, OnUnbalanced, ReservableCurrency,
	},
};
use scale_info::prelude::format;
use num_integer::Roots;
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

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type SortitionSumGameSource: SortitionSumGameLink<
			SumTreeName = SumTreeName,
			AccountId = Self::AccountId,
		>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;

		/// Handler for the unbalanced increment when rewarding (minting rewards)
		type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

		/// Handler for the unbalanced decrement when slashing (burning collateral)
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// Schelling Game Storage:

	#[pallet::storage]
	pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_period)]
	pub type PeriodName<T> = StorageMap<_, Blake2_128Concat, SumTreeName, Period>;

	#[pallet::type_value]
	pub fn DefaultMinBlockTime<T: Config>() -> StakingTime<BlockNumberOf<T>> {
		let staking_time = StakingTime {
			min_short_block_length: 50u128.saturated_into::<BlockNumberOf<T>>(),
			min_long_block_length: 80u128.saturated_into::<BlockNumberOf<T>>(),
		};
		staking_time
		// 6 sec (1 block)
		// 3 days (43200), 10 days (144000)
		// 15 mins (150)
		// 5 mins (50)
		// 8 mins (80)
	}

	///`StakingTime` `min_short_block_length` for changing `Period::Evidence` to `Period::Staking`   
	///`StakingTime` `min_long_block_length` for changing other periods in `change_period`   
	#[pallet::storage]
	#[pallet::getter(fn min_block_time)]
	pub type MinBlockTime<T> = StorageMap<
		_,
		Blake2_128Concat,
		SchellingGameType,
		StakingTime<BlockNumberOf<T>>,
		ValueQuery,
		DefaultMinBlockTime<T>,
	>;

	#[pallet::type_value]
	pub fn DefaultMinStake<T: Config>() -> BalanceOf<T> {
		100u128.saturated_into::<BalanceOf<T>>()
	}

	#[pallet::storage]
	#[pallet::getter(fn min_juror_stake)]
	pub type MinJurorStake<T> = StorageMap<
		_,
		Blake2_128Concat,
		SchellingGameType,
		BalanceOf<T>,
		ValueQuery,
		DefaultMinStake<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn draws_in_round)]
	pub type DrawsInRound<T> = StorageMap<_, Blake2_128Concat, SumTreeName, u64, ValueQuery>; // A counter of draws made in the current round.


	#[pallet::storage]
	#[pallet::getter(fn evidence_start_time)]
	pub type EvidenceStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn staking_start_time)]
	pub type StakingStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn commit_start_time)]
	pub type CommitStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vote_start_time)]
	pub type VoteStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	
	/// Drawn jurors containing account id and stake Vec<(AccountId, Stake)>
	/// Should be stored in sorted order by AccountId
	#[pallet::storage]
	#[pallet::getter(fn  drawn_jurors)]
	pub type DrawnJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeName, Vec<(T::AccountId, u64)>, ValueQuery>;
	#[pallet::storage]
	#[pallet::getter(fn unstaked_jurors)]
	pub type UnstakedJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeName, Vec<T::AccountId>, ValueQuery>;

	#[pallet::type_value]
	pub fn DefaultDrawJurorsLimitNum<T: Config>() -> DrawJurorsLimit {
		let draw_juror_limit = DrawJurorsLimit { max_draws: 5, max_draws_appeal: 10 };
		// change max draws more than 30 in production
		draw_juror_limit
	}

	#[pallet::storage]
	#[pallet::getter(fn draw_jurors_for_profile_limit)]
	pub type DrawJurorsLimitNum<T> = StorageMap<
		_,
		Blake2_128Concat,
		SchellingGameType,
		DrawJurorsLimit,
		ValueQuery,
		DefaultDrawJurorsLimitNum<T>,
	>;

	/// VoteCommits for Yes or No voting
	#[pallet::storage]
	#[pallet::getter(fn vote_commits)]
	pub type VoteCommits<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		SumTreeName,
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
		SumTreeName,
		Blake2_128Concat,
		T::AccountId,
		ScoreCommitVote,
	>;

	/// Reveal values of score schelling game as Vec<i64>
	#[pallet::storage]
	#[pallet::getter(fn reveal_score_values)]
	pub type RevealScoreValues<T: Config> = StorageMap<
	_,
	Blake2_128Concat,
	SumTreeName,
	Vec<i64>,
	ValueQuery,
	>;

	/// New mean from the reveal values in score schelling game
	/// Improvement: This step will not be required if all jurors incentives are distributed at one time	
	#[pallet::storage]
	#[pallet::getter(fn new_mean_reveal_score)]
	pub type IncentiveMeanRevealScore<T: Config> = StorageMap<
	_,
	Blake2_128Concat,
	SumTreeName,
	i64
	>;

    /// Decision count for two choices after reveal vote:  (count for 0, count for 1)
	#[pallet::storage]
	#[pallet::getter(fn decision_count)]
	pub type DecisionCount<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, (u64, u64), ValueQuery>; // Count for 0, Count for 1
	#[pallet::type_value]
	pub fn DefaultJurorIncentives<T: Config>() -> (u64, u64) {
		(100, 100)
	}
    
	/// Total amount of incentives distributed to jurors. 
	/// Improvements: Increase incentives on appeal.
	#[pallet::storage]
	#[pallet::getter(fn juror_incentives)]
	pub type JurorIncentives<T> = StorageMap<
		_,
		Blake2_128Concat,
		SchellingGameType,
		(u64, u64), // (looser burn, winner mint)
		ValueQuery,
		DefaultJurorIncentives<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn juror_incentive_distribution)]
	pub type JurorsIncentiveDistributedAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeName, Vec<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
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
		MaxJurorNotDrawn,
		CommitPeriodNotOver,
		VotePeriodNotOver,
		PeriodDoesNotExists,
		PeriodDontMatch,
		StakeLessThanMin,
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
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
