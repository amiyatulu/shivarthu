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
pub mod types;

use frame_support::sp_std::{prelude::*};
use shared_storage_link::SharedStorageLink;
use schelling_game_shared::types::{Period, RangePoint, SchellingGameType};
use schelling_game_shared_link::SchellingGameSharedLink;
use sortition_sum_game::types::SumTreeName;
use frame_support::{
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
	PalletId,
};
use pallet_support::{
    ensure_content_is_valid, new_who_and_when, remove_from_vec,
    Content, PositiveExternalityPostId, WhoAndWhen, WhoAndWhenOf,
};
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;


// use scale_info::prelude::format;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type SharedStorageSource: SharedStorageLink<AccountId = AccountIdOf<Self>>;
		type SchellingGameSharedSource: SchellingGameSharedLink<
			SumTreeName = SumTreeName,
			SchellingGameType = SchellingGameType,
			BlockNumber = Self::BlockNumber,
			AccountId = AccountIdOf<Self>,
			Balance = BalanceOf<Self>,
			RangePoint = RangePoint,
			Period = Period,
		>;
		type Currency: ReservableCurrency<Self::AccountId>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// #[pallet::storage]
	// #[pallet::getter(fn positive_externality_evidence)]
	// pub type PositiveExternalityScore<T: Config> = 


	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
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
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
                // Every 3 months 
                // Check blocknumber evidence are uploaded within today and last 3 months 
                // Start time-> First 10 days, any juror can stake, and change to stake period
                // Add the blocknumber when positive externality score is added as (u8, blocknumber) tuple.


			
				
	}
}
