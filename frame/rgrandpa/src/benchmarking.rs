//! Benchmarking setup for pallet-rgrandpa

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use sp_std::{vec, vec::Vec, boxed::Box};
use frame_system::Origin;

benchmarks! {
	set_parameter {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, 100,T::BlockNumber::from(20 as u32))
	verify {
		assert_eq!(CyclePercent::<T>::get(), 100);
	}
}

impl_benchmark_test_suite!(
  	RGrandpa,
  	crate::mock::ExtBuilder::default().build(),
	crate::mock::Test,
);
