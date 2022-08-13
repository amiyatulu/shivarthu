use crate::{mock::*, Error, types::{Period, SchellingGameType}};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{OnFinalize, OnInitialize};
use sortition_sum_game::types::{SumTreeName};


fn run_to_block(n: u64) {
	while System::block_number() < n {
		TemplateModule::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		TemplateModule::on_initialize(System::block_number());
	}
}

fn return_key(citizen_id: u128) -> SumTreeName {
	let key = SumTreeName::UniqueIdenfier1 {
		citizen_id: citizen_id,
		name: "challengeprofile".as_bytes().to_vec(),
	};
	key
}

fn return_game_type() -> SchellingGameType {
	SchellingGameType::ProfileApproval
}




#[test]
fn evidence_period_not_over_test(){
	new_test_ext().execute_with(|| {
       let key = return_key(0);
	   assert_ok!(TemplateModule::set_to_evidence_period(key.clone()));
	   assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
	   let game_type = return_game_type();
	   assert_noop!(TemplateModule::set_to_staking_period(key.clone(), game_type, 10, 20),  Error::<Test>::EvidencePeriodNotOver);
	});
}



#[test]
fn evidence_period__test(){
	new_test_ext().execute_with(|| {
       let key = return_key(0);
	   assert_ok!(TemplateModule::set_to_evidence_period(key.clone()));
	   assert_eq!(TemplateModule::get_period(&key).unwrap(), Period::Evidence);
	   let game_type = return_game_type();
	   assert_ok!(TemplateModule::set_to_staking_period(key.clone(), game_type, 10, 60));
	});
}

