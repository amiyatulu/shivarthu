#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::dispatch::DispatchResult;
use frame_support::sp_std::prelude::*;

pub trait SchellingGameSharedLink {
	type SumTreeName;
	type SchellingGameType;
	type BlockNumber;
	type AccountId;
	type Balance;
	type RangePoint;
	type Period;

	fn get_period_link(key: Self::SumTreeName) -> Option<Self::Period>;

	fn set_to_evidence_period_link(
		key: Self::SumTreeName,
		now: Self::BlockNumber,
	) -> DispatchResult;
	fn create_tree_helper_link(key: Self::SumTreeName, k: u64) -> DispatchResult;

	fn set_to_staking_period_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> DispatchResult;

	fn set_to_staking_period_pe_link(
		key: Self::SumTreeName,
		now: Self::BlockNumber,
	) -> DispatchResult;
	fn change_period_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> DispatchResult;
	fn apply_jurors_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		who: Self::AccountId,
		stake: Self::Balance,
	) -> DispatchResult;
	fn draw_jurors_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		interations: u64,
	) -> DispatchResult;
	fn unstaking_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> DispatchResult;
	fn commit_vote_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult;
	fn reveal_vote_two_choice_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: u128,
		salt: Vec<u8>,
	) -> DispatchResult;
	fn get_incentives_two_choice_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		who: Self::AccountId,
	) -> DispatchResult;
	fn get_evidence_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32>;
	fn get_staking_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32>;
	fn get_drawing_period_end_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
	) -> (u64, u64, bool);
	fn get_commit_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32>;
	fn get_vote_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32>;
	fn selected_as_juror_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> bool;
	fn commit_vote_for_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult;
	fn reveal_vote_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: i64,
		salt: Vec<u8>,
	) -> DispatchResult;

	fn get_incentives_score_schelling_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		range_point: Self::RangePoint,
	) -> DispatchResult;

	fn get_mean_value_link( key: Self::SumTreeName) -> i64;

	fn get_all_incentives_two_choice_helper(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
	) -> DispatchResult;

	fn get_drawn_jurors(key: Self::SumTreeName) -> Vec<(Self::AccountId, u64)>;
}
