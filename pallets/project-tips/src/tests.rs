use crate::types::TippingName;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

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

