#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};

pub trait SortitionSumGameLink {
	type SumTreeName;
	type AccountId;
	fn create_tree_link(key: Self::SumTreeName, k: u64) -> DispatchResult;
	fn set_link(key: Self::SumTreeName, value: u64, citizen_id: Self::AccountId) -> DispatchResult;
	fn stake_of_link(
		key: Self::SumTreeName,
		citizen_id: Self::AccountId,
	) -> Result<Option<u64>, DispatchError>;
	fn draw_link(key: Self::SumTreeName, draw_number: u64) -> Result<Self::AccountId, DispatchError>;
	fn remove_tree_link(key: Self::SumTreeName) -> DispatchResult;
}
