use crate::*;

impl<T: Config> Pallet<T> {
    pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}
}
