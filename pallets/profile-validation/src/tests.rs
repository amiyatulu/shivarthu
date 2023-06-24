use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use pallet_support::Content;
use crate::types::{CitizenDetailsPost};
use pallet_support::{WhoAndWhen};

#[test]
fn add_citizen_profile_check() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let content: Content = Content::IPFS("bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy".as_bytes().to_vec());
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let data = ProfileValidation::citizen_profile(1);
		let profile = Some(CitizenDetailsPost::<Test> { created: WhoAndWhen { account: 1, block: 1, time: 0 }, content: content, citizen_id: 1, owner: 1, edited: false, hidden: false, upvotes_count: 0, downvotes_count: 0 });
		assert_eq!(data, profile);
	});
}

#[test]
fn check_fund_addition() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let content: Content = Content::IPFS("bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy".as_bytes().to_vec());
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let data = ProfileValidation::citizen_profile(1);
		let profile = Some(CitizenDetailsPost::<Test> { created: WhoAndWhen { account: 1, block: 1, time: 0 }, content: content, citizen_id: 1, owner: 1, edited: false, hidden: false, upvotes_count: 0, downvotes_count: 0 });
		assert_eq!(data, profile);
		let balance = Balances::free_balance(3);
		assert_eq!(300000, balance);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3),1, 100 ));
		let balance = Balances::free_balance(3);
		assert_eq!(300000 - 100, balance);
		let total_fund = ProfileValidation::total_fund_for_profile_collected(1);
		assert_eq!(100, total_fund);
	})
}
