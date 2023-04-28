use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_support::{Content, WhoAndWhen};
use crate::types::PositiveExternalityPost;

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
fn test_apply_jurors() {
	new_test_ext().execute_with(|| {
	   assert_ok!(TemplateModule::create_positive_externality_post(Origin::signed(1), Content::None));
	   let post = TemplateModule::positive_externality_post_by_id(1);
	//    println!("{:?}", post);

	   let post_compare = Some(PositiveExternalityPost { id: 1, created: WhoAndWhen { account: 1, block: 0, time: 0 }, edited: false, owner: 1, content: Content::None, hidden: false, upvotes_count: 0, downvotes_count: 0 });
	   assert_eq!(post, post_compare);
	//    assert_ok!(TemplateModule::apply_jurors_positive_externality(Origin::signed(1), 2, 60));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		
	});
}


