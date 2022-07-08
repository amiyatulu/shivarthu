#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_std::{prelude::*};
use sp_api::codec::Codec;

sp_api::decl_runtime_apis! {
	pub trait ElectionApi<AccountId> where AccountId: Codec{
		fn candidate_ids(departmentid: u128) -> Vec<AccountId>;
		fn members_ids(departmentid: u128) -> Vec<AccountId>;
		fn runners_up_ids(departmentid: u128) -> Vec<AccountId>;
	}
}