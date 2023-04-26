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

    pub fn ensure_validation_on_positive_externality(account: T::AccountId) -> DispatchResult {

        let bool_data = ValidatePositiveExternality::<T>::get(account);
        ensure!(bool_data == true, Error::<T>::ValidationPositiveExternalityIsOff);

        Ok(())
    }

    pub fn ensure_min_stake_positive_externality(account: T::AccountId) -> DispatchResult {
        let stake = PositiveExternalityStakeBalance::<T>::get(account);
        let min_stake = MinimumPositiveExternalityStake::<T>::get();
        ensure!(stake >= min_stake, Error::<T>::LessThanMinStake);
        


        Ok(())
    }

    pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn u64_to_block_saturated(input: u64) -> BlockNumberOf<T> {
		input.saturated_into::<BlockNumberOf<T>>()
	}
	
}
