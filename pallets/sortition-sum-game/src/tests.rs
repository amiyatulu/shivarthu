use crate::{mock::*, types::SumTreeName, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 10 };
		assert_ok!(TemplateModule::create_tree(key.clone(), 5));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3));
		assert_ok!(TemplateModule::set(key.clone(), 50, 4));
		assert_eq!(TemplateModule::stake_of(key.clone(), 1), Ok(Some(20)));
		assert_eq!(TemplateModule::draw(key.clone(), 90), Ok(4));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let key = SumTreeName::ProfileValidation { citizen_address: 1, block_number: 10 };
		assert_ok!(TemplateModule::create_tree(key.clone(), 2));
		assert_ok!(TemplateModule::set(key.clone(), 10, 1));
		assert_ok!(TemplateModule::set(key.clone(), 20, 1));
		assert_ok!(TemplateModule::set(key.clone(), 30, 2));
		assert_ok!(TemplateModule::set(key.clone(), 40, 3));
		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		println!("{:?}", data2);
		assert_ok!(TemplateModule::set(key.clone(), 50, 4));
		assert_ok!(TemplateModule::set(key.clone(), 0, 3));

		let data2 = TemplateModule::query_leafs(key.clone(), 0, 5);
		println!("{:?}", data2);

		let data = TemplateModule::draw(key.clone(), 98);
		println!("{:?}", data);
	});
}
