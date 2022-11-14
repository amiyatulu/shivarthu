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
			Err(_) => Err(Error::<T>::TagExists.into()),
		}
	}
}
