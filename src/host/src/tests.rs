use crate::{mock::*, *};
use frame::deps::frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Host::do_something(RuntimeOrigin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(Something::<Test>::get().map(|v| v.block_number), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            Host::cause_error(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}
