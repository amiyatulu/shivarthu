use crate::{mock::*, Error};
use crate::{CitizenDetails, DepartmentDetails, RawEvent};
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

#[test]
fn create_profile_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_citizen(
			Origin::signed(1),
			"hashcode".as_bytes().to_vec()
		));
		assert_eq!(TemplateModule::citizen_count(), 1);
		let citizen_details = CitizenDetails {
			profile_hash: "hashcode".as_bytes().to_vec(),
			citizenid: 0,
		};
		assert_eq!(TemplateModule::citizen_details(0), citizen_details);
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

	assert_ok!(TemplateModule::add_citizen_to_deparment(
		Origin::signed(2),
		0
	));
	assert_ok!(TemplateModule::add_citizen_to_deparment(
		Origin::signed(2),
		1
	));
	let expected_event = Event::pallet_template(RawEvent::CitizenDepartment(1, 2));
	assert_eq!(System::events()[4].event, expected_event);
	let citizen_id = TemplateModule::citizen_id(2);
	// println!("citizen id .... {}", citizen_id.unwrap());
	assert_eq!(
		vec![0, 1],
		TemplateModule::citizen_deparments(citizen_id.unwrap())
	);
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

	assert_ok!(TemplateModule::add_citizen_to_deparment(
		Origin::signed(2),
		0
	));
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

	assert_ok!(TemplateModule::add_citizen(
		Origin::signed(4),
		"Profilehash".as_bytes().to_vec()
	));
	// Voter associated to department
	assert_ok!(TemplateModule::add_citizen_to_deparment(
		Origin::signed(3),
		0
	));
	assert_ok!(TemplateModule::add_citizen(
		Origin::signed(5),
		"Profilehash".as_bytes().to_vec()
	));
	assert_ok!(TemplateModule::add_citizen_to_deparment(
		Origin::signed(5),
		0
	));
	// Commit vote
	assert_ok!(TemplateModule::commit_vote(
		Origin::signed(3),
		count - 1,
		0,
		"2-Votinghash".as_bytes().to_vec()
	));

	assert_noop!(
		TemplateModule::commit_vote(
			Origin::signed(4),
			count - 1,
			0,
			"2-Votinghash2".as_bytes().to_vec()
		),
		Error::<Test>::DepartmentNotAssociated
	);

	assert_noop!(
		TemplateModule::commit_vote(
			Origin::signed(5),
			count - 1,
			0,
			"2-Votinghash".as_bytes().to_vec()
		),
		Error::<Test>::AlreadyCommitUsed
	);
}

fn create_reveal_vote() {
	let count = TemplateModule::deparment_count();
	assert_ok!(TemplateModule::reveal_vote(
		Origin::signed(3),
		count - 1,
		0,
		"2".as_bytes().to_vec(),
		"Votinghash".as_bytes().to_vec(),
		"votecommit".as_bytes().to_vec(),
	));
}

#[test]
fn peer_department_approve() {
	new_test_ext().execute_with(|| create_department());
}

#[test]
fn commit_vote() {
	new_test_ext().execute_with(|| create_commit_vote());
}

#[test]
fn test_seq_phragmen() {
	new_test_ext().execute_with(|| {
		let mut candidates = Vec::new();
		for number in 1..10 {
			candidates.push(number);
		}
		let mut voters = Vec::new();
		for number in 10..20 {
			voters.push(number);
		}

		let _result = TemplateModule::my_test_seq_phragmen(Origin::signed(1), candidates, voters );


	})

}

// #[test]
// fn reveal_vote() {
// 	new_test_ext().execute_with(|| {
// 		create_commit_vote();
// 		create_reveal_vote();
// 	});
// }


// #[test]
// fn hash_test() {
// 	new_test_ext().execute_with(|| {
// 		let ok = TemplateModule::test_hash(Origin::signed(1),"1-abcdef".as_bytes().to_vec(), "e2a18e9b74f228590ca8c563cecfc58c28455b2dde25b4bbdc663e99e791f47c".as_bytes().to_vec());
// 	});
// }
