#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
// One can enhance validation measures by increasing staking power for local residents or individuals with positive externalities—those who contribute to the network for a good cause.
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

use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
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
pub use types::DEPARTMENT_REQUIRED_FUND_ID;
use types::{
	DepartmentFundingStatus, DepartmentRequiredFund, FundingStatus, TippingName, TippingValue,
};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type SumTreeNameType<T> = SumTreeName<AccountIdOf<T>, BlockNumberOf<T>>;
type DepartmentId = u64;
type DepartmentRequiredFundId = u64;

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
	pub trait Config:
		frame_system::Config + schelling_game_shared::Config + pallet_timestamp::Config
	{
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
	pub fn DefaultForNextDepartmentRequiredFundId() -> DepartmentRequiredFundId {
		DEPARTMENT_REQUIRED_FUND_ID
	}

	#[pallet::storage]
	#[pallet::getter(fn next_department_required_fund_id)]
	pub type NextDepartmentRequiredFundId<T: Config> = StorageValue<
		_,
		DepartmentRequiredFundId,
		ValueQuery,
		DefaultForNextDepartmentRequiredFundId,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_department_required_funds)]
	pub type DepartmentRequiredFunds<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentRequiredFundId, DepartmentRequiredFund<T>>;

	#[pallet::storage]
	#[pallet::getter(fn validate_positive_externality)]
	pub type ValidateDepartmentRequiredFund<T: Config> =
		StorageMap<_, Twox64Concat, DepartmentRequiredFundId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validation_department_required_funds_block_number)]
	pub type ValidationDepartmentRequiredFundsBlock<T: Config> =
		StorageMap<_, Blake2_128Concat, DepartmentRequiredFundId, BlockNumberOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn department_funding_status)]
	pub type DepartmentFundingStatusForDepartmentId<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DepartmentId,
		DepartmentFundingStatus<BlockNumberOf<T>, FundingStatus>,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		DepartmentFundCreated {
			account: T::AccountId,
			department_required_fund_id: DepartmentRequiredFundId,
		},
		StakingPeriodStarted {
			department_required_fund_id: DepartmentRequiredFundId,
			block_number: BlockNumberOf<T>,
		},
		ApplyJurors {
			department_required_fund_id: DepartmentRequiredFundId,
			block_number: BlockNumberOf<T>,
			account: T::AccountId,
		},
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
		DepartmentRequiredFundDontExits,
		BlockDepartmentRequiredFundIdNotExists,
		ValidationForDepartmentRequiredFundIdIsOff,
		FundingStatusProcessing,
		ReapplicationTimeNotReached,
		ConditionDontMatch,
	}

	// Check deparment exists, it will done using loose coupling
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_department_required_fund(
			origin: OriginFor<T>,
			department_id: DepartmentId,
			tipping_name: TippingName,
			funding_needed: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let tipping_value = Self::value_of_tipping_name(tipping_name);
			let max_tipping_value = tipping_value.max_tipping_value;
			let stake_required = tipping_value.stake_required;
			let new_department_fund_id = Self::next_department_required_fund_id();
			let new_department_fund: DepartmentRequiredFund<T> = DepartmentRequiredFund::new(
				new_department_fund_id,
				department_id,
				tipping_name,
				funding_needed,
				who.clone(),
			);
			ensure!(funding_needed <= max_tipping_value, Error::<T>::FundingMoreThanTippingValue);
			// Check user has done kyc
			let _ = <T as pallet::Config>::Currency::withdraw(
				&who,
				stake_required,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;
			DepartmentRequiredFunds::insert(new_department_fund_id, new_department_fund);
			NextDepartmentRequiredFundId::<T>::mutate(|n| {
				*n += 1;
			});

			Self::deposit_event(Event::DepartmentFundCreated {
				account: who,
				department_required_fund_id: new_department_fund_id,
			});
			Ok(())
		}

		// Check update and discussion time over, only project creator can apply staking period
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn apply_staking_period(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::ensure_validation_to_do(department_required_fund_id)?;
			let department_id = Self::get_department_id_from_department_required_fund_id(
				department_required_fund_id,
			)?;
			let department_funding_status = Self::ensure_can_stake_using_status(department_id)?;
			DepartmentFundingStatusForDepartmentId::<T>::insert(
				department_id,
				department_funding_status,
			);
			let now = <frame_system::Pallet<T>>::block_number();
			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: now.clone(),
			};
			ValidationDepartmentRequiredFundsBlock::<T>::insert(
				department_required_fund_id,
				now.clone(),
			);
			T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now.clone())?;
			T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

			Self::deposit_event(Event::StakingPeriodStarted {
				department_required_fund_id,
				block_number: now,
			});

			Ok(())
		}

		// // Check update and discussion time over, only project creator can apply staking period
		// #[pallet::call_index(1)]
		// #[pallet::weight(0)]
		// pub fn apply_staking_period(origin: OriginFor<T>, department_required_fund_id: DepartmentRequiredFundId) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::ensure_user_is_project_creator_and_project_exists(project_id, who)?;
		// 	Self::ensure_staking_period_set_once_project_id(project_id)?;

		// 	let now = <frame_system::Pallet<T>>::block_number();

		// 	let key = SumTreeName::ProjectTips { project_id, block_number: now.clone() };

		// 	<ValidationProjectBlock<T>>::insert(project_id, now.clone());
		// 	// check what if called again, its done with `ensure_staking_period_set_once_project_id`
		// 	T::SchellingGameSharedSource::set_to_staking_period_pe_link(key.clone(), now.clone())?;
		// 	T::SchellingGameSharedSource::create_tree_helper_link(key, 3)?;

		// 	Self::deposit_event(Event::StakinPeriodStarted { project_id, block_number: now });

		// 	Ok(())
		// }

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn apply_jurors_project_tips(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
			stake: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;

			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			let phase_data = Self::get_phase_data();

			T::SchellingGameSharedSource::apply_jurors_helper_link(
				key,
				phase_data,
				who.clone(),
				stake,
			)?;
			Self::deposit_event(Event::ApplyJurors {
				department_required_fund_id,
				block_number,
				account: who,
			});

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn pass_period(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;

			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			let now = <frame_system::Pallet<T>>::block_number();
			let phase_data = Self::get_phase_data();
			T::SchellingGameSharedSource::change_period_link(key, phase_data, now)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn draw_jurors(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
			iterations: u64,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;

			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			let phase_data = Self::get_phase_data();

			T::SchellingGameSharedSource::draw_jurors_helper_link(key, phase_data, iterations)?;

			Ok(())
		}

		// Unstaking
		// Stop drawn juror to unstake ✔️
		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn unstaking(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;
			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			T::SchellingGameSharedSource::unstaking_helper_link(key, who)?;
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn commit_vote(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
			vote_commit: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;
			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			T::SchellingGameSharedSource::commit_vote_helper_link(key, who, vote_commit)?;
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn reveal_vote(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
			choice: u128,
			salt: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;
			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			T::SchellingGameSharedSource::reveal_vote_two_choice_helper_link(
				key, who, choice, salt,
			)?;
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(0)]
		pub fn get_incentives(
			origin: OriginFor<T>,
			department_required_fund_id: DepartmentRequiredFundId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let block_number =
				Self::get_block_number_of_schelling_game(department_required_fund_id)?;
			let key = SumTreeName::DepartmentRequiredFund {
				department_required_fund_id,
				block_number: block_number.clone(),
			};

			let phase_data = Self::get_phase_data();
			T::SchellingGameSharedSource::get_incentives_two_choice_helper_link(
				key, phase_data, who,
			)?;
			Ok(())
		}
	}
}
