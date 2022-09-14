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

/// The maximum votes allowed per voter.
pub const MAXIMUM_VOTE: usize = 16;

use crate::types::{DepartmentDetails, Renouncing, SeatHolder, Voter};


use frame_support::{
	traits::{
		defensive_prelude::*, Currency, CurrencyToVote, Get,
		OnUnbalanced, ReservableCurrency,
	},
};
use sp_npos_elections::{ElectionResult, ExtendedBalance};
use sp_runtime::{
	traits::Zero,
	DispatchError, Perbill,
};
use sp_std::prelude::*;


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

		/// Handler for the unbalanced reduction when a candidate has lost (and is not a runner-up)
		type LoserCandidate: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Handler for the unbalanced reduction when a member has been kicked.
		type KickedMember: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Convert a balance into a number used for election calculation.
		/// This must fit into a `u64` but is allowed to be sensibly lossy.
		type CurrencyToVote: CurrencyToVote<BalanceOf<Self>>;

		#[pallet::constant]
		type CandidacyBond: Get<BalanceOf<Self>>;
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

    
	// Departments will remain in separate pallet
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
		2
	}

	#[pallet::type_value]
	pub fn DefaultDesiredRunnersUp<T: Config>() -> u128 {
		2
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
	pub type Voting<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		u128,
		Twox64Concat,
		T::AccountId,
		Voter<T::AccountId>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		EmptyTerm,
		/// Note that old members and runners-up are also candidates.
		CandidateSlashed {
			candidate: <T as frame_system::Config>::AccountId,
			amount: BalanceOf<T>,
		},
		ElectionError,
		/// A seat holder was slashed by amount by being forcefully removed from the set.
		SeatHolderSlashed {
			seat_holder: <T as frame_system::Config>::AccountId,
			amount: BalanceOf<T>,
		},
		/// Someone has renounced their candidacy.
		Renounced {
			candidate: <T as frame_system::Config>::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		EmptyTermError,
		MaximumVotesExceeded,
		NoVotes,
		UnableToVote,
		TooManyVotes,
		InvalidWitnessData,
		DuplicatedCandidate,
		MemberSubmit,
		RunnerUpSubmit,
		InsufficientCandidateFunds,
		NotMember,
		InvalidRenouncing,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		// We get scores of the who for score schelling game pallet 🟩
		pub fn vote(
			origin: OriginFor<T>,
			departmentid: u128,
			votes: Vec<T::AccountId>,
			score: u64,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// votes should not be empty and more than `MAXIMUM_VOTE` in any case.
			ensure!(votes.len() <= MAXIMUM_VOTE, Error::<T>::MaximumVotesExceeded);
			ensure!(!votes.is_empty(), Error::<T>::NoVotes);

			let candidates_count = <Candidates<T>>::decode_len(&departmentid).unwrap_or(0);
			let members_count = <Members<T>>::decode_len(&departmentid).unwrap_or(0);
			let runners_up_count = <RunnersUp<T>>::decode_len(&departmentid).unwrap_or(0);

			// can never submit a vote of there are no members, and cannot submit more votes than
			// all potential vote targets.
			// addition is valid: candidates, members and runners-up will never overlap.
			let allowed_votes =
				candidates_count.saturating_add(members_count).saturating_add(runners_up_count);
			ensure!(!allowed_votes.is_zero(), Error::<T>::UnableToVote);
			ensure!(votes.len() <= allowed_votes, Error::<T>::TooManyVotes);

			Voting::<T>::insert(&departmentid, &who, Voter { votes, score });

			Ok(None.into())
		}
        

		/// Submit oneself for candidacy. A fixed amount of deposit is recorded.
		///
		/// All candidates are wiped at the end of the term. They either become a member/runner-up,
		/// or leave the system while their deposit is slashed.
		///
		/// The dispatch origin of this call must be signed.
		///
		/// ### Warning
		///
		/// Even if a candidate ends up being a member, they must call [`Call::renounce_candidacy`]
		/// to get their deposit back. Losing the spot in an election will always lead to a slash.
		///
		/// # <weight>
		/// The number of current candidates must be provided as witness data.
		/// # </weight>
		/// 
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn submit_candidacy(
			origin: OriginFor<T>,
			departmentid: u128,
			#[pallet::compact] candidate_count: u32,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let actual_count = <Candidates<T>>::decode_len(&departmentid).unwrap_or(0);
			ensure!(actual_count as u32 <= candidate_count, Error::<T>::InvalidWitnessData);

			let index = Self::is_candidate(&who, departmentid)
				.err()
				.ok_or(Error::<T>::DuplicatedCandidate)?;

			ensure!(!Self::is_member(&who, departmentid), Error::<T>::MemberSubmit);
			ensure!(!Self::is_runner_up(&who, departmentid), Error::<T>::RunnerUpSubmit);

			T::Currency::reserve(&who, T::CandidacyBond::get())
				.map_err(|_| Error::<T>::InsufficientCandidateFunds)?;

			<Candidates<T>>::mutate(departmentid, |c| {
				c.insert(index, (who, T::CandidacyBond::get()))
			});
			Ok(None.into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn renounce_candidacy(
			origin: OriginFor<T>,
			renouncing: Renouncing,
			departmentid: u128,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match renouncing {
				Renouncing::Member => {
					let _ = Self::remove_and_replace_member(&who, false, departmentid)
						.map_err(|_| Error::<T>::InvalidRenouncing)?;
					Self::deposit_event(Event::Renounced { candidate: who });
				},
				Renouncing::RunnerUp => {
					<RunnersUp<T>>::try_mutate::<_, _, Error<T>, _>(departmentid, |runners_up| {
						let index = runners_up
							.iter()
							.position(|SeatHolder { who: r, .. }| r == &who)
							.ok_or(Error::<T>::InvalidRenouncing)?;
						// can't fail anymore.
						let SeatHolder { deposit, .. } = runners_up.remove(index);
						let _remainder = T::Currency::unreserve(&who, deposit);
						debug_assert!(_remainder.is_zero());
						Self::deposit_event(Event::Renounced { candidate: who });
						Ok(())
					})?;
				},
				Renouncing::Candidate(count) => {
					<Candidates<T>>::try_mutate::<_, _, Error<T>, _>(departmentid, |candidates| {
						ensure!(count >= candidates.len() as u32, Error::<T>::InvalidWitnessData);
						let index = candidates
							.binary_search_by(|(c, _)| c.cmp(&who))
							.map_err(|_| Error::<T>::InvalidRenouncing)?;
						let (_removed, deposit) = candidates.remove(index);
						let _remainder = T::Currency::unreserve(&who, deposit);
						debug_assert!(_remainder.is_zero());
						Self::deposit_event(Event::Renounced { candidate: who });
						Ok(())
					})?;
				},
			};
			Ok(None.into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn do_phragmen(origin: OriginFor<T>, departmentid: u128) -> DispatchResult {
			let _who = ensure_signed(origin)?;
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

			// helper closures to deal with balance/stake.
			let total_issuance = T::Currency::total_issuance();
			let _to_votes = |b: BalanceOf<T>| T::CurrencyToVote::to_vote(b, total_issuance);
			let to_balance = |e: ExtendedBalance| T::CurrencyToVote::to_currency(e, total_issuance);
			let voters_and_score = <Voting<T>>::iter_prefix(&departmentid)
				.map(|(voter, Voter { score, votes, .. })| (voter, score, votes))
				.collect::<Vec<_>>();

			let _ = sp_npos_elections::seq_phragmen(
				num_to_elect,
				candidate_ids,
				voters_and_score,
				None,
			)
			.map(|ElectionResult::<T::AccountId, Perbill> { winners, assignments: _ }| {
				// this is already sorted by id.
				let _old_members_ids_sorted = <Members<T>>::take(departmentid)
					.into_iter()
					.map(|m| m.who)
					.collect::<Vec<T::AccountId>>();

				// this one needs a sort by id
				let mut old_runners_up_ids_sorted = <RunnersUp<T>>::take(departmentid)
					.into_iter()
					.map(|r| r.who)
					.collect::<Vec<T::AccountId>>();
				old_runners_up_ids_sorted.sort();

				// filter out those who end up with no backing stake.
				let mut new_set_with_stake = winners
					.into_iter()
					.filter_map(|(m, b)| if b.is_zero() { None } else { Some((m, to_balance(b))) })
					.collect::<Vec<(T::AccountId, BalanceOf<T>)>>();
				// split new set into winners and runners up.
				let split_point = desired_seats.min(new_set_with_stake.len());
				let mut new_members_sorted_by_id =
					new_set_with_stake.drain(..split_point).collect::<Vec<_>>();
				new_members_sorted_by_id.sort_by(|i, j| i.0.cmp(&j.0));

				// all the rest will be runners-up
				new_set_with_stake.reverse();
				let new_runners_up_sorted_by_rank = new_set_with_stake;
				let mut new_runners_up_ids_sorted = new_runners_up_sorted_by_rank
					.iter()
					.map(|(r, _)| r.clone())
					.collect::<Vec<_>>();
				new_runners_up_ids_sorted.sort();

				// new_members_sorted_by_id is sorted by account id.
				let new_members_ids_sorted = new_members_sorted_by_id
					.iter()
					.map(|(m, _)| m.clone())
					.collect::<Vec<T::AccountId>>();

				// All candidates/members/runners-up who are no longer retaining a position as a
				// seat holder will lose their bond.
				candidates_and_deposit.iter().for_each(|(c, d)| {
					if new_members_ids_sorted.binary_search(c).is_err()
						&& new_runners_up_ids_sorted.binary_search(c).is_err()
					{
						let (imbalance, _) = T::Currency::slash_reserved(c, *d);
						T::LoserCandidate::on_unbalanced(imbalance);
						Self::deposit_event(Event::CandidateSlashed {
							candidate: c.clone(),
							amount: *d,
						});
					}
				});
				// write final values to storage.
				let deposit_of_candidate = |x: &T::AccountId| -> BalanceOf<T> {
					// defensive-only. This closure is used against the new members and new
					// runners-up, both of which are phragmen winners and thus must have
					// deposit.
					candidates_and_deposit
						.iter()
						.find_map(|(c, d)| if c == x { Some(*d) } else { None })
						.defensive_unwrap_or_default()
				};

				// fetch deposits from the one recorded one. This will make sure that a
				// candidate who submitted candidacy before a change to candidacy deposit will
				// have the correct amount recorded.
				<Members<T>>::insert(
					departmentid,
					new_members_sorted_by_id
						.iter()
						.map(|(who, stake)| SeatHolder {
							deposit: deposit_of_candidate(who),
							who: who.clone(),
							stake: *stake,
						})
						.collect::<Vec<_>>(),
				);

				<RunnersUp<T>>::insert(
					departmentid,
					new_runners_up_sorted_by_rank
						.into_iter()
						.map(|(who, stake)| SeatHolder {
							deposit: deposit_of_candidate(&who),
							who,
							stake,
						})
						.collect::<Vec<_>>(),
				);

				// clean candidates.
				<Candidates<T>>::remove(&departmentid);
			})
			.map_err(|e| {
				log::error!(
					target: "runtime::elections-phragmen",
					"Failed to run election [{:?}].",
					e,
				);
				Self::deposit_event(Event::ElectionError);
			});

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
