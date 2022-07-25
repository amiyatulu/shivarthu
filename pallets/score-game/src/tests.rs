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
fn check_standard_deviation() {
	new_test_ext().execute_with(|| {
		let data_initial = vec![-10, 1, 1, 1, 5, 1, 1, 7];
		let data_integer = data_initial.into_iter().map(|x| x * 1000).collect::<Vec<i64>>();
		let data_mean_integer = TemplateModule::mean_integer(&data_integer);
		assert_eq!(Some(875), data_mean_integer);

		let data_std_deviation_integer = TemplateModule::std_deviation_interger(&data_integer);
		assert_eq!(Some(4648), data_std_deviation_integer);

		let new_mean_integer = TemplateModule::calculate_new_mean(
			&data_integer,
			data_mean_integer,
			data_std_deviation_integer,
		);
		assert_eq!(Some(1666), new_mean_integer)
	});
}
