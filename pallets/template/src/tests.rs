use crate::{
	mock::*,
	types::{CitizenDetails, ProfileFundInfo},
	Error,
};
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
		assert_ok!(TemplateModule::add_profile_fund(Origin::signed(2), 0));
		let profile_fundinfocheck = ProfileFundInfo { deposit: 100, start: 0, validated: false, reapply: false };
		let profile_fundinfo = TemplateModule::profile_fund(0);
		assert_eq!(profile_fundinfo, Some(profile_fundinfocheck));
	});
}
