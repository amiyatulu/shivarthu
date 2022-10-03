use crate::*;

use schelling_game_shared_link::SchellingGameSharedLink;

impl<T: Config> SchellingGameSharedLink for Pallet<T> {
	type SumTreeName = SumTreeName;
	type SchellingGameType = SchellingGameType;
	type BlockNumber = BlockNumberOf<T>;
	type AccountId = AccountIdOf<T>;
	type Balance = BalanceOf<T>;
	type RangePoint = RangePoint;

	/// Set `PeriodName` to `Period::Evidence`
	/// Called with submission of `Evidence` stake e.g. Profile stake
	/// Also set `EvidenceStartTime`    
	fn set_to_evidence_period_link(
		key: Self::SumTreeName,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::set_to_evidence_period(key, now)
	}

	/// Create a sortition sum tree   
	fn create_tree_helper_link(key: Self::SumTreeName, k: u64) -> DispatchResult {
		Self::create_tree_link_helper(key, k)
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
	fn set_to_staking_period_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::set_to_staking_period(key, game_type, now)
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
	fn change_period_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> DispatchResult {
		Self::change_period(key, game_type, now)
	}

	/// Apply Jurors      
	/// Ensure `Period` is `Staking`      
	/// Slash the stake.   
	/// Store the stake on sortition sum tree if doesn't exists.   
	fn apply_jurors_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		who: Self::AccountId,
		stake: Self::Balance,
	) -> DispatchResult {
		Self::apply_jurors_helper(key, game_type, who, stake)
	}

	/// Draw Jurors  
	/// Ensure `Period` is `Drawing`  
	/// `iterations` is number of jurors drawn per call  
	/// Ensure total draws `draws_in_round` is less than `max_draws`
	fn draw_jurors_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		iterations: u64,
	) -> DispatchResult {
		Self::draw_jurors_helper(key, game_type, iterations)
	}

	/// Unstake those who are not drawn as jurors   
	/// They can withdraw their stake   
	fn unstaking_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> DispatchResult {
		Self::unstaking_helper(key, who)
	}

	/// Commit vote   
	fn commit_vote_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult {
		Self::commit_vote_helper(key, who, vote_commit)
	}

	/// Reveal vote   
	/// There are two vote choices 0 or 1  
	fn reveal_vote_two_choice_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: u128,
		salt: Vec<u8>,
	) -> DispatchResult {
		Self::reveal_vote_two_choice_helper(key, who, choice, salt)
	}
	/// Distribute incentives for two choices        
	/// Winner gets `stake` + `winning_incentives`      
	/// If decision is draw, jurors receive their `stake`    
	/// Lost jurors gets `stake * 3/4`   
	/// When they receive their incentives, their accountid is stored in `JurorsIncentiveDistributedAccounts`        
	fn get_incentives_two_choice_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		who: Self::AccountId,
	) -> DispatchResult {
		Self::get_incentives_two_choice_helper(key, game_type, who)
	}

	/// Blocks left for ending evidence period
	/// When evidence time ends, you can submit the challenge stake    
	/// `start_block_number` evidence start time which you will get from `EvidenceStartTime`    
	fn get_evidence_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_evidence_period_end_block_helper(key, game_type, now)
	}

	/// Blocks left for ending staking period  
	fn get_staking_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_staking_period_end_block_helper(key, game_type, now)
	}

	/// Return true when drawing period is over, otherwise false   
	fn get_drawing_period_end_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
	) -> (u64, u64, bool) {
		Self::get_drawing_period_end_helper(key, game_type)
	}

	/// Blocks left for ending drawing period
	fn get_commit_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_commit_period_end_block_helper(key, game_type, now)
	}

	/// Blocks left for ending vote period
	fn get_vote_period_end_block_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		now: Self::BlockNumber,
	) -> Option<u32> {
		Self::get_vote_period_end_block_helper(key, game_type, now)
	}

	/// Check if `AccountId` is selected as juror
	fn selected_as_juror_helper_link(key: Self::SumTreeName, who: Self::AccountId) -> bool {
		Self::selected_as_juror_helper(key, who)
	}

	/// Commit vote for score schelling game
	fn commit_vote_for_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		vote_commit: [u8; 32],
	) -> DispatchResult {
		Self::commit_vote_for_score_helper(key, who, vote_commit)
	}

	/// Reveal vote for score schelling game
	fn reveal_vote_score_helper_link(
		key: Self::SumTreeName,
		who: Self::AccountId,
		choice: i64,
		salt: Vec<u8>,
	) -> DispatchResult {
		Self::reveal_vote_score_helper(key, who, choice, salt)
	}
    
	/// Distribute incentives to all score schelling game jurors
	fn get_incentives_score_schelling_helper_link(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
		range_point: Self::RangePoint,
	) -> DispatchResult {
		Self::get_incentives_score_schelling_helper(key, game_type, range_point)
	}

	/// Distribute incentives to all two choice shelling game jurors
	fn get_all_incentives_two_choice_helper(
		key: Self::SumTreeName,
		game_type: Self::SchellingGameType,
	) -> DispatchResult {
		Self::get_all_incentives_two_choice_helper(key, game_type)
	}
}
