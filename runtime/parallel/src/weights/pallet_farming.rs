
//! Autogenerated weights for `pallet_farming`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-88-3-164`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("parallel-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parallel
// benchmark
// pallet
// --chain=parallel-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_farming
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/parallel/src/weights/pallet_farming.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_farming`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_farming::WeightInfo for WeightInfo<T> {
	// Storage: Farming Pools (r:1 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(36_558_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	fn set_pool_status() -> Weight {
		Weight::from_ref_time(36_132_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	fn set_pool_cool_down_duration() -> Weight {
		Weight::from_ref_time(36_106_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	fn reset_pool_unlock_height() -> Weight {
		Weight::from_ref_time(37_044_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		Weight::from_ref_time(124_253_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	fn withdraw() -> Weight {
		Weight::from_ref_time(76_477_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Farming Pools (r:1 w:0)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn redeem() -> Weight {
		Weight::from_ref_time(93_230_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Farming Positions (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn claim() -> Weight {
		Weight::from_ref_time(112_411_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Farming Pools (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn dispatch_reward() -> Weight {
		Weight::from_ref_time(105_775_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
}
