use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn create_phase_with_all_data(
		evidence_length: u64,
		end_of_staking_time: u64,
		staking_length: u64,
		drawing_length: u64,
		commit_length: u64,
		vote_length: u64,
		appeal_length: u64,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: u64,
		juror_incentives: (u64, u64),
	) -> PhaseDataOf<T> {
		PhaseData::create_phase_with_all_data(
			evidence_length,
			end_of_staking_time,
			staking_length,
			drawing_length,
			commit_length,
			vote_length,
			appeal_length,
			max_draws,
			min_number_juror_staked,
			min_juror_stake,
			juror_incentives,
		)
	}
	pub(super) fn create_phase_data(
		block_length: u64,
		max_draws: u64,
		min_number_juror_staked: u64,
		min_juror_stake: u64,
		juror_incentives: (u64, u64),
	) -> PhaseDataOf<T> {
		PhaseData::create_with_data(
			block_length,
			max_draws,
			min_number_juror_staked,
			min_juror_stake,
			juror_incentives,
		)
	}
	/// Set to evidence period, when some one stakes for validation
	pub(super) fn set_to_evidence_period(
		key: SumTreeNameType<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(_period) => Err(Error::<T>::PeriodExists)?,
			None => {
				let period = Period::Evidence;
				<PeriodName<T>>::insert(&key, period);
				<EvidenceStartTime<T>>::insert(&key, now);
			},
		}
		Ok(())
	}

	/// Check `Period` is `Evidence`, and change it to `Staking`   
	/// It is called with function that submits challenge stake after `end_block` of evidence period  
	/// Checks evidence period is over
	#[doc=include_str!("docimage/set_to_staking_period_1.svg")]
	/// ```ignore
	/// if time >= block_time.min_short_block_length {
	///        // change `Period` to `Staking`
	///  }
	/// ```
	pub(super) fn set_to_staking_period(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		if let Some(Period::Evidence) = <PeriodName<T>>::get(&key) {
			let evidence_stake_block_number = <EvidenceStartTime<T>>::get(&key);
			let time = now.checked_sub(&evidence_stake_block_number).expect("Overflow");
			let evidence_length = phase_data.evidence_length;
			let end_length_for_staking = phase_data.end_of_staking_time;
			let total_length =
				evidence_length.checked_add(&end_length_for_staking).expect("overflow");
			if time >= evidence_length && time < total_length {
				let new_period = Period::Staking;
				<PeriodName<T>>::insert(&key, new_period);
				<StakingStartTime<T>>::insert(&key, now);
			} else if time >= total_length {
				Err(Error::<T>::TimeForStakingOver)?
			} else {
				Err(Error::<T>::EvidencePeriodNotOver)?
			}
		} else {
			Err(Error::<T>::PeriodIsNotEvidence)?
		}

		Ok(())
	}

	/// Check time for staking over
	pub(super) fn ensure_time_for_staking_over(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		let evidence_stake_block_number = <EvidenceStartTime<T>>::get(&key);
		let time = now.checked_sub(&evidence_stake_block_number).expect("Overflow");
		let evidence_length = phase_data.evidence_length;
		let end_length_for_staking = phase_data.end_of_staking_time;
		let total_length = evidence_length.checked_add(&end_length_for_staking).expect("overflow");
		ensure!(time >= total_length, Error::<T>::TimeForStakingNotOver);
		Ok(())
	}

	/// Set staking period when evidence period is not required
	pub(super) fn set_to_staking_period_pe(
		key: SumTreeNameType<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		if let None = <PeriodName<T>>::get(&key) {
			let new_period = Period::Staking;
			<PeriodName<T>>::insert(&key, new_period);
			<StakingStartTime<T>>::insert(&key, now);
		} else {
			Err(Error::<T>::PeriodIsNotNone)?
		}

		Ok(())
	}

	pub(super) fn create_tree_link_helper(key: SumTreeNameType<T>, k: u64) -> DispatchResult {
		T::SortitionSumGameSource::create_tree_link(key.clone(), k)?;
		Ok(())
	}

	/// Change the `Period`
	///    
	/// `Period::Staking` to `Period::Drawing`
	#[doc=include_str!("docimage/change_period_link_1.svg")]
	/// ```ignore
	/// if now >= min_long_block_length + staking_start_time {
	///   // Change `Period::Staking` to `Period::Drawing`   
	/// }
	/// ```
	///
	///  `Period::Drawing` to `Period::Commit`   
	/// When maximum juror are drawn   
	///  
	/// `Period::Commit` to `Period::Vote`       
	/// ```ignore
	/// if now >= min_long_block_length + commit_start_time {
	///   // Change `Period::Commit` to `Period::Vote`  
	/// }
	/// ```
	///
	/// `Period::Vote` to `Period::Execution`   
	/// ```ignore
	/// if now >= min_long_block_length + vote_start_time {
	///   // Change `Period::Vote` to `Period::Execution`   
	/// }
	/// ```
	pub(super) fn change_period(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				match period {
					Period::Evidence => todo!(),
					Period::Staking => {
						// Also check has min number of jurors has staked
						let staking_start_time = <StakingStartTime<T>>::get(&key);
						let staking_length = phase_data.staking_length;
						if now >= staking_length + staking_start_time {
							let new_period = Period::Drawing;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::StakingPeriodNotOver)?
						}
					},
					Period::Drawing => {
						// Also give time
						let max_draws = phase_data.max_draws;
						let draws_in_round = <DrawsInRound<T>>::get(&key);
						if draws_in_round >= max_draws {
							<CommitStartTime<T>>::insert(&key, now);
							let new_period = Period::Commit;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::MaxJurorNotDrawn)?
						}
					},
					Period::Commit => {
						let commit_start_time = <CommitStartTime<T>>::get(&key);
						let commit_length = phase_data.commit_length;
						if now >= commit_length + commit_start_time {
							<VoteStartTime<T>>::insert(&key, now);
							let new_period = Period::Vote;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::CommitPeriodNotOver)?
						}
					},
					Period::Vote => {
						let vote_start_time = <VoteStartTime<T>>::get(&key);
						let vote_length = phase_data.vote_length;
						if now >= vote_length + vote_start_time {
							let new_period = Period::Execution;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::VotePeriodNotOver)?
						}
					},
					Period::Appeal => todo!(),
					Period::Execution => todo!(),
				}
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}
		Ok(())
	}

	pub(super) fn apply_jurors_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		who: AccountIdOf<T>,
		stake: BalanceOf<T>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Staking, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}
		let min_stake = phase_data.min_juror_stake;

		ensure!(stake >= min_stake, Error::<T>::JurorStakeLessThanMin);

		// let imb = T::Currency::withdraw(
		// 	&who,
		// 	stake,
		// 	WithdrawReasons::TRANSFER,
		// 	ExistenceRequirement::AllowDeath,
		// )?;

		// T::Currency::resolve_creating(&Self::juror_stake_account(), imb);

		let imbalance = T::Currency::slash(&who, stake).0;
		T::Slash::on_unbalanced(imbalance);

		// let stake_of = Self::stake_of(key.clone(), profile_citizenid)?;

		let stake_u64 = Self::balance_to_u64_saturated(stake);

		let stake_of = T::SortitionSumGameSource::stake_of_link(key.clone(), who.clone())?;

		match stake_of {
			Some(_stake) => Err(Error::<T>::AlreadyStaked)?,
			None => {
				let result = T::SortitionSumGameSource::set_link(key, stake_u64, who);
				result
			},
		}
	}

	// Improvements: Set stake to zero after a juror is drawn, so that they are not drawn again. Store the stake in storage map in DrawnJurors, and use it in get_incentives_helper
	pub(super) fn draw_jurors_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		iterations: u64,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Drawing, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}
		let max_draws = phase_data.max_draws;
		let draws_in_round = <DrawsInRound<T>>::get(&key);
		ensure!(draws_in_round < max_draws.into(), Error::<T>::MaxDrawExceeded);
		let mut end_index = draws_in_round + iterations;
		if draws_in_round + iterations >= max_draws {
			end_index = max_draws;
		}
		let mut draw_increment = draws_in_round.clone();

		for _ in draws_in_round..end_index {
			let nonce = Self::get_and_increment_nonce();
			let random_seed = T::RandomnessSource::random(&nonce).encode();
			let random_number = u64::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u64; qed");
			// let mut rng = rand::thread_rng();
			// let random_number: u64 = rng.gen();
			// log::info!("Random number: {:?}", random_number);
			let accountid = T::SortitionSumGameSource::draw_link(key.clone(), random_number)?;
			let stake = T::SortitionSumGameSource::stake_of_link(key.clone(), accountid.clone())?;

			let mut drawn_juror = <DrawnJurors<T>>::get(&key);
			match drawn_juror.binary_search_by(|(c, _)| c.cmp(&accountid)) {
				Ok(_) => {},
				Err(index) => {
					drawn_juror.insert(index, (accountid.clone(), stake.unwrap()));
					<DrawnJurors<T>>::insert(&key, drawn_juror);
					draw_increment = draw_increment + 1;
					// println!("draw_increment, {:?}", draw_increment);
					T::SortitionSumGameSource::set_link(key.clone(), 0, accountid)?;
				},
			}
			<DrawsInRound<T>>::insert(&key, draw_increment);
		}
		Ok(())
	}

	// When DrawnJurors contains stake, use drawn_juror.binary_search_by(|(c, _)| c.cmp(&who));
	pub(super) fn unstaking_helper(key: SumTreeNameType<T>, who: AccountIdOf<T>) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(
					period == Period::Commit
						|| period == Period::Vote
						|| period == Period::Execution,
					Error::<T>::PeriodDontMatch
				);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}

		let drawn_juror = <DrawnJurors<T>>::get(&key);
		match drawn_juror.binary_search_by(|(c, _)| c.cmp(&who.clone())) {
			Ok(_) => Err(Error::<T>::SelectedAsJuror)?,
			Err(_) => {},
		}

		let stake_of = T::SortitionSumGameSource::stake_of_link(key.clone(), who.clone())?;

		match stake_of {
			Some(stake) => {
				let balance = Self::u64_to_balance_saturated(stake);
				let mut unstaked_jurors = <UnstakedJurors<T>>::get(&key);
				match unstaked_jurors.binary_search(&who) {
					Ok(_) => Err(Error::<T>::AlreadyUnstaked)?,
					Err(index) => {
						unstaked_jurors.insert(index, who.clone());
						<UnstakedJurors<T>>::insert(&key, unstaked_jurors);
						// let _ = T::Currency::resolve_into_existing(
						// 	&who,
						// 	T::Currency::withdraw(
						// 		&Self::juror_stake_account(),
						// 		balance,
						// 		WithdrawReasons::TRANSFER,
						// 		ExistenceRequirement::AllowDeath,
						// 	)?,
						// );
						let r = T::Currency::deposit_into_existing(&who, balance).ok().unwrap();
						T::Reward::on_unbalanced(r);
					},
				}
			},
			None => Err(Error::<T>::StakeDoesNotExists)?,
		}

		// println!("stakeof {:?}", stake_of);

		Ok(())
	}

	pub(super) fn commit_vote_helper(
		key: SumTreeNameType<T>,
		who: AccountIdOf<T>,
		vote_commit: [u8; 32],
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Commit, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}
		let drawn_jurors = <DrawnJurors<T>>::get(&key);
		match drawn_jurors.binary_search_by(|(c, _)| c.cmp(&who.clone())) {
			Ok(_) => {
				let vote_commit_struct = CommitVote {
					commit: vote_commit,
					votestatus: VoteStatus::Commited,
					revealed_vote: None,
				};
				<VoteCommits<T>>::insert(&key, &who, vote_commit_struct);
			},
			Err(_) => Err(Error::<T>::JurorDoesNotExists)?,
		}
		Ok(())
	}

	pub(super) fn reveal_vote_two_choice_helper(
		key: SumTreeNameType<T>,
		who: AccountIdOf<T>,
		choice: u128,
		salt: Vec<u8>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Vote, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}
		let who_commit_vote = <VoteCommits<T>>::get(&key, &who);
		match who_commit_vote {
			Some(mut commit_struct) => {
				ensure!(
					commit_struct.votestatus == VoteStatus::Commited,
					Error::<T>::VoteStatusNotCommited
				);
				let mut vote = format!("{}", choice).as_bytes().to_vec();
				// let mut vote = choice.clone();
				let mut salt_a = salt.clone();
				vote.append(&mut salt_a);
				let vote_bytes: &[u8] = &vote;
				let hash = sp_io::hashing::keccak_256(vote_bytes);
				let commit: &[u8] = &commit_struct.commit;
				if hash == commit {
					let mut decision_tuple = <DecisionCount<T>>::get(&key);
					if choice == 1 {
						decision_tuple.1 = decision_tuple.1 + 1;
						<DecisionCount<T>>::insert(&key, decision_tuple);
						commit_struct.revealed_vote = Some(RevealedVote::Yes);
					} else if choice == 0 {
						decision_tuple.0 = decision_tuple.0 + 1;
						<DecisionCount<T>>::insert(&key, decision_tuple);
						commit_struct.revealed_vote = Some(RevealedVote::No);
					} else {
						Err(Error::<T>::NotValidChoice)?
					}
					commit_struct.votestatus = VoteStatus::Revealed;
					<VoteCommits<T>>::insert(&key, &who, commit_struct);
				} else {
					Err(Error::<T>::CommitDoesNotMatch)?
				}
			},
			None => Err(Error::<T>::CommitDoesNotExists)?,
		}

		Ok(())
	}

	/// Distribute incentives in a single go.
	pub(super) fn get_all_incentives_two_choice_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Execution, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}

		let drawn_jurors = <DrawnJurors<T>>::get(&key);
		let reveal_votes_iterator = <VoteCommits<T>>::iter_prefix(&key);

		let mut reveal_votes = reveal_votes_iterator
			.map(|(account_id, commit_vote)| (account_id, commit_vote.revealed_vote))
			.collect::<Vec<(_, _)>>();
		reveal_votes.sort_by(|a, b| a.0.cmp(&b.0));
		let decision_count = <DecisionCount<T>>::get(&key);
		let incentives = phase_data.juror_incentives;
		let (winning_decision, winning_incentives) =
			Self::get_winning_incentives(decision_count, incentives);
		for juror in drawn_jurors {
			match reveal_votes.binary_search_by(|(c, _)| c.cmp(&juror.0)) {
				Ok(index) => {
					let account_n_vote = reveal_votes[index].clone();
					if let Some(vote) = account_n_vote.1 {
						match winning_decision {
							WinningDecision::WinnerYes => match vote {
								RevealedVote::Yes => {
									let result = Self::winner_getting_incentives2(
										juror.0.clone(),
										winning_incentives,
										juror.1,
									)?;
									result
								},
								RevealedVote::No => {
									let result =
										Self::looser_getting_incentives2(juror.0.clone(), juror.1)?;
									result
								},
							},
							WinningDecision::WinnerNo => match vote {
								RevealedVote::Yes => {
									let result =
										Self::looser_getting_incentives2(juror.0.clone(), juror.1)?;
									result
								},
								RevealedVote::No => {
									let result = Self::winner_getting_incentives2(
										juror.0.clone(),
										winning_incentives,
										juror.1,
									)?;
									result
								},
							},
							WinningDecision::Draw => {
								let result =
									Self::getting_incentives_draw2(juror.0.clone(), juror.1)?;
								result
							},
						}
					}
				},
				Err(_) => {},
			}
		}
		// Remove SorititionSumTrees in `sortition-sum-game` pallet
		let _result = T::SortitionSumGameSource::remove_tree_link(key.clone());

		// Remove DrawnJurors
		<DrawnJurors<T>>::remove(&key);

		// Remove VoteCommits
		<VoteCommits<T>>::remove_prefix(key.clone(), None); // Deprecated: Use clear_prefix instead
													// let reveal_votes_iterator2 = <VoteCommits<T>>::iter_prefix(&key);
													// reveal_votes_iterator2.for_each(|(account_id, _)|{
													// 	<VoteCommits<T>>::remove(key.clone(), account_id);
													// });

		Ok(())
	}

	// Improvements: Will it be better to distribute all jurors incentives in single call
	pub(super) fn get_incentives_two_choice_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		who: AccountIdOf<T>,
	) -> DispatchResult {
		match <PeriodName<T>>::get(&key) {
			Some(period) => {
				ensure!(period == Period::Execution, Error::<T>::PeriodDontMatch);
			},
			None => Err(Error::<T>::PeriodDoesNotExists)?,
		}

		let drawn_juror = <DrawnJurors<T>>::get(&key);

		let who_commit_vote = <VoteCommits<T>>::get(&key, &who);
		match who_commit_vote {
			Some(commit_struct) => {
				let vote_option = commit_struct.revealed_vote;
				match vote_option {
					Some(vote) => {
						let decision_count: (u64, u64) = <DecisionCount<T>>::get(&key);
						let incentives = phase_data.juror_incentives;
						let (winning_decision, winning_incentives) =
							Self::get_winning_incentives(decision_count, incentives);
						if let Ok(i) = drawn_juror.binary_search_by(|(c, _)| c.cmp(&who.clone())) {
							let stake = drawn_juror[i].1;
							match winning_decision {
								WinningDecision::WinnerYes => match vote {
									RevealedVote::Yes => {
										let result = Self::winner_getting_incentives(
											key.clone(),
											who.clone(),
											winning_incentives,
											stake,
										)?;
										result
									},
									RevealedVote::No => {
										let result = Self::looser_getting_incentives(
											key.clone(),
											who.clone(),
											stake,
										)?;
										result
									},
								},
								WinningDecision::WinnerNo => match vote {
									RevealedVote::Yes => {
										let result = Self::looser_getting_incentives(
											key.clone(),
											who.clone(),
											stake,
										)?;
										result
									},
									RevealedVote::No => {
										let result = Self::winner_getting_incentives(
											key.clone(),
											who.clone(),
											winning_incentives,
											stake,
										)?;
										result
									},
								},
								WinningDecision::Draw => {
									let result = Self::getting_incentives_draw(
										key.clone(),
										who.clone(),
										stake.clone(),
									)?;
									result
								},
							}
						} else {
							Err(Error::<T>::StakeDoesNotExists)?
						}
					},
					None => Err(Error::<T>::VoteNotRevealed)?,
				}
			},
			None => Err(Error::<T>::CommitDoesNotExists)?,
		}
		Ok(())
	}

	pub(super) fn getting_incentives_draw(
		key: SumTreeNameType<T>,
		who: AccountIdOf<T>,
		stake: u64,
	) -> DispatchResult {
		let balance = Self::u64_to_balance_saturated(stake);
		let mut juror_got_incentives = <JurorsIncentiveDistributedAccounts<T>>::get(&key);
		match juror_got_incentives.binary_search(&who) {
			Ok(_) => Err(Error::<T>::AlreadyGotIncentives)?,
			Err(index) => {
				juror_got_incentives.insert(index, who.clone());
				<JurorsIncentiveDistributedAccounts<T>>::insert(&key, juror_got_incentives);
				let r = T::Currency::deposit_into_existing(&who, balance).ok().unwrap();
				T::Reward::on_unbalanced(r);
			},
		}

		Ok(())
	}

	pub(super) fn getting_incentives_draw2(who: AccountIdOf<T>, stake: u64) -> DispatchResult {
		let balance = Self::u64_to_balance_saturated(stake);

		let r = T::Currency::deposit_into_existing(&who, balance).ok().unwrap();
		T::Reward::on_unbalanced(r);

		Ok(())
	}

	pub(super) fn looser_getting_incentives(
		key: SumTreeNameType<T>,
		who: AccountIdOf<T>,
		stake: u64,
	) -> DispatchResult {
		let balance = Self::u64_to_balance_saturated(stake * 3 / 4);
		let mut juror_got_incentives = <JurorsIncentiveDistributedAccounts<T>>::get(&key);
		match juror_got_incentives.binary_search(&who) {
			Ok(_) => Err(Error::<T>::AlreadyGotIncentives)?,
			Err(index) => {
				juror_got_incentives.insert(index, who.clone());
				<JurorsIncentiveDistributedAccounts<T>>::insert(&key, juror_got_incentives);
				let r = T::Currency::deposit_into_existing(&who, balance).ok().unwrap();
				T::Reward::on_unbalanced(r);
			},
		}
		Ok(())
	}

	pub(super) fn looser_getting_incentives2(who: AccountIdOf<T>, stake: u64) -> DispatchResult {
		let balance = Self::u64_to_balance_saturated(stake * 3 / 4);

		let r = T::Currency::deposit_into_existing(&who, balance).ok().unwrap();
		T::Reward::on_unbalanced(r);

		Ok(())
	}

	pub(super) fn winner_getting_incentives(
		key: SumTreeNameType<T>,
		who: AccountIdOf<T>,
		winning_incentives: u64,
		stake: u64,
	) -> DispatchResult {
		let mut juror_got_incentives = <JurorsIncentiveDistributedAccounts<T>>::get(&key);
		match juror_got_incentives.binary_search(&who) {
			Ok(_) => Err(Error::<T>::AlreadyGotIncentives)?,
			Err(index) => {
				juror_got_incentives.insert(index, who.clone());
				<JurorsIncentiveDistributedAccounts<T>>::insert(&key, juror_got_incentives);
				let total_incentives = stake.checked_add(winning_incentives).expect("overflow");
				let incentives = Self::u64_to_balance_saturated(total_incentives);
				let r = T::Currency::deposit_into_existing(&who, incentives).ok().unwrap();
				T::Reward::on_unbalanced(r);
			},
		};

		Ok(())
	}

	pub(super) fn winner_getting_incentives2(
		who: AccountIdOf<T>,
		winning_incentives: u64,
		stake: u64,
	) -> DispatchResult {
		let total_incentives = stake.checked_add(winning_incentives).expect("overflow");
		let incentives = Self::u64_to_balance_saturated(total_incentives);
		let r = T::Currency::deposit_into_existing(&who, incentives).ok().unwrap();
		T::Reward::on_unbalanced(r);

		Ok(())
	}

	pub(super) fn get_winning_decision(decision_tuple: (u64, u64)) -> WinningDecision {
		if decision_tuple.1 > decision_tuple.0 {
			WinningDecision::WinnerYes // Decision 1 won
		} else if decision_tuple.0 > decision_tuple.1 {
			WinningDecision::WinnerNo // Decision 0 won
		} else {
			WinningDecision::Draw // draw
		}
	}

	pub(super) fn get_winning_decision_value(key: SumTreeNameType<T>) -> WinningDecision {
		let decision_tuple: (u64, u64) = <DecisionCount<T>>::get(&key);
		Self::get_winning_decision(decision_tuple)
	}

	pub(super) fn get_winning_incentives(
		decision_tuple: (u64, u64),
		incentive_tuple: (u64, u64),
	) -> (WinningDecision, u64) {
		let winning_decision = Self::get_winning_decision(decision_tuple);
		match winning_decision {
			WinningDecision::WinnerYes => {
				let winning_incentives =
					(incentive_tuple.1).checked_div(decision_tuple.1).expect("Overflow");
				(WinningDecision::WinnerYes, winning_incentives)
			},
			WinningDecision::WinnerNo => {
				let winning_incentives =
					(incentive_tuple.1).checked_div(decision_tuple.0).expect("Overflow");
				(WinningDecision::WinnerNo, winning_incentives)
			},
			WinningDecision::Draw => (WinningDecision::Draw, 0),
		}
	}

	pub(super) fn balance_to_u64_saturated(input: BalanceOf<T>) -> u64 {
		input.saturated_into::<u64>()
	}

	pub(super) fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
		input.saturated_into::<BalanceOf<T>>()
	}

	pub(super) fn block_number_to_u32_saturated(input: BlockNumberOf<T>) -> u32 {
		input.saturated_into::<u32>()
	}
	pub(super) fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = <Nonce<T>>::get();
		<Nonce<T>>::put(nonce.wrapping_add(1));
		// let n = nonce * 1000 + 1000; // remove and uncomment in production
		// n.encode()

		nonce.encode()
	}
	pub(super) fn get_evidence_period_end_block_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> Option<u32> {
		let start_block_number = <EvidenceStartTime<T>>::get(&key);
		let evidence_length = phase_data.evidence_length;
		let end_block = start_block_number.checked_add(&evidence_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub(super) fn get_staking_period_end_block_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> Option<u32> {
		let staking_start_time = <StakingStartTime<T>>::get(&key);
		let staking_length = phase_data.staking_length;
		let end_block = staking_start_time.checked_add(&staking_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub(super) fn get_drawing_period_end_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
	) -> (u64, u64, bool) {
		let max_draws = phase_data.max_draws;
		let draws_in_round = <DrawsInRound<T>>::get(&key);
		if draws_in_round >= max_draws.into() {
			(max_draws, draws_in_round, true)
		} else {
			(max_draws, draws_in_round, false)
		}
	}

	pub(super) fn get_commit_period_end_block_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> Option<u32> {
		let commit_start_time = <CommitStartTime<T>>::get(&key);
		let commit_length = phase_data.commit_length;
		let end_block = commit_start_time.checked_add(&commit_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub(super) fn get_vote_period_end_block_helper(
		key: SumTreeNameType<T>,
		phase_data: PhaseDataOf<T>,
		now: BlockNumberOf<T>,
	) -> Option<u32> {
		let vote_start_time = <VoteStartTime<T>>::get(&key);
		let vote_length = phase_data.vote_length;
		let end_block = vote_start_time.checked_add(&vote_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now);
		match left_block {
			Some(val) => {
				let left_block_u32 = Self::block_number_to_u32_saturated(val);
				Some(left_block_u32)
			},
			None => Some(0),
		}
	}

	pub(super) fn selected_as_juror_helper(key: SumTreeNameType<T>, who: T::AccountId) -> bool {
		let drawn_juror = <DrawnJurors<T>>::get(&key);
		match drawn_juror.binary_search_by(|(c, _)| c.cmp(&who.clone())) {
			Ok(_) => true,
			Err(_) => false,
		}
	}
}
