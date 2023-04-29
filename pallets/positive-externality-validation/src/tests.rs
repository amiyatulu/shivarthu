use crate::types::PositiveExternalityPost;
use crate::{mock::*, Error};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_support::{assert_noop, assert_ok};
use pallet_support::{Content, WhoAndWhen};

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
fn test_positive_externality_post() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_positive_externality_post(
			Origin::signed(1),
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

		assert_ok!(TemplateModule::add_positive_externality_stake(Origin::signed(1), 10000));
		let stake = TemplateModule::positive_externality_user_stake(1);
		assert_eq!(stake, 10000);
	});
}

#[test]
fn test_setting_positive_externality_validation() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(Origin::signed(1), true));
		let value = TemplateModule::validate_positive_externality(1);
		assert_eq!(value, true);
	});
}

#[test]
fn test_applying_for_staking_period() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_validate_positive_externality(Origin::signed(1), true));
		assert_ok!(TemplateModule::add_positive_externality_stake(Origin::signed(1), 10000));
		run_to_block(1298000);
		assert_ok!(TemplateModule::apply_staking_period(Origin::signed(2), 1));
		run_to_block(1298000+ 1298000);
		assert_ok!(TemplateModule::apply_staking_period(Origin::signed(2), 1));

	});
}
