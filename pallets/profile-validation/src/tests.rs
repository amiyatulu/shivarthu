use crate::types::CitizenDetailsPost;
use crate::{
	mock::{self, *},
	Error, Event,
};
use frame_support::{assert_noop, assert_ok};
use pallet_support::Content;
use pallet_support::WhoAndWhen;
use schelling_game_shared::types::Period;
use sortition_sum_game::types::SumTreeName;

#[test]
fn add_citizen_profile_check() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let data = ProfileValidation::citizen_profile(1);
		let profile = Some(CitizenDetailsPost::<Test> {
			created: WhoAndWhen { account: 1, block: 1, time: 0 },
			content,
			citizen_id: 1,
			owner: 1,
			edited: false,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		});
		assert_eq!(data, profile);
		System::set_block_number(5);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqz"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let data = ProfileValidation::citizen_profile(1);
		let profile = Some(CitizenDetailsPost::<Test> {
			created: WhoAndWhen { account: 1, block: 5, time: 0 },
			content,
			citizen_id: 1,
			owner: 1,
			edited: false,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		});
		assert_eq!(data, profile);
	});
}

#[test]
fn check_fund_addition() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(10);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let data = ProfileValidation::citizen_profile(1);
		let profile = Some(CitizenDetailsPost::<Test> {
			created: WhoAndWhen { account: 1, block: 10, time: 0 },
			content,
			citizen_id: 1,
			owner: 1,
			edited: false,
			hidden: false,
			upvotes_count: 0,
			downvotes_count: 0,
		});
		assert_eq!(data, profile);
		let balance = Balances::free_balance(3);
		assert_eq!(300000, balance);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3), 1, 100));
		let balance = Balances::free_balance(3);
		assert_eq!(300000 - 100, balance);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqz"
				.as_bytes()
				.to_vec(),
		);
		assert_noop!(
			ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()),
			Error::<Test>::NoMoreUpdates
		);
		let data = ProfileValidation::profile_fund_details(1, 3).unwrap();
		assert_eq!(100, data.deposit);
		let total_fund = ProfileValidation::total_fund_for_profile_collected(1);
		assert_eq!(100, total_fund);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3), 1, 100));
		let balance = Balances::free_balance(3);
		assert_eq!(300000 - 200, balance);
		let data = ProfileValidation::profile_fund_details(1, 3).unwrap();
		assert_eq!(200, data.deposit);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(4), 1, 500));
		let balance = Balances::free_balance(4);
		assert_eq!(300000 - 500, balance);
		let data = ProfileValidation::profile_fund_details(1, 4).unwrap();
		assert_eq!(500, data.deposit);
		assert_noop!(
			ProfileValidation::add_profile_stake(RuntimeOrigin::signed(5), 1, 1000),
			Error::<Test>::AmountFundedGreaterThanRequired
		);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(5), 1, 300));
		System::assert_last_event(Event::ProfileFund { profile: 1, funder: 5 }.into());

		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 10 };
		let period = SchellingGameShared::get_period(key);
		assert_eq!(Some(Period::Evidence), period);
	})
}

#[test]
fn challenge_evidence() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3), 1, 1000));
		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 1 };
		let period = SchellingGameShared::get_period(key.clone());
		assert_eq!(Some(Period::Evidence), period);

		let challenge_content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhabc"
				.as_bytes()
				.to_vec(),
		);

		let phase_data = ProfileValidation::get_phase_data();

		System::set_block_number(phase_data.evidence_length + 1);
		assert_ok!(ProfileValidation::challenge_profile(
			RuntimeOrigin::signed(4),
			1,
			challenge_content.clone()
		));
		let period = SchellingGameShared::get_period(key.clone());
		assert_eq!(Some(Period::Staking), period);

		assert_noop!(
			ProfileValidation::challenge_profile(
				RuntimeOrigin::signed(4),
				2,
				challenge_content.clone()
			),
			Error::<Test>::CitizenDoNotExists
		);
	})
}
