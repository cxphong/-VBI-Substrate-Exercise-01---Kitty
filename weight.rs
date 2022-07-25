
//! Autogenerated weights for `pallet_kitty`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_kitty
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_kitty`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_kitty::WeightInfo for WeightInfo<T> {
	// Storage: Kitty KittyOwner (r:2 w:2)
	// Storage: Kitty KittyDNAs (r:1 w:1)
	fn transfer_kitty() -> Weight {
		(46_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}