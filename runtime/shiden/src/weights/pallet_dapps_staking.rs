
//! Autogenerated weights for `pallet_dapps_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("testnet"), DB CACHE: 128

// Executed Command:
// ./target/release/astar-collator
// benchmark
// --chain
// testnet
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_dapps_staking
// --steps
// 20
// --repeat
// 10
// --extrinsic
// *
// --output
// .


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_dapps_staking.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dapps_staking::WeightInfo for WeightInfo<T> {
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: DappsStaking PreApprovalIsEnabled (r:1 w:0)
	fn register() -> Weight {
		(91_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractLastClaimed (r:1 w:1)
	// Storage: DappsStaking ContractEraStake (r:1 w:0)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:0 w:1)
	// Storage: DappsStaking ContractLastStaked (r:0 w:1)
	// Storage: DappsStaking Ledger (r:25 w:25)
	// Storage: Balances Locks (r:25 w:25)
	// Storage: DappsStaking RewardsClaimed (r:0 w:25)
	fn unregister(n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 110_000
			.saturating_add((61_408_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: DappsStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		(3_900_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DappsStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		(11_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking ContractLastStaked (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking ContractLastClaimed (r:0 w:1)
	// Storage: DappsStaking ContractEraStake (r:0 w:1)
	fn bond_and_stake(n: u32, ) -> Weight {
		(176_807_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((760_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking ContractLastStaked (r:1 w:1)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: DappsStaking RewardsClaimed (r:0 w:1)
	fn unbond_unstake_and_withdraw(n: u32, ) -> Weight {
		(191_412_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((741_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking ContractLastStaked (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractLastClaimed (r:1 w:1)
	// Storage: DappsStaking ContractEraStake (r:1 w:2)
	// Storage: DappsStaking EraRewardsAndStakes (r:30 w:0)
	// Storage: DappsStaking RewardsClaimed (r:2 w:2)
	fn claim(n: u32, m: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 257_000
			.saturating_add((22_457_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 2_084_000
			.saturating_add((31_572_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: DappsStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
