use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_apply_jurors() {
	new_test_ext().execute_with(|| {
	   assert_ok!(TemplateModule::apply_jurors_positive_externality(Origin::signed(1), 2, 60));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		
	});
}


