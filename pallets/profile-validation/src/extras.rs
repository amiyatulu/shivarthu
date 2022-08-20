use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn get_citizen_accountid(citizenid: u128) -> Result<T::AccountId, DispatchError> {
		let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
		Ok(profile.accountid)
	}

	pub(super) fn fund_profile_account() -> T::AccountId {
		PALLET_ID.into_sub_account(1)
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
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let game_type = SchellingGameType::ProfileApproval;
		match <ProfileFundDetails<T>>::get(&profile_citizenid) {
			Some(_profilefundinfo) => {
				// let start_block_number = profilefundinfo.start;
				let result =
					T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
						key, game_type, now,
					);
				result
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

		let game_type = SchellingGameType::ProfileApproval;

		let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
			key, game_type, now,
		);
		result
	}

	pub fn get_drawing_period_end(profile_citizenid: u128) -> (u64, u64, bool) {
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let game_type = SchellingGameType::ProfileApproval;

		let result =
			T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, game_type);
		result
	}

	pub fn get_commit_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let game_type = SchellingGameType::ProfileApproval;

		let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
			key, game_type, now,
		);
		result
	}

	pub fn get_vote_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let game_type = SchellingGameType::ProfileApproval;

		let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
			key, game_type, now,
		);
		result
	}

	pub fn selected_as_juror(profile_citizenid: u128, who: T::AccountId) -> bool {
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};

		let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
		result
	}
}
