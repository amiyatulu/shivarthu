use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn remove_tags(departmentid: DepartmentId, tag: Vec<u8>) -> DispatchResult {
		let mut tags = Tags::<T>::get(&departmentid);

		match tags.binary_search(&tag) {
			Ok(index) => {
				tags.remove(index);
				Tags::<T>::insert(&departmentid, tags);
				Self::deposit_event(Event::TagRemoved(departmentid, tag));
				Ok(())
			},
			Err(_) => Err(Error::<T>::TagDoesnotExists.into()),
		}
	}

	/// Ensure user has not downvoted. If downvoted add the AccountId
	pub(super) fn ensure_user_not_downvoted_tag(
		departmentid: DepartmentId,
		who: T::AccountId,
		tag: Vec<u8>,
	) -> DispatchResult {
		let mut usertags = <UserDownVote<T>>::get(&(departmentid, who.clone()));

		match usertags.binary_search(&tag) {
			Ok(_) => Err(Error::<T>::TagExists.into()),
			Err(index) => {
				usertags.insert(index, tag.clone());
				<UserDownVote<T>>::insert(&(departmentid, who), usertags);
				Ok(())
			},
		}
	}
}
