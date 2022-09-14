use crate::*;

impl<T: Config> Pallet<T> {
	/// Get a concatenation of previous members and runners-up and their deposits.
	///
	/// These accounts are essentially treated as candidates.
	pub(super) fn implicit_candidates_with_deposit(
		departmentid: u128,
	) -> Vec<(T::AccountId, BalanceOf<T>)> {
		// invariant: these two are always without duplicates.
		Self::members(departmentid)
			.into_iter()
			.map(|m| (m.who, m.deposit))
			.chain(Self::runners_up(departmentid).into_iter().map(|r| (r.who, r.deposit)))
			.collect::<Vec<_>>()
	}

	/// Check if `who` is a candidate. It returns the insert index if the element does not exists as
	/// an error.
	pub(super) fn is_candidate(who: &T::AccountId, departmentid: u128) -> Result<(), usize> {
		Self::candidates(departmentid).binary_search_by(|c| c.0.cmp(who)).map(|_| ())
	}

	/// Check if `who` is a voter. It may or may not be a _current_ one.
	pub(super) fn _is_voter(who: &T::AccountId, departmentid: u128) -> bool {
		Voting::<T>::contains_key(departmentid, who)
	}

	/// Check if `who` is currently an active member.
	pub(super) fn is_member(who: &T::AccountId, departmentid: u128) -> bool {
		Self::members(departmentid).binary_search_by(|m| m.who.cmp(who)).is_ok()
	}

	/// Check if `who` is currently an active runner-up.
	pub(super) fn is_runner_up(who: &T::AccountId, departmentid: u128) -> bool {
		Self::runners_up(departmentid).iter().any(|r| &r.who == who)
	}

	pub fn candidate_ids(departmentid: u128) -> Vec<T::AccountId> {
		Self::candidates(departmentid)
			.into_iter()
			.map(|(c, _)| c)
			.collect::<Vec<T::AccountId>>()
	}

	/// Get the members' account ids.
	pub fn members_ids(departmentid: u128) -> Vec<T::AccountId> {
		Self::members(departmentid)
			.into_iter()
			.map(|m| m.who)
			.collect::<Vec<T::AccountId>>()
	}

	pub fn runners_up_ids(departmentid: u128) -> Vec<T::AccountId> {
		Self::runners_up(departmentid)
			.into_iter()
			.map(|r| r.who)
			.collect::<Vec<T::AccountId>>()
	}

	

	pub(super) fn remove_and_replace_member(
		who: &T::AccountId,
		slash: bool,
		departmentid: u128,
	) -> Result<bool, DispatchError> {
		// closure will return:
		// - `Ok(Option(replacement))` if member was removed and replacement was replaced.
		// - `Ok(None)` if member was removed but no replacement was found
		// - `Err(_)` if who is not a member.
		<Members<T>>::try_mutate::<_, _, Error<T>, _>(departmentid, |members| {
			let remove_index = members
				.binary_search_by(|m| m.who.cmp(who))
				.map_err(|_| Error::<T>::NotMember)?;
			// we remove the member anyhow, regardless of having a runner-up or not.
			let removed = members.remove(remove_index);

			// slash or unreserve
			if slash {
				let (imbalance, _remainder) = T::Currency::slash_reserved(who, removed.deposit);
				debug_assert!(_remainder.is_zero());
				T::LoserCandidate::on_unbalanced(imbalance);
				Self::deposit_event(Event::SeatHolderSlashed {
					seat_holder: who.clone(),
					amount: removed.deposit,
				});
			} else {
				T::Currency::unreserve(who, removed.deposit);
			}

			let maybe_next_best =
				<RunnersUp<T>>::mutate(departmentid, |r| r.pop()).map(|next_best| {
					// defensive-only: Members and runners-up are disjoint. This will always be err and
					// give us an index to insert.
					if let Err(index) = members.binary_search_by(|m| m.who.cmp(&next_best.who)) {
						members.insert(index, next_best.clone());
					} else {
						// overlap. This can never happen. If so, it seems like our intended replacement
						// is already a member, so not much more to do.
						log::error!(
							target: "runtime::elections-phragmen",
							"A member seems to also be a runner-up.",
						);
					}
					next_best
				});
			Ok(maybe_next_best)
		})?;

		Ok(true)
	}
}
