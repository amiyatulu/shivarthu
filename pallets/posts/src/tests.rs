use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {

	});
}
