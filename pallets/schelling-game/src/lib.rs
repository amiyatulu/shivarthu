#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::types::{
		SchellingType, SortitionSumTree,
		SumTreeName
	};
	use frame_support::sp_runtime::traits::AccountIdConversion;
	use frame_support::sp_runtime::traits::{CheckedSub, CheckedAdd};
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::sp_std::{collections::btree_map::BTreeMap, vec::Vec};
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_support::{traits::Randomness};
	use frame_support::{
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	


	// Schelling Game Storage

	#[pallet::storage]
	#[pallet::getter(fn sortition_sum_trees)]
	pub type SortitionSumTrees<T> = StorageMap<_, Blake2_128Concat, SumTreeName, SortitionSumTree>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	
		StorageOverflow,
		KMustGreaterThanOne,
		TreeAlreadyExists,
		TreeDoesnotExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
	}

	impl<T: Config> Pallet<T> {
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
				}
			}
			Ok(())
		}

		pub fn set(
			key: SumTreeName,
			value: u64,
			citizen_id: u128,
		) -> DispatchResult {
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
									value.checked_sub(tree.nodes[tree_index as usize]).ok_or("StorageOverflow")?
								} else {
									(tree.nodes[tree_index as usize]).checked_sub(value).ok_or("StorageOverflow")?
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
					}

					None => {
						Self::if_tree_index_zero(value, citizen_id, tree, 0, key);
					}
				},
			}

			Ok(())
		}

		fn update_parents(
			mut tree: SortitionSumTree,
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
			citizen_id: u128,
			mut tree: SortitionSumTree,
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
						let parent_id = *tree.node_indexes_to_ids.get(&parent_index).unwrap();
						let new_index = tree_index + 1;
						tree.nodes.push(*tree.nodes.get(parent_index as usize).unwrap());
						tree.node_indexes_to_ids.remove(&parent_index);
						tree.ids_to_node_indexes.insert(parent_id, new_index);
						tree.node_indexes_to_ids.insert(new_index, parent_id);
					}
				} else {
					let tree_index = tree.stack.get(tree.stack.len() - 1);
					tree.nodes[*tree_index.unwrap() as usize] = value;
					tree.stack.pop();
				}

				tree.ids_to_node_indexes.insert(citizen_id, tree_index);
				tree.node_indexes_to_ids.insert(tree_index, citizen_id);

				// update_parents 🟥

				Self::update_parents(tree, tree_index, true, value, key);
			}
		}

		pub fn stake_of(key: SumTreeName, citizen_id: u128) -> Result<u64, DispatchError> {
			let tree_option = <SortitionSumTrees<T>>::get(&key);
			match tree_option {
				None => Err(Error::<T>::TreeDoesnotExist)?,
				Some(tree) => {
					let tree_index_data = tree.ids_to_node_indexes.get(&citizen_id).unwrap();

					let value: u64;
					let tree_index = *tree_index_data;
					if tree_index == 0 {
						value = 0;
					} else {
						value = tree.nodes[tree_index as usize];
					}
					Ok(value)
				}
			}
		}

		pub fn draw(key: SumTreeName, draw_number: u64) -> Result<u128, DispatchError> {
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

					Ok(*tree.node_indexes_to_ids.get(&tree_index).unwrap())
				}
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
		pub fn query_leafs(key: SumTreeName, cursor: u64, count: u64) -> Result<(u64, Vec<u64>, bool), DispatchError>  {
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
					for j in loop_start_index..tree.nodes.len() as u64{
						if values_index < count {
							values.push(tree.nodes[j as usize]);
							values_index = values_index + 1;
						} else {
							has_more  = true;
							break;
						}

					}

					Ok((start_index, values, has_more))
					
				}
			}

		}

	}
}
