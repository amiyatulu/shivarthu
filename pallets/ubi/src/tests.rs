use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn fund_ubi_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::fun_ubi(Origin::signed(1)));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
		
// 	});
// }


