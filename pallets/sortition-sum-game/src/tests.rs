use crate::{
	mock::*,
	types::{SumTreeName},
};
use frame_support::{assert_ok};



#[test]
fn profile_fund_test() {
	new_test_ext().execute_with(|| {
		let key = SumTreeName::UniqueIdenfier1{citizen_id: 1, name:"key1".as_bytes().to_vec() };
		assert_ok!(TemplateModule::create_tree(key.clone(), 5));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1 ));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1 ));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2 ));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3 ));
		assert_ok!(TemplateModule::set(key.clone(), 50, 4 ));
		assert_eq!(TemplateModule::stake_of(key.clone(), 1 ), Ok(Some(20)));
		assert_eq!(TemplateModule::draw(key.clone(), 90), Ok(4));
	});
}

#[test]
fn schelling_game_remove_stake() {
	new_test_ext().execute_with(|| {
		let key = SumTreeName::UniqueIdenfier1{citizen_id: 1, name:"key1".as_bytes().to_vec() };
		assert_ok!(TemplateModule::create_tree(key.clone(), 2));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1 ));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1 ));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2 ));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3 ));
		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		println!("{:?}", data2);
		assert_ok!(TemplateModule::set(key.clone(), 50, 4 ));
		// assert_ok!(TemplateModule::set(Origin::signed(1), key.clone(), 0, 4 ));

		// let data = TemplateModule::draw(key.clone(), 130 );
		

		// println!("{:?}", data);

		// let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		// println!("{:?}", data2);

	

		

		// let data = TemplateModule::draw(key.clone(), 130);
		// println!("{:?}", data);

	    assert_ok!(TemplateModule::set(key.clone(), 0, 3 ));

		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		println!("{:?}", data2);

		let data = TemplateModule::draw(key.clone(), 98 );
		println!("{:?}", data);





		// assert_eq!(TemplateModule::stake_of(key.clone(), 1 ), Ok(20));
		// assert_eq!(TemplateModule::draw("key1".as_bytes().to_vec(), 120), Ok(4));

		
	});
}
