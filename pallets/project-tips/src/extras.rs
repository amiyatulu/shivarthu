use crate::*;


impl<T: Config> Pallet<T> {

	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}
	pub fn ensure_min_stake_deparment(department_id: DeparmentId) -> DispatchResult {
		let stake = DepartmentStakeBalance::<T>::get(department_id);
		let min_stake = MinimumDepartmentStake::<T>::get();
		// println!("stake {:?}", stake);
		// println!("min stake {:?}", min_stake);
		ensure!(stake >= min_stake, Error::<T>::LessThanMinStake);

		Ok(())
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
		input.saturated_into::<BlockNumberOf<T>>()
	}
}
