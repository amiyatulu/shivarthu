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

use frame_support::sp_std::prelude::*;
use frame_support::{
	traits::{
		Currency, OnUnbalanced, ReservableCurrency,
	},
};
use frame_support::sp_runtime::SaturatedConversion;

use profile_validation_link::ProfileValidationLink;
// use scale_info::prelude::format;
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
	use frame_support::pallet_prelude::{DispatchResult, *};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ProfileValidationSource: ProfileValidationLink<AccountId = AccountIdOf<Self>>;
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
	#[pallet::getter(fn citizen_got_ubi_block_number)]
	pub type CitizenLastUbiBlock<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberOf<T>>;

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
		/// Fund fixed UBI
		/// Fund positive externality of more positive externality score
		/// Give tranferable staking coins
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn fun_ubi(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			T::ProfileValidationSource::account_is_validated_link(who.clone())?;
			let number_of_validated_accounts = T::ProfileValidationSource::get_approved_citizen_count_link();
			let ubi_block_number = <CitizenLastUbiBlock<T>>::get(who.clone());
			let now = <frame_system::Pallet<T>>::block_number();
			let total_issuance = T::Currency::total_issuance();
			let balance_hundred = Self::u64_to_balance_saturated(100);
			let one_percentage_issuance = total_issuance/balance_hundred;
			let balance_twelve = Self::u64_to_balance_saturated(12);
			let total_ubi_per_month = one_percentage_issuance/balance_twelve; 
			let balance_number_of_validated_accounts = Self::u64_to_balance_saturated(number_of_validated_accounts);
			let ubi_person = total_ubi_per_month/ balance_number_of_validated_accounts;
			let r = T::Currency::deposit_into_existing(&who, ubi_person).ok().unwrap();
			T::Reward::on_unbalanced(r);
			// println!("test {:?}", total_issuance);
			// println!("10 percentage {:?}", one_percentage_issuance);
			// println!("Length {:}", number_of_validated_accounts);
			Ok(())
		}
	}
}
