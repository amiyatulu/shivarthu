#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};

pub trait SharedStorageLink {
	
	type AccountId;

	fn check_citizen_is_approved_link(address: Self::AccountId) -> DispatchResult;

	fn get_approved_citizen_count_link() -> u64;
	fn set_positive_externality_link(address: Self::AccountId, score: i64)-> DispatchResult;
	
}
