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
mod types;

use frame_support::sp_runtime::traits::Saturating;
use frame_support::sp_runtime::SaturatedConversion;
use frame_support::sp_std::prelude::*;
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
};
use frame_support::{
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
	PalletId,
};
use pallet_support::{
	ensure_content_is_valid, new_who_and_when, remove_from_vec, Content, PositiveExternalityPostId,
	WhoAndWhen, WhoAndWhenOf,
};
use schelling_game_shared::types::{Period, PhaseData, RangePoint, SchellingGameType};
use schelling_game_shared_link::SchellingGameSharedLink;
use shared_storage_link::SharedStorageLink;
use sortition_sum_game::types::SumTreeName;
use types::{Project, TippingName, TippingValue};
pub use types::PROJECT_ID;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;
type DepartmentId = u64;
type ProjectId = u64;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + schelling_game_shared::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		type SharedStorageSource: SharedStorageLink<AccountId = AccountIdOf<Self>>;
		type SchellingGameSharedSource: SchellingGameSharedLink<
			SumTreeName = SumTreeName<Self::AccountId, Self::BlockNumber>,
			SchellingGameType = SchellingGameType,
			BlockNumber = Self::BlockNumber,
			AccountId = AccountIdOf<Self>,
			Balance = BalanceOf<Self>,
			RangePoint = RangePoint,
			Period = Period,
			PhaseData = PhaseData<Self>,
		>;
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn MinimumDepartmentStake<T: Config>() -> BalanceOf<T> {
		10000u128.saturated_into::<BalanceOf<T>>()
	}

	#[pallet::type_value]
	pub fn DefaultForNextProjectId() -> ProjectId {
		PROJECT_ID
	}

	#[pallet::storage]
	#[pallet::getter(fn next_project_id)]
	pub type NextProjectId<T: Config> =
		StorageValue<_, ProjectId, ValueQuery, DefaultForNextProjectId>;

	#[pallet::storage]
	#[pallet::getter(fn department_stake)]
	pub type DepartmentStakeBalance<T: Config> =
		StorageMap<_, Twox64Concat, DepartmentId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validation_department_block_number)]
	pub type ValidationDepartmentBlock<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentId, BlockNumberOf<T>, ValueQuery>;

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
		LessThanMinStake,
		CannotStakeNow,
		ChoiceOutOfRange,
		FundingMoreThanTippingValue,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_project_stake(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			tipping_name: TippingName,
			funding_needed: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let tipping_value = Self::value_of_tipping_name(tipping_name);
			let max_tipping_value = tipping_value.max_tipping_value;
			let stake_required = tipping_value.stake_required;
			let project_id = Self::next_project_id();

			let new_project: Project<T> = Project::new(project_id, department_id, tipping_name, funding_needed, who.clone());

			ensure!(funding_needed <= max_tipping_value, Error::<T>::FundingMoreThanTippingValue);
			// Check user has done kyc
			let _ = <T as pallet::Config>::Currency::withdraw(
				&who,
				stake_required,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;

			// let stake = DepartmentStakeBalance::<T>::get(department_id);
			// let total_balance = stake.saturating_add(deposit);
			// DepartmentStakeBalance::<T>::insert(department_id, total_balance);

			// emit event
			Ok(())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn set_validate_positive_externality(
		// 	origin: OriginFor<T>,
		// 	value: bool,
		// ) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		// 	// Check user has done kyc

		// 	ValidatePositiveExternality::<T>::insert(&who, value);
		// 	// emit event
		// 	Ok(())
		// }

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn apply_staking_period(
			origin: OriginFor<T>,
			department_id: DepartmentId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Self::ensure_validation_on_positive_externality(user_to_calculate.clone())?;
			Self::ensure_min_stake_deparment(department_id)?;

			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);
			let now = <frame_system::Pallet<T>>::block_number();
			let six_month_number = (6 * 30 * 24 * 60 * 60) / 6;
			let six_month_block = Self::u64_to_block_saturated(six_month_number);
			let modulus = now % six_month_block;
			let storage_main_block = now - modulus;
			// println!("{:?}", now);
			// println!("{:?}", three_month_number);
			// println!("{:?}", storage_main_block);
			// println!("{:?}", pe_block_number);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: storage_main_block.clone(),
			};

			// let game_type = SchellingGameType::PositiveExternality;

			if storage_main_block > pe_block_number {
				<ValidationDepartmentBlock<T>>::insert(department_id, storage_main_block);
				// check what if called again
				T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now)?;
				T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

			//  println!("{:?}", data);
			} else {
				return Err(Error::<T>::CannotStakeNow.into());
			}

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn apply_jurors_positive_externality(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			stake: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Self::ensure_validation_on_positive_externality(user_to_calculate.clone())?;
			Self::ensure_min_stake_deparment(department_id)?;

			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			let phase_data = Self::get_phase_data();

			T::SchellingGameSharedSource::apply_jurors_helper_link(key, phase_data, who, stake)?;

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn pass_period(origin: OriginFor<T>, department_id: DepartmentId) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			let now = <frame_system::Pallet<T>>::block_number();
			let phase_data = Self::get_phase_data();
			T::SchellingGameSharedSource::change_period_link(key, phase_data, now)?;

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn draw_jurors_positive_externality(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			iterations: u64,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			let phase_data = Self::get_phase_data();

			T::SchellingGameSharedSource::draw_jurors_helper_link(key, phase_data, iterations)?;

			Ok(())
		}

		// Unstaking
		// Stop drawn juror to unstake ✔️
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn unstaking(origin: OriginFor<T>, department_id: DepartmentId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			T::SchellingGameSharedSource::unstaking_helper_link(key, who)?;
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn commit_vote(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			vote_commit: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			T::SchellingGameSharedSource::commit_vote_for_score_helper_link(key, who, vote_commit)?;
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn reveal_vote(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			choice: i64,
			salt: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(choice <= 5 && choice >= 1, Error::<T>::ChoiceOutOfRange);

			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);

			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			T::SchellingGameSharedSource::reveal_vote_score_helper_link(key, who, choice, salt)?;
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(0)]
		pub fn get_incentives(origin: OriginFor<T>, department_id: DepartmentId) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let pe_block_number = <ValidationDepartmentBlock<T>>::get(department_id);
			let key = SumTreeName::DepartmentScore {
				department_id,
				block_number: pe_block_number.clone(),
			};

			let phase_data = Self::get_phase_data();
			T::SchellingGameSharedSource::get_incentives_score_schelling_helper_link(
				key.clone(),
				phase_data,
				RangePoint::ZeroToFive,
			)?;

			let score = T::SchellingGameSharedSource::get_mean_value_link(key.clone());
			// // println!("Score {:?}", score);

			// To do
			// T::SharedStorageSource::set_positive_externality_link(user_to_calculate, score)?;

			Ok(())
		}
	}
}
