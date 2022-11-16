use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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
fn add_tag_works() {
	new_test_ext().execute_with(|| {
		let tag = 	"Municipality".as_bytes().to_vec();	
		assert_ok!(TemplateModule::add_tag(Origin::signed(1), 1, tag.clone()));
		let tags = TemplateModule::department_tags(1);
		let mut value_tags: Vec<Vec<u8>> = vec![];
		value_tags.push(tag);
		assert_eq!(tags, value_tags);		
	});
}

#[test]
fn downvote_and_remove_tags_works(){
	new_test_ext().execute_with(|| {
		let tag = 	"Municipality".as_bytes().to_vec();	
		assert_ok!(TemplateModule::add_tag(Origin::signed(1), 1, tag.clone()));
		assert_ok!(TemplateModule::donwvote_tag(Origin::signed(1), 1, tag.clone()));

	});
}


