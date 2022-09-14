use crate::{
	mock::*,
	types::{Period, SchellingGameType},
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

fn _return_min_long_block_length() -> u64 {
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
		let _period = TemplateModule::get_period(key.clone());
		// assert_eq!(Option<Period::Staking>, period);
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
