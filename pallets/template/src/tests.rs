use crate::{mock::*, Error};
use crate::{DepartmentDetails, RawEvent};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_department_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::create_deparment(
			Origin::signed(1),
			"Education".as_bytes().to_vec(),
			"India".as_bytes().to_vec(),
			"hashcode".as_bytes().to_vec()
		));
		assert_eq!(TemplateModule::deparment_count(), 1);
		let dep_details = DepartmentDetails {
			name: "Education".as_bytes().to_vec(),
			location: "India".as_bytes().to_vec(),
			details: "hashcode".as_bytes().to_vec(),
			departmentid: 0,
		};
		assert_eq!(TemplateModule::department_name(0), dep_details);
	});
}

fn create_department() {
	assert_ok!(TemplateModule::create_deparment(
		Origin::signed(1),
		"Education".as_bytes().to_vec(),
		"India".as_bytes().to_vec(),
		"hashcode".as_bytes().to_vec()
	));
	let count = TemplateModule::deparment_count();
	assert_eq!(count, 1);
	assert_ok!(TemplateModule::create_deparment(
		Origin::signed(1),
		"Muncipallity".as_bytes().to_vec(),
		"India".as_bytes().to_vec(),
		"hashcode".as_bytes().to_vec()
	));
	assert_ok!(TemplateModule::add_citizen(
		Origin::signed(2),
		"Profilehash".as_bytes().to_vec()
	));
	assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(2), 1));
	assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(2), 0));
	assert_ok!(TemplateModule::check_deparment_of_citizen(
		Origin::signed(2),
		1
	));
	let expected_event = Event::pallet_template(RawEvent::PeerDepartment(1, 2));
	assert_eq!(System::events()[5].event, expected_event);
	assert_eq!(vec![0, 1], TemplateModule::peer_deparments(2));
}

fn create_commit_vote() {
	assert_ok!(TemplateModule::create_deparment(
		Origin::signed(1),
		"Education".as_bytes().to_vec(),
		"India".as_bytes().to_vec(),
		"hashcode".as_bytes().to_vec()
	));
	let count = TemplateModule::deparment_count();
	assert_ok!(TemplateModule::add_citizen(
		Origin::signed(2),
		"Profilehash".as_bytes().to_vec()
	));

	assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(2), 0));
	assert_ok!(TemplateModule::add_candidate_nominee(
		Origin::signed(2),
		count - 1,
		0
	));

	// Voter details
	assert_ok!(TemplateModule::add_citizen(
		Origin::signed(3),
		"Profilehash".as_bytes().to_vec()
	));
	// Voter associated to department
	assert_ok!(TemplateModule::add_peers_to_deparment(Origin::signed(3), 0));
	// Commit vote
	assert_ok!(TemplateModule::commit_vote(
		Origin::signed(3),
		count - 1,
		0,
		"Votinghash".as_bytes().to_vec()
	));
	assert_noop!(
		TemplateModule::commit_vote(
			Origin::signed(4),
			count - 1,
			0,
			"Votinghash".as_bytes().to_vec()
		),
		Error::<Test>::DepartmentNotAssociated
	);
}

#[test]
fn peer_department_approve() {
	new_test_ext().execute_with(|| create_department());
}

#[test]
fn commit_vote() {
	new_test_ext().execute_with(|| create_commit_vote());
}
