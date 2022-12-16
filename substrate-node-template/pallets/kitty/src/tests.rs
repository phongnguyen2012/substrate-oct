use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::testing::H256;

#[test]
fn test_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let ALICE =1;
		let dna: Vec<u8> = vec![1,2,3,4,5,6,7,8];
		let price = 200u128;
		System::set_block_number(300);
		let kitty = Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna ,price);
		assert_ok!(kitty);
		
	});
}
#[test]
fn test_transfer_kitty(){
	new_test_ext().execute_with(||{
		let ALICE =1;
		let BOB =2;
		let dna: Vec<u8> = vec![10,20,30,40];
		let price = 200u128;
		let dna2: Vec<u8> = vec![10,20,30,40];
		let price2 = 200u128;
		System::set_block_number(300);
		assert_ok!(Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna ,price));
		System::set_block_number(400);
		assert_ok!(Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna2 ,price2));

		let hash1 = Kitty::kitty_owned(ALICE);
		
		
		let dna_hash = match array_bytes::hex_n_into::<_, H256, 32>("0xc46efd3d73fb016e9a3c8932075d7f40def0bda8a03a8983212910f136b9ea4b") {
			Ok(value) => value,
			Err(e) => panic!("Error:{:?}",e)
			
		};
		println!("dna_hash {:?}", dna_hash.clone());
		assert_ok!(Kitty::transfer(RuntimeOrigin::signed(ALICE), BOB, dna_hash));
	})
}
#[test]
fn get_kitty(){
	new_test_ext().execute_with(|| {
		let ALICE =1;
		let dna1: Vec<u8> = vec![98,68,7,8];
		let price1 = 200u128;
		let dna2: Vec<u8> = vec![53,97,89,7,8];
		let price2 = 500u128;
		let dna3: Vec<u8> = vec![35,82,90];
		let price3 = 700u128;
		let dna4: Vec<u8> = vec![30,96,37,88];
		let price4 = 300u128;
		System::set_block_number(300);
		let kitty1 = Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna1 ,price1);
		System::set_block_number(200);
		let kitty2 = Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna2 ,price2);
		System::set_block_number(400);
		let kitty3 = Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna3 ,price3);
		System::set_block_number(500);
		let kitty4 = Kitty::create_kitty(RuntimeOrigin::signed(ALICE),dna4 ,price4);
		
		let list_kitty = Kitty::kitty_owned(ALICE);
		println!("{:?}", list_kitty);

		assert_eq!(Kitty::total_kitty(ALICE), 4);
		//300+310+320+
		assert_eq!(Kitty::total_balance(ALICE), 1700);

	})
}
