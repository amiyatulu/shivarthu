#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};

pub trait ProfileValidationLink {
	
	type AccountId;

	fn account_is_validated_link(address: Self::AccountId) -> DispatchResult;
	
}
