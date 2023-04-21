use frame_support::dispatch::DispatchResult;

use super::*;

impl<T: Config> PositiveExternalityPost<T> {
    pub fn new(
        id: PositiveExternalityPostId,
        created_by: T::AccountId,
        content: Content,
    ) -> Self {
        PositiveExternalityPost {
            id,
            created: new_who_and_when::<T>(created_by.clone()),
            edited: false,
            owner: created_by,
            content,
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

impl<T: Config> Pallet<T> {

    
	
}
