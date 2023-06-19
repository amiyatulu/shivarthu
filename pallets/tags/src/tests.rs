use crate::{mock::*, Error, Event, types::DownVoteDetails};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_tag_works() {
	new_test_ext().execute_with(|| {
		let tag = "Municipality".as_bytes().to_vec();
		assert_ok!(TemplateModule::add_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		let tags = TemplateModule::department_tags(1);
		let mut value_tags: Vec<Vec<u8>> = vec![];
		value_tags.push(tag);
		assert_eq!(tags, value_tags);
	});
}

#[test]
fn downvote_works() {
	new_test_ext().execute_with(|| {
		let tag = "Municipality".as_bytes().to_vec();
		assert_ok!(TemplateModule::add_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		let downvote_details = TemplateModule::downvote_details_of_tag(1, tag.clone());
		assert_eq!(downvote_details.downvote, 1);
	});
}

#[test]
fn downvote_again_error() {
	new_test_ext().execute_with(|| {
		let tag = "Municipality".as_bytes().to_vec();
		assert_ok!(TemplateModule::add_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		let downvote_details = TemplateModule::downvote_details_of_tag(1, tag.clone());
		assert_eq!(downvote_details.downvote, 1);
		let tag2 = "Education".as_bytes().to_vec();
		assert_ok!(TemplateModule::add_tag(RuntimeOrigin::signed(1), 1, tag2.clone()));
		assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(1), 1, tag2.clone()));
		let downvote_details = TemplateModule::downvote_details_of_tag(1, tag2.clone());
		assert_eq!(downvote_details.downvote, 1);
		assert_noop!(
			TemplateModule::donwvote_tag(RuntimeOrigin::signed(1), 1, tag.clone()),
			Error::<Test>::UserAlreadyDownVoted
		);
		assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(2), 1, tag.clone()));
		let downvote_details = TemplateModule::downvote_details_of_tag(1, tag.clone());
		assert_eq!(downvote_details.downvote, 2);
	});
}

#[test]
fn downvote_remove_tag() {
	new_test_ext().execute_with(|| {
		let tag = "Municipality".as_bytes().to_vec();
		let down_vote_threshold = TemplateModule::downvote_threshold();
		assert_ok!(TemplateModule::add_tag(RuntimeOrigin::signed(1), 1, tag.clone()));
		for x in 1..down_vote_threshold {
			assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(x.into()), 1, tag.clone()));
			let downvote_details = TemplateModule::downvote_details_of_tag(1, tag.clone());
			assert_eq!(downvote_details.downvote, x);
			// println!("x={}", x);
		}
		assert_ok!(TemplateModule::donwvote_tag(RuntimeOrigin::signed(5), 1, tag.clone()));
		assert_noop!(
			TemplateModule::donwvote_tag(RuntimeOrigin::signed(6), 1, tag.clone()),
			Error::<Test>::TagDoesnotExists
		);
		let value_tags: Vec<Vec<u8>> = vec![];
		assert_eq!(TemplateModule::department_tags(1), value_tags);
		let downvote_details = DownVoteDetails::default();
		assert_eq!(TemplateModule::downvote_details_of_tag(1, tag.clone()), downvote_details);
	});
}
