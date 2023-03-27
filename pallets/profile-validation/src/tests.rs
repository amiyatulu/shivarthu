use crate::{
	mock::*,
	types::{ChallengerFundInfo, CitizenDetails, ProfileFundInfo},
	Error,
};
use frame_support::{assert_noop, assert_ok};


#[test]
fn it_works_for_default_value() {
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
fn check_update_profile_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(Origin::signed(1), "hashcode".as_bytes().to_vec()));
		assert_eq!(TemplateModule::citizen_count(), 1);
		let citizen_profile = CitizenDetails {
			profile_hash: "hashcode".as_bytes().to_vec(),
			citizenid: 0,
			accountid: 1,
		};
		assert_eq!(TemplateModule::citizen_profile(0), Some(citizen_profile));
		assert_ok!(TemplateModule::update_profile(Origin::signed(1), 0, "hashcode2".as_bytes().to_vec()));
		let citizen_profile = CitizenDetails {
			profile_hash: "hashcode2".as_bytes().to_vec(),
			citizenid: 0,
			accountid: 1,
		};
		assert_eq!(TemplateModule::citizen_profile(0), Some(citizen_profile));
		

	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
