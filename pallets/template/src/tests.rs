use crate::{
	mock::*,
	types::{ChallengerFundInfo, CitizenDetails, ProfileFundInfo, SumTreeName},
	Error,
};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_support::{assert_noop, assert_ok};

fn run_to_block(n: u64) {
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

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn create_profile_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(Origin::signed(1), "hashcode".as_bytes().to_vec()));
		assert_eq!(TemplateModule::citizen_count(), 1);
		let citizen_profile = CitizenDetails {
			profile_hash: "hashcode".as_bytes().to_vec(),
			citizenid: 0,
			accountid: 1,
		};
		assert_eq!(TemplateModule::citizen_profile(0), Some(citizen_profile));
	});
}

#[test]
fn profile_fund_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(Origin::signed(1), "hashcode".as_bytes().to_vec()));
		assert_eq!(Balances::free_balance(2), 200000);
		assert_ok!(TemplateModule::add_profile_fund(Origin::signed(2), 0));
		assert_eq!(Balances::free_balance(2), 199000);
		let profile_fundinfocheck =
			ProfileFundInfo { deposit: 1000, start: 0, validated: false, reapply: false };
		let profile_fundinfo = TemplateModule::profile_fund(0);
		assert_eq!(profile_fundinfo, Some(profile_fundinfocheck));
	});
}

#[test]
fn challenge_profile_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(Origin::signed(1), "hashcode".as_bytes().to_vec()));
		assert_ok!(TemplateModule::add_profile_fund(Origin::signed(2), 0));
		assert_eq!(Balances::free_balance(3), 300000);
		assert_ok!(TemplateModule::challenge_profile(Origin::signed(3), 0));
		assert_eq!(Balances::free_balance(3), 299900);
	});
}

#[test]
fn sum_tree_set() {
	new_test_ext().execute_with(|| {
		let key = SumTreeName::UniqueIdenfier1 { citizen_id: 1, name: "key1".as_bytes().to_vec() };
		assert_ok!(TemplateModule::create_tree(key.clone(), 5));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3));
		assert_ok!(TemplateModule::set(key.clone(), 50, 4));
		assert_eq!(TemplateModule::stake_of(key.clone(), 1), Ok(Some(20)));
		assert_eq!(TemplateModule::draw(key.clone(), 90), Ok(4));
		let stake = TemplateModule::stake_of(key.clone(), 10);
		println!("Stake {:?}", stake);
	});
}

#[test]
fn schelling_game_remove_stake() {
	new_test_ext().execute_with(|| {
		let key = SumTreeName::UniqueIdenfier1 { citizen_id: 1, name: "key1".as_bytes().to_vec() };
		assert_ok!(TemplateModule::create_tree(key.clone(), 2));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3));
		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		// println!("{:?}", data2);
		assert_ok!(TemplateModule::set(key.clone(), 50, 4));
		// assert_ok!(TemplateModule::set(Origin::signed(1), key.clone(), 0, 4 ));

		// let data = TemplateModule::draw(key.clone(), 130 );

		// println!("{:?}", data);

		// let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		// println!("{:?}", data2);

		// let data = TemplateModule::draw(key.clone(), 130);
		// println!("{:?}", data);

		assert_ok!(TemplateModule::set(key.clone(), 0, 3));

		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		// println!("{:?}", data2);

		let data = TemplateModule::draw(key.clone(), 98);
		// println!("{:?}", data);

		// assert_eq!(TemplateModule::stake_of(key.clone(), 1 ), Ok(20));
		// assert_eq!(TemplateModule::draw("key1".as_bytes().to_vec(), 120), Ok(4));
	});
}

#[test]
fn draw_jurors_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(Origin::signed(1), "hashcode".as_bytes().to_vec()));
		assert_eq!(Balances::free_balance(2), 200000);
		assert_ok!(TemplateModule::add_profile_fund(Origin::signed(2), 0));
		assert_eq!(Balances::free_balance(2), 199000);
		let profile_fundinfocheck =
			ProfileFundInfo { deposit: 1000, start: 0, validated: false, reapply: false };
		let profile_fundinfo = TemplateModule::profile_fund(0);
		assert_eq!(profile_fundinfo, Some(profile_fundinfocheck));
		run_to_block(10);
		assert_eq!(Balances::free_balance(3), 300000);
		assert_ok!(TemplateModule::challenge_profile(Origin::signed(3), 0));
		assert_eq!(Balances::free_balance(3), 299900);
		assert_eq!(
			TemplateModule::challenger_fund(0),
			Some(ChallengerFundInfo {
				challengerid: 3,
				deposit: 100,
				start: 10,
				challenge_completed: false
			})
		);
		run_to_block(43200 + 10 +144000);
		assert_ok!(TemplateModule::pass_period(Origin::signed(2), 0));
		// Applyjuror
		for j in 4..30 {
			assert_ok!(TemplateModule::apply_jurors(Origin::signed(j), 0, j * 100));
		}
        // run_to_block(43200 + 10 +144000 + 10);
		let key = SumTreeName::UniqueIdenfier1 {
			citizen_id: 0,
			name: "challengeprofile".as_bytes().to_vec(),
		};

		let staking_start_time = TemplateModule::staking_start_time(key.clone());
		// println!("staking start time {:?}", staking_start_time);

		let block_time = TemplateModule::min_block_time();
		// println!("block time {:?}", block_time.min_block_length);

		run_to_block(staking_start_time + block_time.min_block_length);

		assert_ok!(TemplateModule::pass_period(Origin::signed(2), 0));

		assert_ok!(TemplateModule::draw_jurors(Origin::signed(1), 0, 4));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(3, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![13,14,15], drawn_jurors);
		assert_ok!(TemplateModule::draw_jurors(Origin::signed(1), 0, 4));
		let draws_in_round = TemplateModule::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);
		let drawn_jurors = TemplateModule::drawn_jurors(key.clone());
		assert_eq!(vec![4, 13, 14, 15, 16], drawn_jurors);
		assert_ok!(TemplateModule::pass_period(Origin::signed(2), 0));
		// assert_ok!(TemplateModule::draw_jurors(Origin::signed(1), 0, 4));
	});
}
