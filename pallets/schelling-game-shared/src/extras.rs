use crate::*;

impl<T: Config> Pallet<T> {
	// Set to evidence period, when some one stakes for validation
	pub(super) fn set_to_evidence_period(key: SumTreeName) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(_period) => Err(Error::<T>::PeriodExists)?,
			None => {
				let period = Period::Evidence;
				<PeriodName<T>>::insert(&key, period);
			},
		}
		Ok(())
	}

	// Check Period is Evidence, and change it to staking
	// Check evidence period is over (from the time when stake for evidence was sumitted )
	pub(super) fn set_to_staking_period(
		key: SumTreeName,
		game_type: SchellingGameType,
		evidence_stake_block_number: BlockNumberOf<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		if let Some(Period::Evidence) = <PeriodName<T>>::get(&key) {
			let time = now.checked_sub(&evidence_stake_block_number).expect("Overflow");
			let block_time = <MinBlockTime<T>>::get(&game_type);
			if time >= block_time.min_short_block_length {
				let new_period = Period::Staking;
				<PeriodName<T>>::insert(&key, new_period);
				<StakingStartTime<T>>::insert(&key, now);
			}
		}
		
		Ok(())
	}
}
