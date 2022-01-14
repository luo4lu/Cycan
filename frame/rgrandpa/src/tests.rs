use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_core::crypto::AccountId32;
use sp_runtime::{
	assert_eq_error_rate, traits::BadOrigin,
};

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
#[test]
fn rgrandpa_set_param() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(RGrandpa::set_parameter(Origin::signed(ALICE), 20, 20), BadOrigin);
		assert_ok!(RGrandpa::set_parameter(Origin::root(), 20, 20));
		assert_eq!(RGrandpa::cycle_confirmer_num(), 20);
		assert_eq!(RGrandpa::cycle_block_num(), 20);
		assert_eq!(Staking::minimum_validator_count(), 2);
		assert_eq!(Staking::validator_count(), 100);
	});
}
