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

use frame_support::sp_std::prelude::*;
// use scale_info::prelude::format;

type DepartmentId = u128;
type DownVoteNum = u8;
use frame_support::pallet_prelude::{DispatchResult, *};
use frame_system::pallet_prelude::*;
use types::{DownVoteDetails};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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

	/// Department tags
	#[pallet::storage]
	#[pallet::getter(fn department_tags)]
	pub(super) type Tags<T> =
		StorageMap<_, Blake2_128Concat, DepartmentId, Vec<Vec<u8>>, ValueQuery>;

	/// Down vote a tag
	#[pallet::storage]
	#[pallet::getter(fn downvote_details_of_tag)]
	pub(super) type DownVoteDetailsTags<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		DepartmentId,
		Blake2_128Concat,
		Vec<u8>,
		DownVoteDetails<T::AccountId>,
		ValueQuery,
	>;

	/// Default Threshold down vote for tag
	#[pallet::type_value]
	pub fn DefaultDownVoteThreshold() -> DownVoteNum {
		5
	}

	/// Threshold for down vote
	#[pallet::storage]
	#[pallet::getter(fn downvote_threshold)]
	pub type DownVoteThreshold<T> =
		StorageValue<_, DownVoteNum, ValueQuery, DefaultDownVoteThreshold>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		TagInserted(DepartmentId, Vec<u8>), // Tag inserted
		TagRemoved(DepartmentId, Vec<u8>),  // Tag removed
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		TagExists,
		TagDoesnotExists,
		UserAlreadyDownVoted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create tag
		/// [] Check who belongs to department representative
		/// [] Limit the length of tag
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_tag(
			origin: OriginFor<T>,
			departmentid: DepartmentId,
			tag: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut tags = Tags::<T>::get(&departmentid);

			match tags.binary_search(&tag) {
				Ok(_) => Err(Error::<T>::TagExists.into()),
				Err(index) => {
					tags.insert(index, tag.clone());
					Tags::<T>::insert(&departmentid, tags);
					Self::deposit_event(Event::TagInserted(departmentid, tag));
					Ok(())
				},
			}
		}
		/// Downvote tag
		/// [] Check who belongs to department representive
		/// [] Check tags exsts in Tags
		/// [✓] Check user has not downvoted again
		/// [✓] Delete tag if it reaches maximum downvote
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn donwvote_tag(
			origin: OriginFor<T>,
			departmentid: DepartmentId,
			tag: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_tag_exists(departmentid,tag.clone())?;
			let dv = Self::ensure_user_not_downvoted_then_downvote(departmentid, who, tag.clone())?;
			let threshold = DownVoteThreshold::<T>::get();

			if dv >= threshold {
				Self::remove_tags(departmentid, tag)?;
			}

			Ok(())
		}
        // Remove down vote

		
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			// let s = format!("The number is {}", 1);
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
