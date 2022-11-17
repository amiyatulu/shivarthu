use crate::*;

impl<T: Config> Pallet<T> {
	/// Remove tags and remove DownVoteTags
	pub(super) fn remove_tags(departmentid: DepartmentId, tag: Vec<u8>) -> DispatchResult {
		let mut tags = Tags::<T>::get(&departmentid);

		match tags.binary_search(&tag) {
			Ok(index) => {
				tags.remove(index);
				Tags::<T>::insert(&departmentid, tags);
				DownVoteDetailsTags::<T>::remove(&departmentid, &tag);
				Self::deposit_event(Event::TagRemoved(departmentid, tag));
				Ok(())
			},
			Err(_) => Err(Error::<T>::TagDoesnotExists.into()),
		}
	}

	// Ensure tag exists
	pub(super) fn ensure_tag_exists(departmentid: DepartmentId, tag: Vec<u8>) -> DispatchResult {
		let tags = Tags::<T>::get(&departmentid);

		match tags.binary_search(&tag) {
			Ok(_) => Ok(()),
			Err(_) => Err(Error::<T>::TagDoesnotExists.into()),
		}
	}

	/// Ensure user has not downvoted. If downvoted add the AccountId
	pub(super) fn ensure_user_not_downvoted_then_downvote(
		departmentid: DepartmentId,
		who: T::AccountId,
		tag: Vec<u8>,
	) -> Result<DownVoteNum, DispatchError> {
		let mut down_vote_details = DownVoteDetailsTags::<T>::get(&departmentid, &tag);
		let mut users_that_downvoted = down_vote_details.downvote_users;
		let dv = down_vote_details.downvote.checked_add(1).ok_or("overflow")?;


		match users_that_downvoted.binary_search(&who) {
			Ok(_) => Err(Error::<T>::UserAlreadyDownVoted.into()),
			Err(index) => {
				users_that_downvoted.insert(index, who);
				down_vote_details.downvote_users = users_that_downvoted;
				down_vote_details.downvote = dv;
				DownVoteDetailsTags::<T>::insert(&departmentid, &tag, down_vote_details);
				Ok(dv)
			},
		}
	}
}
