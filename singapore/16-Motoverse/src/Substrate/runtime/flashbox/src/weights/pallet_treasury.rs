// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("flashbox_dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_treasury
// --extrinsic
// *
// --chain=flashbox_dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/flashbox_weights/pallet_treasury.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_treasury using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Treasury::ProposalCount` (r:1 w:1)
	/// Proof: `Treasury::ProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:0 w:1)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn spend_local() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `1887`
		// Minimum execution time: 14_919_000 picoseconds.
		Weight::from_parts(15_391_000, 1887)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Treasury::ProposalCount` (r:1 w:1)
	/// Proof: `Treasury::ProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:0 w:1)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn propose_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `1489`
		// Minimum execution time: 29_332_000 picoseconds.
		Weight::from_parts(30_262_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Treasury::Proposals` (r:1 w:1)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479`
		//  Estimated: `6196`
		// Minimum execution time: 44_952_000 picoseconds.
		Weight::from_parts(45_823_000, 6196)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Treasury::Proposals` (r:1 w:0)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 99]`.
	fn approve_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508 + p * (8 ±0)`
		//  Estimated: `3573`
		// Minimum execution time: 11_523_000 picoseconds.
		Weight::from_parts(14_652_779, 3573)
			// Standard Error: 1_697
			.saturating_add(Weight::from_parts(92_988, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `165`
		//  Estimated: `1887`
		// Minimum execution time: 8_840_000 picoseconds.
		Weight::from_parts(9_042_000, 1887)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `System::Account` (r:199 w:199)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Deactivated` (r:1 w:1)
	/// Proof: `Treasury::Deactivated` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::InactiveIssuance` (r:1 w:1)
	/// Proof: `Balances::InactiveIssuance` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:99 w:99)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 99]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + p * (253 ±0)`
		//  Estimated: `3593 + p * (5206 ±0)`
		// Minimum execution time: 24_069_000 picoseconds.
		Weight::from_parts(34_424_078, 3593)
			// Standard Error: 12_544
			.saturating_add(Weight::from_parts(39_750_052, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 5206).saturating_mul(p.into()))
	}
	/// Storage: `Treasury::SpendCount` (r:1 w:1)
	/// Proof: `Treasury::SpendCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Spends` (r:0 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `1489`
		// Minimum execution time: 13_378_000 picoseconds.
		Weight::from_parts(13_909_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `6196`
		// Minimum execution time: 60_479_000 picoseconds.
		Weight::from_parts(61_747_000, 6196)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn check_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `198`
		//  Estimated: `3534`
		// Minimum execution time: 15_365_000 picoseconds.
		Weight::from_parts(15_684_000, 3534)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn void_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `198`
		//  Estimated: `3534`
		// Minimum execution time: 14_392_000 picoseconds.
		Weight::from_parts(14_842_000, 3534)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}