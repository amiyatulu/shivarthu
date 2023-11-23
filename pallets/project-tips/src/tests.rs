use crate::types::TippingName;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sortition_sum_game::types::SumTreeName;
use schelling_game_shared::types::Period;



#[test]
fn check_create_project_function() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let stake_required = tipping_value.stake_required;
		let funding_needed = max_tipping_value - 100;
		let balance = Balances::free_balance(1);
		assert_ok!(ProjectTips::create_project(
			RuntimeOrigin::signed(1),
			2,
			tipping_name,
			funding_needed
		));

		let after_balance = Balances::free_balance(1);

		assert_eq!(after_balance, balance - stake_required);

		System::assert_last_event(Event::ProjectCreated { account: 1, project_id: 1 }.into());

		let next_project_id = ProjectTips::next_project_id();

		assert_eq!(2, next_project_id);

		let funding_needed = max_tipping_value + 100;

		assert_noop!(
			ProjectTips::create_project(RuntimeOrigin::signed(1), 2, tipping_name, funding_needed),
			Error::<Test>::FundingMoreThanTippingValue
		);
	});
}

#[test]
fn check_apply_staking_period_function() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 2),
			Error::<Test>::ProjectDontExists
		);

		let tipping_name = TippingName::SmallTipper;
		let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
		let max_tipping_value = tipping_value.max_tipping_value;
		let funding_needed = max_tipping_value - 100;
		assert_ok!(ProjectTips::create_project(
			RuntimeOrigin::signed(1),
			2,
			tipping_name,
			funding_needed
		));

		assert_noop!(
			ProjectTips::apply_staking_period(RuntimeOrigin::signed(3), 1),
			Error::<Test>::ProjectCreatorDontMatch
		);

		assert_ok!(ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 1));

		System::assert_last_event(
			Event::StakinPeriodStarted { project_id: 1, block_number: 1 }.into(),
		);
		System::set_block_number(5);
		assert_noop!(
			ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 1),
			Error::<Test>::ProjectIdStakingPeriodAlreadySet
		);
	});
}


#[test]
fn schelling_game_test() {
	new_test_ext().execute_with(|| {
			System::set_block_number(1);
			let tipping_name = TippingName::SmallTipper;
			let tipping_value = ProjectTips::value_of_tipping_name(tipping_name);
			let max_tipping_value = tipping_value.max_tipping_value;
			let stake_required = tipping_value.stake_required;
			let funding_needed = max_tipping_value - 100;
			let balance = Balances::free_balance(1);
			assert_ok!(ProjectTips::create_project(
				RuntimeOrigin::signed(1),
				2,
				tipping_name,
				funding_needed
			));
	
			let after_balance = Balances::free_balance(1);
	
			assert_eq!(after_balance, balance - stake_required);

			assert_ok!(ProjectTips::apply_staking_period(RuntimeOrigin::signed(1), 1));

			let phase_data = ProjectTips::get_phase_data();
	
	
			let balance = Balances::free_balance(29);
			assert_eq!(300000, balance);
			for j in 4..30 {
				assert_ok!(ProjectTips::apply_jurors_project_tips(RuntimeOrigin::signed(j), 1, j * 100));
			}
	
			let balance = Balances::free_balance(29);
			assert_eq!(300000 - 29 * 100, balance);
	
			assert_noop!(
				ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5),
				<schelling_game_shared::Error<Test>>::PeriodDontMatch
			);
	
			assert_noop!(
				ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
				<schelling_game_shared::Error<Test>>::StakingPeriodNotOver
			);
	
			System::set_block_number(1 + phase_data.staking_length);
	
			assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));
	
			assert_ok!(ProjectTips::draw_jurors(RuntimeOrigin::signed(5), 1, 5));
	
			let key = SumTreeName::ProjectTips { project_id: 1, block_number: 1 };
	
			let draws_in_round = SchellingGameShared::draws_in_round(key.clone());
			assert_eq!(5, draws_in_round);
	
			let drawn_jurors = SchellingGameShared::drawn_jurors(key.clone());
			assert_eq!(vec![(4, 400), (7, 700), (13, 1300), (14, 1400), (15, 1500)], drawn_jurors);
	
			assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));
	
			let period = SchellingGameShared::get_period(key.clone());
	
			assert_eq!(Some(Period::Commit), period);
	
			let balance: u64 = Balances::free_balance(5);
			assert_eq!(300000 - 5 * 100, balance);
			assert_ok!(ProjectTips::unstaking(RuntimeOrigin::signed(5), 1));
			let balance = Balances::free_balance(5);
			assert_eq!(300000, balance);
	
			let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
			assert_noop!(
				ProjectTips::commit_vote(RuntimeOrigin::signed(6), 1, hash),
				<schelling_game_shared::Error<Test>>::JurorDoesNotExists
			);
			let hash = sp_io::hashing::keccak_256("1salt".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));
	
			// You can replace vote within the commit period.
			let hash = sp_io::hashing::keccak_256("1salt2".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(4), 1, hash));
	
			let hash = sp_io::hashing::keccak_256("1salt3".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(7), 1, hash));
	
			let hash = sp_io::hashing::keccak_256("1salt4".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(13), 1, hash));
	
			let hash = sp_io::hashing::keccak_256("1salt5".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(14), 1, hash));
	
			let hash = sp_io::hashing::keccak_256("0salt6".as_bytes());
			assert_ok!(ProjectTips::commit_vote(RuntimeOrigin::signed(15), 1, hash));
	
			assert_noop!(
				ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
				<schelling_game_shared::Error<Test>>::CommitPeriodNotOver
			);
			System::set_block_number(
				phase_data.evidence_length + 1 + phase_data.staking_length + phase_data.commit_length,
			);
			assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));
	
			assert_noop!(
				ProjectTips::reveal_vote(
					RuntimeOrigin::signed(4),
					1,
					2,
					"salt2".as_bytes().to_vec()
				),
				<schelling_game_shared::Error<Test>>::CommitDoesNotMatch
			);
	
			assert_ok!(ProjectTips::reveal_vote(
				RuntimeOrigin::signed(4),
				1,
				1,
				"salt2".as_bytes().to_vec()
			));
	
			assert_ok!(ProjectTips::reveal_vote(
				RuntimeOrigin::signed(7),
				1,
				1,
				"salt3".as_bytes().to_vec()
			));
	
			assert_ok!(ProjectTips::reveal_vote(
				RuntimeOrigin::signed(13),
				1,
				1,
				"salt4".as_bytes().to_vec()
			));
	
			assert_ok!(ProjectTips::reveal_vote(
				RuntimeOrigin::signed(14),
				1,
				1,
				"salt5".as_bytes().to_vec()
			));
	
			assert_noop!(
				ProjectTips::pass_period(RuntimeOrigin::signed(5), 1),
				<schelling_game_shared::Error<Test>>::VotePeriodNotOver
			);
			System::set_block_number(
				phase_data.evidence_length
					+ 1 + phase_data.staking_length
					+ phase_data.commit_length
					+ phase_data.vote_length,
			);
			assert_ok!(ProjectTips::pass_period(RuntimeOrigin::signed(5), 1));
	
			assert_noop!(
				ProjectTips::get_incentives(RuntimeOrigin::signed(15), 1),
				<schelling_game_shared::Error<Test>>::VoteNotRevealed
			);
			let balance: u64 = Balances::free_balance(14);
			assert_eq!(300000 - 14 * 100, balance);
			assert_ok!(ProjectTips::get_incentives(RuntimeOrigin::signed(14), 1));
			let balance: u64 = Balances::free_balance(14);
			assert_eq!(300025, balance);
	})

}
