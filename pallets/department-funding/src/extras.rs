use crate::*;

impl<T: Config> DepartmentRequiredFund<T> {
	pub fn new(
		department_required_fund_id: DepartmentRequiredFundId,
		department_id: DepartmentId,
		tipping_name: TippingName,
		funding_needed: BalanceOf<T>,
		creator: T::AccountId,
	) -> Self {
		DepartmentRequiredFund {
			created: new_who_and_when::<T>(creator.clone()),
			department_required_fund_id,
			department_id,
			tipping_name,
			funding_needed,
			creator,
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}

	pub fn ensure_validation_on_positive_externality(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> DispatchResult {
		let bool_data = ValidateDepartmentRequiredFund::<T>::get(department_required_fund_id);
		ensure!(bool_data == true, Error::<T>::ValidationForDepartmentRequiredFundIdIsOff);

		Ok(())
	}

	pub fn get_department_id_from_department_required_fund_id(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Result<DepartmentId, DispatchError> {
		Ok(1)
	}

	// pub fn ensure_user_is_project_creator_and_project_exists(
	// 	project_id: ProjectId,
	// 	user: T::AccountId,
	// ) -> DispatchResult {
	// 	let project_option: Option<Project<T>> = Projects::get(project_id);
	// 	match project_option {
	// 		Some(project) => {
	// 			let creator = project.creator;
	// 			ensure!(creator == user, Error::<T>::ProjectCreatorDontMatch);
	// 		},
	// 		None => Err(Error::<T>::ProjectDontExists)?,
	// 	}

	// 	Ok(())
	// }

	// pub fn ensure_staking_period_set_once_project_id(project_id: ProjectId) -> DispatchResult {
	// 	let block_number_option = <ValidationProjectBlock<T>>::get(project_id);
	// 	match block_number_option {
	// 		Some(_block) => Err(Error::<T>::ProjectIdStakingPeriodAlreadySet)?,
	// 		None => Ok(()),
	// 	}
	// }

	pub fn get_block_number_of_schelling_game(
		department_required_fund_id: DepartmentRequiredFundId,
	) -> Result<BlockNumberOf<T>, DispatchError> {
		let block_number_option =
			<ValidationDepartmentRequiredFundsBlock<T>>::get(department_required_fund_id);
		let block_number = match block_number_option {
			Some(block_number) => block_number,
			None => Err(Error::<T>::BlockDepartmentRequiredFundIdNotExists)?,
		};
		Ok(block_number)
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
		input.saturated_into::<BlockNumberOf<T>>()
	}

	pub(super) fn value_of_tipping_name(tipping: TippingName) -> TippingValue<BalanceOf<T>> {
		match tipping {
			TippingName::SmallTipper => TippingValue {
				max_tipping_value: 10_000u64.saturated_into::<BalanceOf<T>>(),
				stake_required: 10u64.saturated_into::<BalanceOf<T>>(),
			},
			TippingName::BigTipper => TippingValue {
				max_tipping_value: 100_000u64.saturated_into::<BalanceOf<T>>(),
				stake_required: 50u64.saturated_into::<BalanceOf<T>>(),
			},
			TippingName::SmallSpender => TippingValue {
				max_tipping_value: 1_000_000u64.saturated_into::<BalanceOf<T>>(),
				stake_required: 100u64.saturated_into::<BalanceOf<T>>(),
			},
			TippingName::MediumSpender => TippingValue {
				max_tipping_value: 10_000_000u64.saturated_into::<BalanceOf<T>>(),
				stake_required: 200u64.saturated_into::<BalanceOf<T>>(),
			},
			TippingName::BigSpender => TippingValue {
				max_tipping_value: 100_000_000u64.saturated_into::<BalanceOf<T>>(),
				stake_required: 500u64.saturated_into::<BalanceOf<T>>(),
			},
		}
	}
}
