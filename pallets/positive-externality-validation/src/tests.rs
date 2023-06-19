use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_support::{Content, WhoAndWhen};
use crate::types::PositiveExternalityPost;

#[test]
fn test_positive_externality_post() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_positive_externality_post(
			RuntimeOrigin::signed(1),
			Content::None
		));
		let post = TemplateModule::positive_externality_post_by_id(1);
		let post_compare = Some(PositiveExternalityPost {
			id: 1,
			created: WhoAndWhen { account: 1, block: 0, time: 0 },
			edited: false,
			owner: 1,
			content: Content::None,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		});
		assert_eq!(post, post_compare);
		//    assert_ok!(TemplateModule::apply_jurors_positive_externality(Origin::signed(1), 2, 60));
	});
}

#[test]
fn test_adding_positive_externality_stake() {
	new_test_ext().execute_with(|| {
		// 	assert_ok!(TemplateModule::create_positive_externality_post(Origin::signed(1), Content::None));
		//    let post = TemplateModule::positive_externality_post_by_id(1);
		//    let post_compare = Some(PositiveExternalityPost { id: 1, created: WhoAndWhen { account: 1, block: 0, time: 0 }, edited: false, owner: 1, content: Content::None, hidden: false, upvotes_count: 0, downvotes_count: 0 });
		//    assert_eq!(post, post_compare);

		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		let stake = TemplateModule::positive_externality_user_stake(1);
		assert_eq!(stake, 10000);
	});
}

#[test]
fn test_setting_positive_externality_validation() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		let value = TemplateModule::validate_positive_externality(1);
		assert_eq!(value, true);
	});
}

#[test]
fn test_applying_for_staking_period() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		System::set_block_number(1298000 + 1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
	});
}

#[test]
fn test_appying_jurors() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(4), 1, 1000));
	});
}

#[test]
fn test_change_period() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(4), 1, 1000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(5), 1, 2000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(6), 1, 3000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(7), 1, 4000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5000));
		System::set_block_number(1298080);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
	})
}

#[test]
fn test_draw_jurors_period() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(4), 1, 1000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(5), 1, 2000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(6), 1, 3000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(7), 1, 4000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5000));
		System::set_block_number(1298080);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
		assert_ok!(TemplateModule::draw_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5));
	})
}

#[test]
fn test_drawn_jurors() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		let balance = Balances::free_balance(4);
		assert_eq!(300000, balance);
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(4), 1, 1000));
		let balance = Balances::free_balance(4);
		assert_eq!(299000, balance);
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(5), 1, 2000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(6), 1, 3000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(7), 1, 4000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5000));
		System::set_block_number(1298080);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
		assert_ok!(TemplateModule::draw_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5));
		let data = TemplateModule::get_drawn_jurors(1);
		assert_eq!(data, [(4, 1000), (5, 2000), (6, 3000), (7, 4000), (8, 5000)]);
		// println!("drawn jurors {:?}",data);
	})
}

#[test]
fn test_commit_and_incentives_vote() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(RuntimeOrigin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(RuntimeOrigin::signed(1), 10000));
		System::set_block_number(1298000);
		assert_ok!(TemplateModule::apply_staking_period(RuntimeOrigin::signed(2), 1));
		let balance = Balances::free_balance(4);
		assert_eq!(300000, balance);
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(4), 1, 1000));
		let balance = Balances::free_balance(4);
		assert_eq!(299000, balance);
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(5), 1, 2000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(6), 1, 3000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(7), 1, 4000));
		assert_ok!(TemplateModule::apply_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5000));
		System::set_block_number(1298080);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
		assert_ok!(TemplateModule::draw_jurors_positive_externality(RuntimeOrigin::signed(8), 1, 5));

		let data = TemplateModule::get_drawn_jurors(1);
		assert_eq!(data, [(4, 1000), (5, 2000), (6, 3000), (7, 4000), (8, 5000)]);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));

		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_ok!(TemplateModule::commit_vote(RuntimeOrigin::signed(4), 1, hash));

		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(TemplateModule::commit_vote(RuntimeOrigin::signed(5), 1, hash));
		let hash = sp_io::hashing::keccak_256("5salt3".as_bytes());
		assert_ok!(TemplateModule::commit_vote(RuntimeOrigin::signed(6), 1, hash));
		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(TemplateModule::commit_vote(RuntimeOrigin::signed(7), 1, hash));
		let hash = sp_io::hashing::keccak_256("5salt5".as_bytes());
		assert_ok!(TemplateModule::commit_vote(RuntimeOrigin::signed(8), 1, hash));
		System::set_block_number(12980160);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));
		assert_ok!(TemplateModule::reveal_vote(
			RuntimeOrigin::signed(4),
			1,
			1,
			"salt".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote(
			RuntimeOrigin::signed(5),
			1,
			1,
			"salt2".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote(
			RuntimeOrigin::signed(6),
			1,
			5,
			"salt3".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote(
			RuntimeOrigin::signed(7),
			1,
			1,
			"salt4".as_bytes().to_vec()
		));
		assert_ok!(TemplateModule::reveal_vote(
			RuntimeOrigin::signed(8),
			1,
			5,
			"salt5".as_bytes().to_vec()
		));
		System::set_block_number(12980260);
		assert_ok!(TemplateModule::pass_period(RuntimeOrigin::signed(4), 1));

		assert_ok!(TemplateModule::get_incentives(RuntimeOrigin::signed(4), 1));
	})
}
