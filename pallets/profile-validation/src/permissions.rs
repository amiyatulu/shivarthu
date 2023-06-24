use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn ensure_account_id_has_profile(account_id: T::AccountId) -> DispatchResult {
		match <GetCitizenId<T>>::get(&account_id) {
			Some(_) => Ok(()),
			None => Err(Error::<T>::CitizenDoNotExists)?,
		}
	}
}
