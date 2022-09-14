use crate::*;

impl<T: Config> Pallet<T> {
	pub fn hello_world() -> u128 {
		10
	}

	pub fn get_challengers_evidence(profile_citizenid: u128, offset: u64, limit: u16) -> Vec<u128> {
		let mut data = <ChallengerEvidenceId<T>>::iter_prefix_values(&profile_citizenid)
			.skip(offset as usize)
			.take(limit as usize)
			.collect::<Vec<_>>();
		data.sort();
		data.reverse();
		data
	}

	pub fn get_evidence_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		match <ProfileFundDetails<T>>::get(&profile_citizenid) {
			Some(profilefundinfo) => {
				let block_number = profilefundinfo.start;
				let block_time = <MinBlockTime<T>>::get();
				let end_block =
					block_number.checked_add(&block_time.min_short_block_length).expect("Overflow");
				let left_block = end_block.checked_sub(&now);
				match left_block {
					Some(val) => {
						let left_block_u32 = Self::block_number_to_u32_saturated(val);
						Some(left_block_u32)
					},
					None => Some(0),
				}
			},
			None => None,
		}
	}

	pub fn get_staking_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let staking_start_time = <StakingStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block = staking_start_time
			.checked_add(&block_time.min_long_block_length)
			.expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub fn get_drawing_period_end(profile_citizenid: u128) -> (u64, u64, bool) {
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let draw_limit = <DrawJurorsForProfileLimitData<T>>::get();
		let draws_in_round = <DrawsInRound<T>>::get(&key);
		if draws_in_round >= draw_limit.max_draws.into() {
			(draw_limit.max_draws, draws_in_round, true)
		} else {
			(draw_limit.max_draws, draws_in_round, false)
		}
	}

	pub fn get_commit_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let commit_start_time = <CommitStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block = commit_start_time
			.checked_add(&block_time.min_long_block_length)
			.expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub fn get_vote_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let vote_start_time = <VoteStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block = vote_start_time
			.checked_add(&block_time.min_long_block_length)
			.expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub fn selected_as_juror(profile_citizenid: u128, who: T::AccountId) -> bool {
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let drawn_juror = <DrawnJurors<T>>::get(&key);
		match drawn_juror.binary_search(&who.clone()) {
			Ok(_) => true,
			Err(_) => false,
		}
	}
	// pub(super) fn super_hello_world() -> u128 {
	// 	20
	// }

	pub(super) fn get_citizen_accountid(citizenid: u128) -> Result<T::AccountId, DispatchError> {
		let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
		Ok(profile.accountid)
	}

	pub(super) fn _get_citizen_id(accountid: T::AccountId) -> Result<u128, DispatchError> {
		match Self::citizen_id(accountid) {
			Some(citizen_id) => Ok(citizen_id),
			None => Err(Error::<T>::ProfileNotFunded)?,
		}
	}

	pub(super) fn _profile_fund_added(citizenid: u128) -> DispatchResult {
		match <ProfileFundDetails<T>>::get(&citizenid) {
			Some(profilefundinfo) => {
				let validated = profilefundinfo.validated;
				let reapply = profilefundinfo.reapply;
				if validated == false && reapply == false {
					Ok(())
				} else {
					Err(Error::<T>::ProfileValidationOver)?
				}
			},
			None => Err(Error::<T>::ProfileNotFunded)?,
		}
	}

	fn _get_profile_fund_info(citizenid: u128) -> Result<ProfileFundInfoOf<T>, DispatchError> {
		match <ProfileFundDetails<T>>::get(&citizenid) {
			Some(profilefundinfo) => {
				let validated = profilefundinfo.validated;
				let reapply = profilefundinfo.reapply;
				if validated == false && reapply == false {
					Ok(profilefundinfo)
				} else {
					Err(Error::<T>::ProfileValidationOver)?
				}
			},
			None => Err(Error::<T>::ProfileNotFunded)?,
		}
	}

	pub(super) fn balance_to_u64_saturated(input: BalanceOf<T>) -> u64 {
		input.saturated_into::<u64>()
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn block_number_to_u32_saturated(input: BlockNumberOf<T>) -> u32 {
		input.saturated_into::<u32>()
	}

	pub(super) fn fund_profile_account() -> T::AccountId {
		PALLET_ID.into_sub_account(1)
	}

	// fn juror_stake_account() -> T::AccountId {
	//     PALLET_ID.into_sub_account(2)
	// }

	// fn draw_juror_for_citizen_profile_function(citizen_id: u128, length: usize) -> DispatchResult {
	// 	let nonce = Self::get_and_increment_nonce();

	// 	let random_seed = T::RandomnessSource::random(&nonce).encode();
	// 	let random_number = u64::decode(&mut random_seed.as_ref())
	// 		.expect("secure hashes should always be bigger than u64; qed");
	// 	Ok(())
	// }

	pub(super) fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = <Nonce<T>>::get();
		<Nonce<T>>::put(nonce.wrapping_add(1));
		let n = nonce * 1000 + 1000; // remove and uncomment in production
		n.encode()

		// nonce.encode()
	}

	pub(super) fn get_winning_decision(decision_tuple: (u64, u64)) -> u8 {
		if decision_tuple.1 > decision_tuple.0 {
			1
		} else if decision_tuple.0 > decision_tuple.1 {
			0
		} else {
			2
		}
	}

	pub(super) fn get_winning_incentives(
		decision_tuple: (u64, u64),
		incentive_tuple: (u64, u64),
	) -> (u8, u64) {
		let winning_decision = Self::get_winning_decision(decision_tuple);
		if winning_decision == 0 {
			let winning_incentives =
				(incentive_tuple.1).checked_div(decision_tuple.0).expect("Overflow");
			(winning_decision, winning_incentives)
		} else if winning_decision == 1 {
			let winning_incentives =
				(incentive_tuple.1).checked_div(decision_tuple.1).expect("Overflow");
			(winning_decision, winning_incentives)
		} else {
			(winning_decision, 0)
		}
	}


}
