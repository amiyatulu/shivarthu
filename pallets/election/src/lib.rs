#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod extras;
mod types;

use crate::types::{DepartmentDetails, SeatHolder, Voter};

use frame_support::{
	traits::{
		Currency, ExistenceRequirement, Get, Imbalance, OnUnbalanced, ReservableCurrency,
		WithdrawReasons,
	},
	PalletId,
};

use frame_support::sp_runtime::traits::{CheckedAdd, CheckedMul, CheckedSub};
use frame_support::sp_std::vec::Vec;
use sp_runtime::{
	traits::{Saturating, StaticLookup, Zero},
	DispatchError, Perbill, RuntimeDebug,
};

pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>;
		/// Handler for the unbalanced increment when rewarding (minting rewards)
		type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

		/// Handler for the unbalanced decrement when slashing (burning collateral)
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	pub type Candidates<T: Config> =
		StorageMap<_, Blake2_128Concat, u128, Vec<(T::AccountId, BalanceOf<T>)>, ValueQuery>; // departmentid => Vec(Candidate Account Id and deposit)

	#[pallet::storage]
	#[pallet::getter(fn department_count)]
	pub type DepartmentCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn location_count)]
	pub type LocationCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn department)]
	pub type Department<T> = StorageMap<_, Blake2_128Concat, u128, DepartmentDetails>;

	#[pallet::type_value]
	pub fn DefaultDesiredMembers<T: Config>() -> u128 {
		1000
	}

	#[pallet::type_value]
	pub fn DefaultDesiredRunnersUp<T: Config>() -> u128 {
		1000
	}

	#[pallet::storage]
	#[pallet::getter(fn desired_members)]
	pub type DesiredMembers<T> =
		StorageMap<_, Blake2_128Concat, u128, u128, ValueQuery, DefaultDesiredMembers<T>>; // Department id => desired seats

	#[pallet::storage]
	#[pallet::getter(fn desired_runnersup)]
	pub type DesiredRunnersup<T> =
		StorageMap<_, Blake2_128Concat, u128, u128, ValueQuery, DefaultDesiredRunnersUp<T>>; // department id => desired runnersup

	// The current elected members.
	///
	/// Invariant: Always sorted based on account id.
	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128,
		Vec<SeatHolder<T::AccountId, BalanceOf<T>>>,
		ValueQuery,
	>; // department id => Vec <SeatHolder>

	/// The current reserved runners-up.
	///
	/// Invariant: Always sorted based on rank (worse to best). Upon removal of a member, the
	/// last (i.e. _best_) runner-up will be replaced.
	#[pallet::storage]
	#[pallet::getter(fn runners_up)]
	pub type RunnersUp<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128,
		Vec<SeatHolder<T::AccountId, BalanceOf<T>>>,
		ValueQuery,
	>; // department id => Vec<SeatHolder>

	/// Votes and experience score with score schelling game of a particular voter.
	///
	/// TWOX-NOTE: SAFE as `AccountId` is a crypto hash.
	#[pallet::storage]
	#[pallet::getter(fn voting)]
	pub type Voting<T: Config> =
		StorageDoubleMap<_, Twox64Concat, u128, Twox64Concat, T::AccountId, Voter<T::AccountId>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		EmptyTerm,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		EmptyTermError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn do_phragmen(origin: OriginFor<T>, departmentid: u128) -> DispatchResult {
			let desired_seats = <DesiredMembers<T>>::get(&departmentid) as usize;
			let desired_runners_up = <DesiredRunnersup<T>>::get(&departmentid) as usize;
			let num_to_elect = desired_runners_up + desired_seats;
			let mut candidates_and_deposit = Self::candidates(&departmentid);

			// add all the previous members and runners-up as candidates as well.
			candidates_and_deposit
				.append(&mut Self::implicit_candidates_with_deposit(departmentid));
			if candidates_and_deposit.len().is_zero() {
				Self::deposit_event(Event::EmptyTerm);
				Err(Error::<T>::EmptyTermError)?;
			}

			// All of the new winners that come out of phragmen will thus have a deposit recorded.
			let candidate_ids =
				candidates_and_deposit.iter().map(|(x, _)| x).cloned().collect::<Vec<_>>();

			let voters_and_score = <Voting<T>>::iter_prefix(&departmentid)
				.map(|(voter, Voter { score, votes, .. })| (voter, score, votes))
				.collect::<Vec<_>>();

			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
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
