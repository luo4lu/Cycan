
//! Autogenerated weights for pallet_rgrandpa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-12-23, STEPS: [1, ], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/cycan
// benchmark
// --chain
// dev
// --execution
// wasm
// --pallet
// pallet_rgrandpa
// --extrinsic
// *
// --steps
// 1
// --repeat
// 10
// --raw
// --output
// weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_rgrandpa.
pub trait WeightInfo {
	fn set_parameter() -> Weight;
}
/// Weight functions for pallet_rgrandpa.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn set_parameter() -> Weight {
		(51_427_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}
// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_parameter() -> Weight {
		(0 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}