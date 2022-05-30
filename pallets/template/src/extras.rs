use crate::*;

impl<T: Config> Pallet<T> {
	pub fn hello_world() -> u128 {
		10
	}

	pub fn get_challengers_evidence(profile_citizenid: u128, offset: u64, limit: u16) -> Vec<u128> {
		let mut data = <ChallengerEvidenceId<T>>::iter_prefix_values(&profile_citizenid)
			.skip(offset as usize)
			.take(limit as usize)
			.collect::<Vec<_>>();
		data.sort();
		data.reverse();
		data
	}

	pub fn get_evidence_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		match <ProfileFundDetails<T>>::get(&profile_citizenid) {
			Some(profilefundinfo) => {
				let block_number = profilefundinfo.start;
				let block_time = <MinBlockTime<T>>::get();
				let end_block =
					block_number.checked_add(&block_time.min_challenge_time).expect("Overflow");
				let left_block = end_block.checked_sub(&now).expect("Overflow");
				let left_block_u32 = Self::block_number_to_u32_saturated(left_block);
				Some(left_block_u32)
			},
			None => None,
		}
	}

	pub fn get_staking_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let staking_start_time = <StakingStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block =
			staking_start_time.checked_add(&block_time.min_block_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now).expect("Overflow");
		let left_block_u32 = Self::block_number_to_u32_saturated(left_block);
		Some(left_block_u32)
	}

	pub fn get_drawing_period_end(profile_citizenid: u128) -> (u64, u64, bool) {
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let draw_limit = <DrawJurorsForProfileLimitData<T>>::get();
		let draws_in_round = <DrawsInRound<T>>::get(&key);
		if draws_in_round >= draw_limit.max_draws.into() {
			(draw_limit.max_draws, draws_in_round, true)
		} else {
			(draw_limit.max_draws, draws_in_round, false)
		}
	}

	pub fn get_commit_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let commit_start_time = <CommitStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block =
			commit_start_time.checked_add(&block_time.min_block_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now).expect("Overflow");
		let left_block_u32 = Self::block_number_to_u32_saturated(left_block);
		Some(left_block_u32)
	}

	pub fn get_vote_period_end_block(profile_citizenid: u128) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: profile_citizenid,
			name: "challengeprofile".as_bytes().to_vec(),
		};
		let vote_start_time = <VoteStartTime<T>>::get(&key);
		let block_time = <MinBlockTime<T>>::get();
		let end_block =
			vote_start_time.checked_add(&block_time.min_block_length).expect("Overflow");
		let left_block = end_block.checked_sub(&now).expect("Overflow");
		let left_block_u32 = Self::block_number_to_u32_saturated(left_block);
		Some(left_block_u32)
	}
	pub(super) fn super_hello_world() -> u128 {
		20
	}

	pub(super) fn get_citizen_accountid(citizenid: u128) -> Result<T::AccountId, DispatchError> {
		let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
		Ok(profile.accountid)
	}

	pub(super) fn get_citizen_id(accountid: T::AccountId) -> Result<u128, DispatchError> {
		match Self::citizen_id(accountid) {
			Some(citizen_id) => Ok(citizen_id),
			None => Err(Error::<T>::ProfileNotFunded)?,
		}
	}

	pub(super) fn profile_fund_added(citizenid: u128) -> DispatchResult {
		match <ProfileFundDetails<T>>::get(&citizenid) {
			Some(profilefundinfo) => {
				let validated = profilefundinfo.validated;
				let reapply = profilefundinfo.reapply;
				if validated == false && reapply == false {
					Ok(())
				} else {
					Err(Error::<T>::ProfileValidationOver)?
				}
			},
			None => Err(Error::<T>::ProfileNotFunded)?,
		}
	}

	fn get_profile_fund_info(citizenid: u128) -> Result<ProfileFundInfoOf<T>, DispatchError> {
		match <ProfileFundDetails<T>>::get(&citizenid) {
			Some(profilefundinfo) => {
				let validated = profilefundinfo.validated;
				let reapply = profilefundinfo.reapply;
				if validated == false && reapply == false {
					Ok(profilefundinfo)
				} else {
					Err(Error::<T>::ProfileValidationOver)?
				}
			},
			None => Err(Error::<T>::ProfileNotFunded)?,
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

	pub(super) fn fund_profile_account() -> T::AccountId {
		PALLET_ID.into_sub_account(1)
	}

	// fn juror_stake_account() -> T::AccountId {
	//     PALLET_ID.into_sub_account(2)
	// }

	fn draw_juror_for_citizen_profile_function(citizen_id: u128, length: usize) -> DispatchResult {
		let nonce = Self::get_and_increment_nonce();

		let random_seed = T::RandomnessSource::random(&nonce).encode();
		let random_number = u64::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u64; qed");
		Ok(())
	}

	pub(super) fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = <Nonce<T>>::get();
		<Nonce<T>>::put(nonce.wrapping_add(1));
		let n = nonce * 1000 + 1000; // remove and uncomment in production
		n.encode()

		// nonce.encode()
	}

	pub(super) fn get_winning_decision(decision_tuple: (u64, u64)) -> u8 {
		if decision_tuple.1 > decision_tuple.0 {
			1
		} else if decision_tuple.0 > decision_tuple.1 {
			0
		} else {
			2
		}
	}

	pub(super) fn get_winning_incentives(
		decision_tuple: (u64, u64),
		incentive_tuple: (u64, u64),
	) -> (u8, u64) {
		let winning_decision = Self::get_winning_decision(decision_tuple);
		if winning_decision == 0 {
			let winning_incentives =
				(incentive_tuple.1).checked_div(decision_tuple.0).expect("Overflow");
			(winning_decision, winning_incentives)
		} else if winning_decision == 1 {
			let winning_incentives =
				(incentive_tuple.1).checked_div(decision_tuple.1).expect("Overflow");
			(winning_decision, winning_incentives)
		} else {
			(winning_decision, 0)
		}
	}

	// SortitionSumTree
	pub fn create_tree(key: SumTreeName, k: u64) -> DispatchResult {
		if k < 1 {
			Err(Error::<T>::KMustGreaterThanOne)?
		}
		let tree_option = <SortitionSumTrees<T>>::get(&key);
		match tree_option {
			Some(_tree) => Err(Error::<T>::TreeAlreadyExists)?,
			None => {
				let mut sum_tree = SortitionSumTree {
					k,
					stack: Vec::new(),
					nodes: Vec::new(),
					ids_to_node_indexes: BTreeMap::new(),
					node_indexes_to_ids: BTreeMap::new(),
				};

				sum_tree.nodes.push(0);

				<SortitionSumTrees<T>>::insert(&key, &sum_tree);
			},
		}
		Ok(())
	}

	pub fn set(key: SumTreeName, value: u64, citizen_id: AccountIdOf<T>) -> DispatchResult {
		let tree_option = <SortitionSumTrees<T>>::get(&key);

		match tree_option {
			None => Err(Error::<T>::TreeDoesnotExist)?,
			Some(mut tree) => match tree.ids_to_node_indexes.get(&citizen_id) {
				Some(tree_index_data) => {
					let tree_index = *tree_index_data;
					if tree_index == 0 {
						Self::if_tree_index_zero(value, citizen_id, tree, tree_index, key);
					} else {
						// Existing node
						if value == 0 {
							let value = tree.nodes[tree_index as usize];
							tree.nodes[tree_index as usize] = 0;
							tree.stack.push(tree_index);
							tree.ids_to_node_indexes.remove(&citizen_id);
							tree.node_indexes_to_ids.remove(&tree_index);

							// UpdateParents 🟥
							Self::update_parents(tree, tree_index, false, value, key);
						} else if value != tree.nodes[tree_index as usize] {
							let plus_or_minus = tree.nodes[tree_index as usize] <= value;
							let plus_or_minus_value = if plus_or_minus {
								value
									.checked_sub(tree.nodes[tree_index as usize])
									.ok_or("StorageOverflow")?
							} else {
								(tree.nodes[tree_index as usize])
									.checked_sub(value)
									.ok_or("StorageOverflow")?
							};
							tree.nodes[tree_index as usize] = value;

							// update parents 🟥
							Self::update_parents(
								tree,
								tree_index,
								plus_or_minus,
								plus_or_minus_value,
								key,
							);
						}
					}
				},

				None => {
					Self::if_tree_index_zero(value, citizen_id, tree, 0, key);
				},
			},
		}

		Ok(())
	}

	fn update_parents(
		mut tree: SortitionSumTree<AccountIdOf<T>>,
		tree_index: u64,
		plus_or_minus: bool,
		value: u64,
		key: SumTreeName,
	) {
		let mut parent_index = tree_index;
		while parent_index != 0 {
			parent_index = (parent_index - 1) / tree.k;
			tree.nodes[parent_index as usize] = if plus_or_minus {
				(tree.nodes[parent_index as usize]).checked_add(value).expect("StorageOverflow")
			} else {
				(tree.nodes[parent_index as usize]).checked_sub(value).expect("StorageOverflow")
			};

			<SortitionSumTrees<T>>::insert(&key, &tree);
		}
	}
	fn if_tree_index_zero(
		value: u64,
		citizen_id: AccountIdOf<T>,
		mut tree: SortitionSumTree<AccountIdOf<T>>,
		mut tree_index: u64,
		key: SumTreeName,
	) {
		// No existing node.
		if value != 0 {
			// Non zero value.
			// Append.
			// Add node.
			if tree.stack.len() == 0 {
				// No vacant spots.
				// Get the index and append the value.
				tree_index = tree.nodes.len() as u64;
				tree.nodes.push(value);

				// println!("{}", tree_index);

				// Potentially append a new node and make the parent a sum node.
				if tree_index != 1 && (tree_index - 1) % tree.k == 0 {
					// Is first child.
					let parent_index = tree_index / tree.k;
					let parent_id = tree.node_indexes_to_ids.get(&parent_index).unwrap().clone();
					let new_index = tree_index + 1;
					tree.nodes.push(*tree.nodes.get(parent_index as usize).unwrap());
					tree.node_indexes_to_ids.remove(&parent_index);
					tree.ids_to_node_indexes.insert(parent_id.clone(), new_index);
					tree.node_indexes_to_ids.insert(new_index, parent_id);
				}
			} else {
				let tree_index = tree.stack.get(tree.stack.len() - 1);
				tree.nodes[*tree_index.unwrap() as usize] = value;
				tree.stack.pop();
			}

			tree.ids_to_node_indexes.insert(citizen_id.clone(), tree_index);
			tree.node_indexes_to_ids.insert(tree_index, citizen_id);

			// update_parents 🟥

			Self::update_parents(tree, tree_index, true, value, key);
		}
	}

	pub fn stake_of(
		key: SumTreeName,
		citizen_id: AccountIdOf<T>,
	) -> Result<Option<u64>, DispatchError> {
		let tree_option = <SortitionSumTrees<T>>::get(&key);
		match tree_option {
			None => Err(Error::<T>::TreeDoesnotExist)?,
			Some(tree) => {
				let tree_index_data;
				match tree.ids_to_node_indexes.get(&citizen_id) {
					Some(v) => tree_index_data = v,
					None => return Ok(None),
				}

				let value: u64;
				let tree_index = *tree_index_data;
				if tree_index == 0 {
					value = 0;
				} else {
					value = tree.nodes[tree_index as usize];
				}
				Ok(Some(value))
			},
		}
	}

	pub fn draw(key: SumTreeName, draw_number: u64) -> Result<AccountIdOf<T>, DispatchError> {
		let tree_option = <SortitionSumTrees<T>>::get(&key);

		match tree_option {
			None => Err(Error::<T>::TreeDoesnotExist)?,
			Some(tree) => {
				let mut tree_index = 0;
				let mut current_draw_number = draw_number % tree.nodes[0];

				while (tree.k * tree_index) + 1 < (tree.nodes.len() as u64) {
					for i in 1..tree.k + 1 {
						let node_index = (tree.k * tree_index) + i;
						let node_value = tree.nodes[node_index as usize];

						if current_draw_number >= node_value {
							current_draw_number -= node_value;
						} else {
							tree_index = node_index;
							break;
						}
					}
				}
				let account_id = tree.node_indexes_to_ids.get(&tree_index).unwrap().clone();
				Ok(account_id)
			},
		}
	}

	/**
	 *  @dev Query the leaves of a tree. Note that if `startIndex == 0`, the tree is empty and the root node will be returned.
	 *  @param key The key of the tree to get the leaves from.
	 *  @param cursor The pagination cursor.
	 *  @param count The number of items to return.
	 *  @return The index at which leaves start, the values of the returned leaves, and whether there are more for pagination.
	 *  `O(n)` where
	 *  `n` is the maximum number of nodes ever appended.
	 */
	pub fn query_leafs(
		key: SumTreeName,
		cursor: u64,
		count: u64,
	) -> Result<(u64, Vec<u64>, bool), DispatchError> {
		let tree_option = <SortitionSumTrees<T>>::get(&key);

		match tree_option {
			None => Err(Error::<T>::TreeDoesnotExist)?,
			Some(tree) => {
				let mut start_index = 0;
				for i in 0..tree.nodes.len() {
					if (tree.k * i as u64) + 1 >= tree.nodes.len() as u64 {
						start_index = i as u64;
						break;
					}
				}
				let loop_start_index = start_index + cursor;

				// let value = if loop_start_index + count > tree.nodes.len() as u64 {
				// 	tree.nodes.len() as u64 - loop_start_index
				// } else {
				// 	count
				// };

				let mut values = Vec::new();
				let mut values_index = 0;
				let mut has_more = false;
				for j in loop_start_index..tree.nodes.len() as u64 {
					if values_index < count {
						values.push(tree.nodes[j as usize]);
						values_index = values_index + 1;
					} else {
						has_more = true;
						break;
					}
				}

				Ok((start_index, values, has_more))
			},
		}
	}
}
