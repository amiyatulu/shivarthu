use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn permission_user_is_citizen(account_id: T::AccountId) -> DispatchResult {
        let approved_citizens = <ApprovedCitizenAddress<T>>::get();

        match approved_citizens.binary_search(&account_id) {
            Ok(_index) => {
                Ok(())
            }
            Err(_) => Err(Error::<T>::CitizenDoNotExists.into()),
        }

	}
}
