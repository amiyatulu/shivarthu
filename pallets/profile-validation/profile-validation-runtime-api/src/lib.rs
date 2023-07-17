#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::sp_std::{vec::Vec};
//  or
use frame_support::sp_std::{prelude::*};
use sp_api::codec::Codec;
type ChallengePostId = u64;

sp_api::decl_runtime_apis! {
	pub trait ProfileValidationApi<AccountId> where AccountId: Codec {
		fn get_challengers_evidence(profile_user_account: AccountId, offset: u64, limit: u16) -> Vec<ChallengePostId>;
		fn get_staking_period_end_block(profile_user_account: AccountId) -> Option<u32>;
		fn get_drawing_period_end(profile_user_account: AccountId) -> (u64, u64, bool);
		fn get_commit_period_end_block(profile_user_account: AccountId) -> Option<u32>;
		fn get_vote_period_end_block(profile_user_account: AccountId) -> Option<u32>;
		fn selected_as_juror(profile_user_account: AccountId, who: AccountId) -> bool;
	}
}