use crate::{mock::{*, self}, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::set_member(mock::Origin::signed(1), 42, 43));
		// Read pallet storage and assert an expected result.
		frame_support::debug(&TemplateModule::check(42, 43));
		assert_eq!(TemplateModule::check(42, 43), Some(1));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// The anti-case will fail:
		// let _set_member_test = TemplateModule::set_member(mock::Origin::signed(1), 42, 43);
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(mock::Origin::signed(1), 42, 43), Error::<Test>::NoneValue);
	});
}
