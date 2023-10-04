use crate::types::CitizenDetailsPost;
use crate::{mock::*, Error, Event};
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

		assert_noop!(
			ProfileValidation::challenge_profile(
				RuntimeOrigin::signed(4),
				1,
				challenge_content.clone()
			),
			<schelling_game_shared::Error<Test>>::EvidencePeriodNotOver
		);

		System::set_block_number(phase_data.evidence_length + 1);
		let fees = ProfileValidation::profile_registration_challenge_fees();
		let balance = Balances::free_balance(4);
		assert_eq!(300000, balance);
		assert_ok!(ProfileValidation::challenge_profile(
			RuntimeOrigin::signed(4),
			1,
			challenge_content.clone()
		));
		let balance = Balances::free_balance(4);
		assert_eq!(300000 - fees, balance);
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

#[test]
fn challenge_profile_after_time_for_staking_over_test() {
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

		assert_noop!(
			ProfileValidation::challenge_profile(
				RuntimeOrigin::signed(4),
				1,
				challenge_content.clone()
			),
			<schelling_game_shared::Error<Test>>::EvidencePeriodNotOver
		);

		System::set_block_number(phase_data.evidence_length + phase_data.end_of_staking_time + 1);
		assert_noop!(
			ProfileValidation::challenge_profile(
				RuntimeOrigin::signed(4),
				1,
				challenge_content.clone()
			),
			<schelling_game_shared::Error<Test>>::TimeForStakingOver
		);
	});
}

#[test]
fn return_profile_stake_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		let balance = Balances::free_balance(3);
		assert_eq!(300000, balance);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3), 1, 400));
		let balance = Balances::free_balance(3);
		assert_eq!(300000 - 400, balance);
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(4), 1, 600));
		let balance = Balances::free_balance(4);
		assert_eq!(300000 - 600, balance);
		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 1 };
		let period = SchellingGameShared::get_period(key.clone());
		assert_eq!(Some(Period::Evidence), period);
		let phase_data = ProfileValidation::get_phase_data();
		System::set_block_number(phase_data.evidence_length + phase_data.end_of_staking_time);
		assert_noop!(
			ProfileValidation::return_profile_stake(RuntimeOrigin::signed(3), 1),
			<schelling_game_shared::Error<Test>>::TimeForStakingNotOver
		);
		System::set_block_number(phase_data.evidence_length + phase_data.end_of_staking_time + 1);
		assert_ok!(ProfileValidation::return_profile_stake(RuntimeOrigin::signed(3), 1));
		let balance = Balances::free_balance(3);
		assert_eq!(300000, balance);
		assert_noop!(
			ProfileValidation::return_profile_stake(RuntimeOrigin::signed(3), 1),
			Error::<Test>::ProfileFundAlreadyReturned
		);

		assert_ok!(ProfileValidation::return_profile_stake(RuntimeOrigin::signed(4), 1));
		let balance = Balances::free_balance(4);
		assert_eq!(300000, balance);
		assert_noop!(
			ProfileValidation::return_profile_stake(RuntimeOrigin::signed(5), 1),
			Error::<Test>::ProfileFundNotExists
		);
	});
}

#[test]
fn schelling_game_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let content: Content = Content::IPFS(
			"bafkreiaiq24be2iioasr6ftyaum3icmj7amtjkom2jeokov5k5ojwzhvqy"
				.as_bytes()
				.to_vec(),
		);
		assert_ok!(ProfileValidation::add_citizen(RuntimeOrigin::signed(1), content.clone()));
		assert_ok!(ProfileValidation::add_profile_stake(RuntimeOrigin::signed(3), 1, 1000));
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

		let balance = Balances::free_balance(29);
		assert_eq!(300000, balance);
		for j in 4..30 {
			assert_ok!(ProfileValidation::apply_jurors(RuntimeOrigin::signed(j), 1, j * 100));
		}

		let balance = Balances::free_balance(29);
		assert_eq!(300000 - 29 * 100, balance);

		assert_noop!(
			ProfileValidation::draw_jurors(RuntimeOrigin::signed(5), 1, 5),
			<schelling_game_shared::Error<Test>>::PeriodDontMatch
		);

		assert_noop!(
			ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1),
			<schelling_game_shared::Error<Test>>::StakingPeriodNotOver
		);

		System::set_block_number(phase_data.evidence_length + 1 + phase_data.staking_length);

		assert_ok!(ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1));

		assert_ok!(ProfileValidation::draw_jurors(RuntimeOrigin::signed(5), 1, 5));

		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 1 };

		let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
		assert_eq!(5, draws_in_round);

		let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
		assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);

		assert_ok!(ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1));

		let period = SchellingGameShared::get_period(key.clone());

		assert_eq!(Some(Period::Commit), period);

		let balance: u64 = Balances::free_balance(5);
		assert_eq!(300000 - 5 * 100, balance);
		assert_ok!(ProfileValidation::unstaking(RuntimeOrigin::signed(5), 1));
		let balance = Balances::free_balance(5);
		assert_eq!(300000, balance);

		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_noop!(
			ProfileValidation::commit_vote(RuntimeOrigin::signed(6), 1, hash),
			<schelling_game_shared::Error<Test>>::JurorDoesNotExists
		);
		let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(4), 1, hash));

		// You can replace vote within the commit period.
		let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(4), 1, hash));

		let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(7), 1, hash));

		let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(13), 1, hash));

		let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(14), 1, hash));

		let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
		assert_ok!(ProfileValidation::commit_vote(RuntimeOrigin::signed(15), 1, hash));

		assert_noop!(
			ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1),
			<schelling_game_shared::Error<Test>>::CommitPeriodNotOver
		);
		System::set_block_number(
			phase_data.evidence_length + 1 + phase_data.staking_length + phase_data.commit_length,
		);
		assert_ok!(ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1));

		assert_noop!(
			ProfileValidation::reveal_vote(
				RuntimeOrigin::signed(4),
				1,
				2,
				"salt2".as_bytes().to_vec()
			),
			<schelling_game_shared::Error<Test>>::CommitDoesNotMatch
		);

		assert_ok!(ProfileValidation::reveal_vote(
			RuntimeOrigin::signed(4),
			1,
			1,
			"salt2".as_bytes().to_vec()
		));

		assert_ok!(ProfileValidation::reveal_vote(
			RuntimeOrigin::signed(7),
			1,
			1,
			"salt3".as_bytes().to_vec()
		));

		assert_ok!(ProfileValidation::reveal_vote(
			RuntimeOrigin::signed(13),
			1,
			1,
			"salt4".as_bytes().to_vec()
		));

		assert_ok!(ProfileValidation::reveal_vote(
			RuntimeOrigin::signed(14),
			1,
			1,
			"salt5".as_bytes().to_vec()
		));

		assert_noop!(
			ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1),
			<schelling_game_shared::Error<Test>>::VotePeriodNotOver
		);
		System::set_block_number(
			phase_data.evidence_length
				+ 1 + phase_data.staking_length
				+ phase_data.commit_length
				+ phase_data.vote_length,
		);
		assert_ok!(ProfileValidation::pass_period(RuntimeOrigin::signed(5), 1));

		assert_noop!(
			ProfileValidation::get_incentives(RuntimeOrigin::signed(15), 1),
			<schelling_game_shared::Error<Test>>::VoteNotRevealed
		);
		let balance: u64 = Balances::free_balance(14);
		assert_eq!(300000 - 14 * 100, balance);
		assert_ok!(ProfileValidation::get_incentives(RuntimeOrigin::signed(14), 1));
		let balance: u64 = Balances::free_balance(14);
		assert_eq!(300025, balance);
	})
}
