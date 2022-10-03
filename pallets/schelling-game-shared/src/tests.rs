use crate::{
	mock::*,
	types::{Period, SchellingGameType, RangePoint},
	Error,
};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_support::{assert_noop, assert_ok};
use sortition_sum_game::types::SumTreeName;

fn _run_to_block(n: u64) {
	while System::block_number() < n {
		TemplateModule::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		TemplateModule::on_initialize(System::block_number());
	}
}

fn return_key_profile(citizen_id: u128) -> SumTreeName {
	let key =
		SumTreeName::UniqueIdenfier1 { citizen_id, name: "challengeprofile".as_bytes().to_vec() };
	key
}

fn return_game_type_profile_approval() -> SchellingGameType {
	SchellingGameType::ProfileApproval
}

fn return_min_short_block_length() -> u64 {
	let schelling_game_type = return_game_type_profile_approval();
	let min_block_time = TemplateModule::min_block_time(schelling_game_type);
	min_block_time.min_short_block_length
}

fn return_min_long_block_length() -> u64 {
	let schelling_game_type = return_game_type_profile_approval();
	let min_block_time = TemplateModule::min_block_time(schelling_game_type);
	min_block_time.min_long_block_length
}

#[test]
fn evidence_period_not_over_test() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let now2 = now + min_short_block_length - 1;
		assert_noop!(
			TemplateModule::set_to_staking_period(key.clone(), game_type, now2),
			Error::<Test>::EvidencePeriodNotOver
		);
	});
}

/// 1) Set evidence period  
/// 2) Set staking period
/// 3) Create tree
#[test]
fn evidence_period_test() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let now2 = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(key.clone(), game_type, now2));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
	});
}

#[test]
fn apply_juror() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let now2 = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(key.clone(), game_type.clone(), now2));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
		// Check the period is staking
		let period = TemplateModule::get_period(key.clone());
		// println!("{:?}", period);
		assert_eq!(Some(Period::Staking), period);
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors_helper(
				key.clone(),
				game_type.clone(),
				j,
				j * 100
			));
		}
	});
}

#[test]
fn challenger_win_test() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let min_long_block_length = return_min_long_block_length();
		let staking_start_time = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(
			key.clone(),
			game_type.clone(),
			staking_start_time
		));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
		// Check the period is staking
		let period = TemplateModule::get_period(key.clone());
		// println!("{:?}", period);
		assert_eq!(Some(Period::Staking), period);
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors_helper(
				key.clone(),
				game_type.clone(),
				j,
				j * 100
			));
		}
		let new_now = staking_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Drawing), period);
		assert_ok!(TemplateModule::draw_jurors_helper(key.clone(), game_type.clone(), 5));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let balance = Balances::free_balance(5);
		assert_eq!(299500, balance);
		assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
		let balance = Balances::free_balance(5);
		assert_eq!(300000, balance);
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
		let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
		let commit_start_time = TemplateModule::commit_start_time(key.clone());
		let new_now = commit_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Vote), period);
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			4,
			1,
			"salt".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			7,
			1,
			"salt2".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			13,
			1,
			"salt3".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			14,
			1,
			"salt4".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			15,
			0,
			"salt5".as_bytes().to_vec()
		));
		let decision = TemplateModule::decision_count(key.clone());
		assert_eq!((1, 4), decision);
		let vote_start_time = TemplateModule::vote_start_time(key.clone());
		let new_now = vote_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
	    assert_eq!(Some(Period::Execution), period);

		let balance = Balances::free_balance(4);
		assert_eq!(299600, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 4));
		let balance = Balances::free_balance(4);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(7);
		// println!("{:?}", balance);
		assert_eq!(299300, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 7));
		let balance = Balances::free_balance(7);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(13);
		assert_eq!(298700, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 13));
		let balance = Balances::free_balance(13);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(298600, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 14));
		let balance = Balances::free_balance(14);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(15);
		assert_eq!(298500, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 15));
		let balance = Balances::free_balance(15);
		assert_eq!(299625, balance);
	});
}


#[test]
fn challenger_win_test_jurors_incentive_in_one_go() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let min_long_block_length = return_min_long_block_length();
		let staking_start_time = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(
			key.clone(),
			game_type.clone(),
			staking_start_time
		));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
		// Check the period is staking
		let period = TemplateModule::get_period(key.clone());
		// println!("{:?}", period);
		assert_eq!(Some(Period::Staking), period);
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors_helper(
				key.clone(),
				game_type.clone(),
				j,
				j * 100
			));
		}
		let new_now = staking_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Drawing), period);
		assert_ok!(TemplateModule::draw_jurors_helper(key.clone(), game_type.clone(), 5));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let balance = Balances::free_balance(5);
		assert_eq!(299500, balance);
		assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
		let balance = Balances::free_balance(5);
		assert_eq!(300000, balance);
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
		let hash = sp_io::hashing::keccak_256("0salt5".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
		let commit_start_time = TemplateModule::commit_start_time(key.clone());
		let new_now = commit_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Vote), period);
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			4,
			1,
			"salt".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			7,
			1,
			"salt2".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			13,
			1,
			"salt3".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			14,
			1,
			"salt4".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			15,
			0,
			"salt5".as_bytes().to_vec()
		));
		let decision = TemplateModule::decision_count(key.clone());
		assert_eq!((1, 4), decision);
		let vote_start_time = TemplateModule::vote_start_time(key.clone());
		let new_now = vote_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
	    assert_eq!(Some(Period::Execution), period);
		let balance = Balances::free_balance(4);
		assert_eq!(299600, balance);
		let balance = Balances::free_balance(7);
		// println!("{:?}", balance);
		assert_eq!(299300, balance);
		let balance = Balances::free_balance(13);
		assert_eq!(298700, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(298600, balance);
		let balance = Balances::free_balance(15);
		assert_eq!(298500, balance);
		assert_ok!(TemplateModule::get_all_incentives_two_choice_helper(key.clone(), game_type.clone()));
        let balance = Balances::free_balance(4);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(7);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(13);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(15);
		assert_eq!(299625, balance);
	});
}

#[test]
fn challenger_lost_test() {
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let min_long_block_length = return_min_long_block_length();
		let staking_start_time = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(
			key.clone(),
			game_type.clone(),
			staking_start_time
		));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
		// Check the period is staking
		let period = TemplateModule::get_period(key.clone());
		// println!("{:?}", period);
		assert_eq!(Some(Period::Staking), period);
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors_helper(
				key.clone(),
				game_type.clone(),
				j,
				j * 100
			));
		}
		let new_now = staking_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Drawing), period);
		assert_ok!(TemplateModule::draw_jurors_helper(key.clone(), game_type.clone(), 5));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let balance = Balances::free_balance(5);
		assert_eq!(299500, balance);
		assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
		let balance = Balances::free_balance(5);
		assert_eq!(300000, balance);
		let hash = sp_io::hashing::keccak_256("0salt".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 4, hash));
		let hash = sp_io::hashing::keccak_256("0salt2".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 7, hash));
		let hash = sp_io::hashing::keccak_256("0salt3".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 13, hash));
		let hash = sp_io::hashing::keccak_256("0salt4".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 14, hash));
		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(TemplateModule::commit_vote_helper(key.clone(), 15, hash));
		let commit_start_time = TemplateModule::commit_start_time(key.clone());
		let new_now = commit_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Vote), period);
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			4,
			0,
			"salt".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			7,
			0,
			"salt2".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			13,
			0,
			"salt3".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			14,
			0,
			"salt4".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_two_choice_helper(
			key.clone(),
			15,
			1,
			"salt5".as_bytes().to_vec()
		));
		let decision = TemplateModule::decision_count(key.clone());
		assert_eq!((4, 1), decision);
		let vote_start_time = TemplateModule::vote_start_time(key.clone());
		let new_now = vote_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
	    assert_eq!(Some(Period::Execution), period);

		let balance = Balances::free_balance(4);
		assert_eq!(299600, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 4));
		let balance = Balances::free_balance(4);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(7);
		// println!("{:?}", balance);
		assert_eq!(299300, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 7));
		let balance = Balances::free_balance(7);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(13);
		assert_eq!(298700, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 13));
		let balance = Balances::free_balance(13);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(298600, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 14));
		let balance = Balances::free_balance(14);
		assert_eq!(300025, balance);
		let balance = Balances::free_balance(15);
		assert_eq!(298500, balance);
		assert_ok!(TemplateModule::get_incentives_two_choice_helper(key.clone(), game_type.clone(), 15));
		let balance = Balances::free_balance(15);
		assert_eq!(299625, balance);
	});
}


#[test]
fn score_schelling_game_test() { 
	new_test_ext().execute_with(|| {
		let key = return_key_profile(0);
		let now = 10;
		assert_ok!(TemplateModule::set_to_evidence_period(key.clone(), now));
		assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
		let game_type = return_game_type_profile_approval();
		let min_short_block_length = return_min_short_block_length();
		let min_long_block_length = return_min_long_block_length();
		let staking_start_time = now + min_short_block_length;
		assert_ok!(TemplateModule::set_to_staking_period(
			key.clone(),
			game_type.clone(),
			staking_start_time
		));
		// Create tree
		assert_ok!(TemplateModule::create_tree_link_helper(key.clone(), 3));
		// Check the period is staking
		let period = TemplateModule::get_period(key.clone());
		// println!("{:?}", period);
		assert_eq!(Some(Period::Staking), period);
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors_helper(
				key.clone(),
				game_type.clone(),
				j,
				j * 100
			));
		}
		let new_now = staking_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Drawing), period);
		assert_ok!(TemplateModule::draw_jurors_helper(key.clone(), game_type.clone(), 5));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let balance = Balances::free_balance(5);
		assert_eq!(299500, balance);
		assert_ok!(TemplateModule::unstaking_helper(key.clone(), 5));
		let balance = Balances::free_balance(5);
		assert_eq!(300000, balance);
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_ok!(TemplateModule::commit_vote_for_score_helper(key.clone(), 4, hash));
		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(TemplateModule::commit_vote_for_score_helper(key.clone(), 7, hash));
		let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
		assert_ok!(TemplateModule::commit_vote_for_score_helper(key.clone(), 13, hash));
		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(TemplateModule::commit_vote_for_score_helper(key.clone(), 14, hash));
		let hash = sp_io::hashing::keccak_256("7salt5".as_bytes());
		assert_ok!(TemplateModule::commit_vote_for_score_helper(key.clone(), 15, hash));
		let commit_start_time = TemplateModule::commit_start_time(key.clone());
		let new_now = commit_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
		assert_eq!(Some(Period::Vote), period);
		assert_ok!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			4,
			1,
			"salt".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			7,
			1,
			"salt2".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			13,
			5,
			"salt3".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			14,
			1,
			"salt4".as_bytes().to_vec()
		));
		assert_noop!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			15,
			8,
			"salt5".as_bytes().to_vec()
		), Error::<Test>::CommitDoesNotMatch );
		assert_ok!(TemplateModule::reveal_vote_score_helper(
			key.clone(),
			15,
			7,
			"salt5".as_bytes().to_vec()
		));
		let vote_start_time = TemplateModule::vote_start_time(key.clone());
		let new_now = vote_start_time + min_long_block_length;
		assert_ok!(TemplateModule::change_period(key.clone(), game_type.clone(), new_now.clone()));
		let period = TemplateModule::get_period(key.clone());
	    assert_eq!(Some(Period::Execution), period);
		let reveal_score = TemplateModule::reveal_score_values(key.clone());
		assert_eq!(vec![1000, 1000, 5000, 1000, 7000], reveal_score);
		let balance = Balances::free_balance(4);
		assert_eq!(299600, balance);
		let balance = Balances::free_balance(7);
		// println!("{:?}", balance);
		assert_eq!(299300, balance);
		let balance = Balances::free_balance(13);
		assert_eq!(298700, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(298600, balance);
		let balance = Balances::free_balance(15);
		assert_eq!(298500, balance);
		assert_ok!(TemplateModule::get_incentives_score_schelling_helper(key.clone(), game_type.clone(), RangePoint::ZeroToTen));
		let mean_values = TemplateModule::new_mean_reveal_score(key.clone());
		assert_eq!(Some(2000), mean_values);
		let balance = Balances::free_balance(4);
		// println!("{:?}", balance);
		assert_eq!(300033, balance);
		let balance = Balances::free_balance(7);
		assert_eq!(300033, balance);
		let balance = Balances::free_balance(13); // Balance deducted as voted 5
		assert_eq!(299675, balance);
		let balance = Balances::free_balance(14);
		assert_eq!(300033, balance);
		let balance = Balances::free_balance(15); // Balance deducted as voted 7
		assert_eq!(299625, balance);




	});
}

