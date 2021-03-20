use crate::{Error, mock::*};
use crate::*;
use frame_support::{assert_ok, assert_noop};

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
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn create_deparment_test() {
	new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::create_deparment(Origin::signed(1), "Education".as_bytes().to_vec(), "India".as_bytes().to_vec(), "hashcode".as_bytes().to_vec()));
		assert_eq!(TemplateModule::deparment_count(), 1);
		let dep_details = DepartmentDetails{
			name : "Education".as_bytes().to_vec(),
			location: "India".as_bytes().to_vec(),
			details: "hashcode".as_bytes().to_vec(),
			departmentid: 0,
		};
		assert_eq!(TemplateModule::department_name(0), dep_details);
		
	});
}

#[test]
fn peer_department_approve() {

	new_test_ext().execute_with(|| { 
		assert_ok!(TemplateModule::create_deparment(Origin::signed(1), "Education".as_bytes().to_vec(), "India".as_bytes().to_vec(), "hashcode".as_bytes().to_vec()));
		assert_ok!(TemplateModule::create_deparment(Origin::signed(1), "Muncipallity".as_bytes().to_vec(), "India".as_bytes().to_vec(), "hashcode".as_bytes().to_vec()));
		assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(2), 1));
		assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(2), 0));
		assert_ok!(TemplateModule::check_peers_deparment(Origin::signed(2), 1));
		// let expected_event = TestEvent::shivarthu_template(RawEvent::PeerDepartment(1, 2));
		// assert_eq!(System::events()[4].event, expected_event);
		assert_eq!(vec![0,1], TemplateModule::peer_deparments(2));
	});

}