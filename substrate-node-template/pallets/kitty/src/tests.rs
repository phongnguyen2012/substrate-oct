use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let dna: Vec<u8> = vec![1,2,3,4,5,6,7,8];
		let price = 200u128;
		let createkitty = Kitty::create_kitty(RuntimeOrigin::signed(1),dna ,price);
		assert_ok!(createkitty);
		// Read pallet storage and assert an expected result.
		assert_eq!(Kitty::kitty_id(), 1);
	});
}
