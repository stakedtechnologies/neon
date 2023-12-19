
// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_dapp_staking_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `devserver-01`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("shibuya-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=shibuya-dev
// --steps=50
// --repeat=20
// --pallet=pallet_dapp_staking_migration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/shibuya-dev/dapp_staking_migration_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;
use pallet_dapp_staking_migration::WeightInfo;

/// Weights for pallet_dapp_staking_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DappsStaking RegisteredDapps (r:2 w:1)
	/// Proof: DappsStaking RegisteredDapps (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DappStaking IntegratedDApps (r:1 w:1)
	/// Proof: DappStaking IntegratedDApps (max_values: Some(65535), max_size: Some(121), added: 2101, mode: MaxEncodedLen)
	/// Storage: DappStaking CounterForIntegratedDApps (r:1 w:1)
	/// Proof: DappStaking CounterForIntegratedDApps (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: DappStaking NextDAppId (r:1 w:1)
	/// Proof: DappStaking NextDAppId (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	fn migrate_dapps_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `6112`
		// Minimum execution time: 44_805_000 picoseconds.
		Weight::from_parts(45_614_000, 6112)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	/// Proof: DappsStaking RegisteredDapps (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	fn migrate_dapps_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3551`
		// Minimum execution time: 3_274_000 picoseconds.
		Weight::from_parts(3_445_000, 3551)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: DappsStaking Ledger (r:2 w:1)
	/// Proof: DappsStaking Ledger (max_values: None, max_size: Some(266), added: 2741, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:1)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DappStaking Ledger (r:1 w:1)
	/// Proof: DappStaking Ledger (max_values: None, max_size: Some(310), added: 2785, mode: MaxEncodedLen)
	/// Storage: DappStaking CurrentEraInfo (r:1 w:1)
	/// Proof: DappStaking CurrentEraInfo (max_values: Some(1), max_size: Some(112), added: 607, mode: MaxEncodedLen)
	fn migrate_ledger_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1748`
		//  Estimated: `6472`
		// Minimum execution time: 67_930_000 picoseconds.
		Weight::from_parts(69_038_000, 6472)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: DappsStaking Ledger (r:1 w:0)
	/// Proof: DappsStaking Ledger (max_values: None, max_size: Some(266), added: 2741, mode: MaxEncodedLen)
	fn migrate_ledger_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3731`
		// Minimum execution time: 2_728_000 picoseconds.
		Weight::from_parts(2_916_000, 3731)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: DappsStaking Ledger (r:2 w:1)
	/// Proof: DappsStaking Ledger (max_values: None, max_size: Some(266), added: 2741, mode: MaxEncodedLen)
	fn cleanup_old_storage_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `739`
		//  Estimated: `6472`
		// Minimum execution time: 7_221_000 picoseconds.
		Weight::from_parts(7_456_000, 6472)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn cleanup_old_storage_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_058_000 picoseconds.
		Weight::from_parts(2_228_000, 0)
	}
}
