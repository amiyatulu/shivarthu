use crate::*;

impl<T: Config> Pallet<T> {
	       /// Get `Space` by id from the storage or return `SpaceNotFound` error.
           pub fn require_space(space_id: SpaceId) -> Result<Space<T>, DispatchError> {
            Ok(Self::space_by_id(space_id).ok_or(Error::<T>::SpaceNotFound)?)
        }
}
