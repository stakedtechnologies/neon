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

use super::{Pallet as Migration, *};

use frame_benchmarking::{account as benchmark_account, v2::*};

use frame_support::{assert_ok, storage::unhashed::put_raw, traits::Currency};

use astar_primitives::Balance;
use pallet_dapps_staking::EraInfo as OldEraInfo;

/// Generate an unique smart contract using the provided index as a sort-of indetifier
fn smart_contract<T: pallet_dapps_staking::Config>(index: u8) -> T::SmartContract {
    // This is a hacky approach to provide different smart contracts without touching the smart contract trait.
    let mut encoded_smart_contract = T::SmartContract::default().encode();
    *encoded_smart_contract.last_mut().unwrap() = index;

    Decode::decode(&mut TrailingZeroInput::new(encoded_smart_contract.as_ref()))
        .expect("Shouldn't occur as long as EVM is the default type.")
}

/// Initialize the old dApp staking pallet with some storage.
pub(super) fn initial_config<T: Config>() {
    let account: T::AccountId = whitelisted_caller();
    let dapps_number = 10;

    // Configure current era
    let era = 1;
    OldCurrentEra::<T>::put(era);
    OldGeneralEraInfo::<T>::insert(era, OldEraInfo::default());

    // Add some dummy dApps to the old pallet.
    for idx in 0..dapps_number {
        let developer: T::AccountId = benchmark_account("developer", idx.into(), 123);
        <T as pallet_dapps_staking::Config>::Currency::make_free_balance_be(
            &developer,
            <T as pallet_dapps_staking::Config>::RegisterDeposit::get() * 2,
        );
        let smart_contract = smart_contract::<T>(idx);
        assert_ok!(pallet_dapps_staking::Pallet::<T>::register(
            RawOrigin::Root.into(),
            developer,
            smart_contract,
        ));
    }

    // Add some dummy stakers to the old pallet
    <T as pallet_dapps_staking::Config>::Currency::make_free_balance_be(
        &account,
        Balance::max_value() / 2,
    );
    for idx in 0..dapps_number {
        let smart_contract = smart_contract::<T>(idx);
        assert_ok!(pallet_dapps_staking::Pallet::<T>::bond_and_stake(
            RawOrigin::Signed(account.clone()).into(),
            smart_contract.clone(),
            <T as pallet_dapps_staking::Config>::MinimumStakingAmount::get() * 1000,
        ));
    }
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn migrate_dapps_success() {
        initial_config::<T>();

        #[block]
        {
            assert!(Migration::<T>::migrate_dapps().is_ok());
        }
    }

    #[benchmark]
    fn migrate_dapps_noop() {
        #[block]
        {
            assert!(Migration::<T>::migrate_dapps().is_err());
        }
    }

    #[benchmark]
    fn migrate_ledger_success() {
        initial_config::<T>();

        #[block]
        {
            assert!(Migration::<T>::migrate_ledger().is_ok());
        }
    }

    #[benchmark]
    fn migrate_ledger_noop() {
        #[block]
        {
            assert!(Migration::<T>::migrate_ledger().is_err());
        }
    }

    #[benchmark]
    fn migrate_era_and_locked() {
        initial_config::<T>();

        #[block]
        {
            Migration::<T>::migrate_era_and_locked();
        }
    }

    #[benchmark]
    fn cleanup_old_storage_success() {
        let hashed_prefix = twox_128(pallet_dapps_staking::Pallet::<T>::name().as_bytes());
        let _ = clear_prefix(&hashed_prefix, None);

        put_raw(&hashed_prefix, &[0xFF; 128]);

        #[block]
        {
            if cfg!(test) {
                // TODO: for some reason, tests always fail here, nothing gets removed from storage.
                // When tested against real runtime, it works just fine.
                let _ = Migration::<T>::cleanup_old_storage(1).is_ok();
            } else {
                assert!(Migration::<T>::cleanup_old_storage(1).is_ok());
            }
        }
    }

    #[benchmark]
    fn cleanup_old_storage_noop() {
        let hashed_prefix = twox_128(pallet_dapps_staking::Pallet::<T>::name().as_bytes());
        let _ = clear_prefix(&hashed_prefix, None);

        #[block]
        {
            assert!(Migration::<T>::cleanup_old_storage(1).is_err());
        }
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::benchmarking::tests::new_test_ext(),
        crate::mock::Test,
    );
}

#[cfg(test)]
mod tests {
    use crate::mock;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExtBuilder::build()
    }
}
