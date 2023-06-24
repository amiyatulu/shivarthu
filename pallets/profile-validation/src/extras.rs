use crate::*;

impl<T: Config> CitizenDetailsPost<T> {
	pub fn new(citizen_id: CitizenId, created_by: T::AccountId, content: Content) -> Self {
		CitizenDetailsPost {
			created: new_who_and_when::<T>(created_by.clone()),
			content,
			citizen_id,
			owner: created_by,
			edited: false,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		}
	}

	pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
		ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
		Ok(())
	}

	pub fn is_owner(&self, account: &T::AccountId) -> bool {
		self.owner == *account
	}
}

impl<T: Config> ChallengeEvidencePost<T> {
	pub fn new(
		kyc_profile_id: T::AccountId,
		created_by: T::AccountId,
		content: Content,
		post_id_if_comment: Option<ChallengePostId>,
	) -> Self {
		ChallengeEvidencePost {
			created: new_who_and_when::<T>(created_by.clone()),
			owner: created_by,
			kyc_profile_id,
			content,
			post_id_if_comment,
			is_comment: false,
		}
	}

	pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
		ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
		Ok(())
	}

	pub fn is_owner(&self, account: &T::AccountId) -> bool {
		self.owner == *account
	}
}

impl<T: Config> Pallet<T> {
	pub(super) fn get_phase_data() -> PhaseData<T> {
		T::SchellingGameSharedSource::create_phase_data(50, 5, 3, 100, (100, 100))
	}

	// pub(super) fn get_citizen_accountid(
	// 	citizenid: CitizenId,
	// ) -> Result<T::AccountId, DispatchError> {
	// 	let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
	// 	Ok(profile.owner)
	// }

	pub(super) fn fund_profile_account() -> T::AccountId {
		PALLET_ID.into_sub_account_truncating(1)
	}

	// pub fn get_challengers_evidence(
	// 	profile_citizenid: CitizenId,
	// 	offset: u64,
	// 	limit: u16,
	// ) -> Vec<ChallengePostId> {
	// 	let mut data = <ChallengerEvidenceId<T>>::iter_prefix_values(&profile_citizenid)
	// 		.skip(offset as usize)
	// 		.take(limit as usize)
	// 		.collect::<Vec<_>>();
	// 	data.sort();
	// 	data.reverse();
	// 	data
	// }

	// pub fn get_evidence_period_end_block(profile_citizenid: CitizenId) -> Option<u32> {
	// 	let now = <frame_system::Pallet<T>>::block_number();
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};
	// 	let phase_data = Self::get_phase_data();
	// 			match <ProfileFundDetails<T>>::get(&profile_citizenid) {
	// 		Some(_profilefundinfo) => {
	// 			// let start_block_number = profilefundinfo.start;
	// 			let result =
	// 				T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
	// 					key, phase_data, now,
	// 				);
	// 			result
	// 		},
	// 		None => None,
	// 	}
	// }

	// pub fn get_staking_period_end_block(profile_citizenid: CitizenId) -> Option<u32> {
	// 	let now = <frame_system::Pallet<T>>::block_number();
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};

	// 	let phase_data = Self::get_phase_data();

	// 	let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
	// 		key, phase_data, now,
	// 	);
	// 	result
	// }

	// pub fn get_drawing_period_end(profile_citizenid: CitizenId) -> (u64, u64, bool) {
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};
	// 	let phase_data = Self::get_phase_data();

	// 	let result =
	// 		T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
	// 	result
	// }

	// pub fn get_commit_period_end_block(profile_citizenid: CitizenId) -> Option<u32> {
	// 	let now = <frame_system::Pallet<T>>::block_number();
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};
	// 	let phase_data = Self::get_phase_data();

	// 	let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
	// 		key, phase_data, now,
	// 	);
	// 	result
	// }

	// pub fn get_vote_period_end_block(profile_citizenid: CitizenId) -> Option<u32> {
	// 	let now = <frame_system::Pallet<T>>::block_number();
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};
	// 	let phase_data = Self::get_phase_data();

	// 	let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
	// 		key, phase_data, now,
	// 	);
	// 	result
	// }

	// pub fn selected_as_juror(profile_citizenid: CitizenId, who: T::AccountId) -> bool {
	// 	let key = SumTreeName::UniqueIdenfier1 {
	// 		citizen_id: profile_citizenid,
	// 		name: "challengeprofile".as_bytes().to_vec(),
	// 	};

	// 	let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
	// 	result
	// }
}
