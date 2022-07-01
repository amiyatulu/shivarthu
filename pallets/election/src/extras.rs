use crate::*;

impl<T: Config> Pallet<T> {
	/// Get a concatenation of previous members and runners-up and their deposits.
	///
	/// These accounts are essentially treated as candidates.
	pub (super) fn implicit_candidates_with_deposit(departmentid: u128) -> Vec<(T::AccountId, BalanceOf<T>)> {
		// invariant: these two are always without duplicates.
		Self::members(departmentid)
			.into_iter()
			.map(|m| (m.who, m.deposit))
			.chain(Self::runners_up(departmentid).into_iter().map(|r| (r.who, r.deposit)))
			.collect::<Vec<_>>()
	}
}
