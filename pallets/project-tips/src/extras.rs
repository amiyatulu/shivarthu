use crate::*;

impl<T: Config> Project<T> {
	pub fn new(
		project_id: ProjectId,
		department_id: DepartmentId,
		tipping_name: TippingName,
		funding_needed: BalanceOf<T>,
		project_leader: T::AccountId,
	) -> Self {
		Project {
			created: new_who_and_when::<T>(project_leader.clone()),
			project_id,
			department_id,
			tipping_name,
			funding_needed,
			project_leader,
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}

	pub fn ensure_user_is_project_creator_and_project_exists(
		project_id: ProjectId,
		user: T::AccountId,
	) -> DispatchResult {
		let project_option: Option<Project<T>> = Projects::get(project_id);
		match project_option {
			Some(project) => {
				let project_leader = project.project_leader;
				ensure!(project_leader == user, Error::<T>::ProjectCreatorDontMatch);
			},
			None => Err(Error::<T>::ProjectDontExists)?,
		}

		Ok(())
	}

	pub fn ensure_staking_period_set_once_project_id(project_id: ProjectId) -> DispatchResult {
		let block_number_option = <ValidationProjectBlock<T>>::get(project_id);
		match block_number_option {
			Some(_block) => Err(Error::<T>::ProjectIdStakingPeriodAlreadySet)?,
			None => Ok(()),
		}
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
