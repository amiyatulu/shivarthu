#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_std::{vec::Vec};

sp_api::decl_runtime_apis! {
	pub trait ShivarthuApi {
		fn hello_world() -> u128;
		fn get_challengers_evidence(profile_citizenid: u128, offset: u64, limit: u16) -> Vec<u128>;
	}
}