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

use super::{Pallet as DappStaking, *};

use astar_primitives::Balance;
use frame_benchmarking::v2::*;

use frame_support::assert_ok;
use frame_system::{Pallet as System, RawOrigin};

use ::assert_matches::assert_matches;

mod utils;
use utils::*;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn maintenance_mode() {
        initial_config::<T>();

        #[extrinsic_call]
        _(RawOrigin::Root, true);

        assert_last_event::<T>(Event::<T>::MaintenanceMode { enabled: true }.into());
    }

    #[benchmark]
    fn register() {
        initial_config::<T>();

        let account: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);

        #[extrinsic_call]
        _(RawOrigin::Root, account.clone(), smart_contract.clone());

        assert_last_event::<T>(
            Event::<T>::DAppRegistered {
                owner: account,
                smart_contract,
                dapp_id: 0,
            }
            .into(),
        );
    }

    #[benchmark]
    fn set_dapp_reward_beneficiary() {
        initial_config::<T>();

        let owner: T::AccountId = whitelisted_caller();
        let beneficiary: Option<T::AccountId> = Some(account("beneficiary", 0, SEED));
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(owner),
            smart_contract.clone(),
            beneficiary.clone(),
        );

        assert_last_event::<T>(
            Event::<T>::DAppRewardDestinationUpdated {
                smart_contract,
                beneficiary,
            }
            .into(),
        );
    }

    #[benchmark]
    fn set_dapp_owner() {
        initial_config::<T>();

        let init_owner: T::AccountId = whitelisted_caller();
        let new_owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            init_owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(init_owner),
            smart_contract.clone(),
            new_owner.clone(),
        );

        assert_last_event::<T>(
            Event::<T>::DAppOwnerChanged {
                smart_contract,
                new_owner,
            }
            .into(),
        );
    }

    #[benchmark]
    fn unregister() {
        initial_config::<T>();

        let owner: T::AccountId = whitelisted_caller();
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(RawOrigin::Root, smart_contract.clone());

        assert_last_event::<T>(
            Event::<T>::DAppUnregistered {
                smart_contract,
                era: ActiveProtocolState::<T>::get().era,
            }
            .into(),
        );
    }

    #[benchmark]
    fn lock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), amount);

        assert_last_event::<T>(
            Event::<T>::Locked {
                account: staker,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn unlock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 2;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), 1);

        assert_last_event::<T>(
            Event::<T>::Unlocking {
                account: staker,
                amount: 1,
            }
            .into(),
        );
    }

    // TODO: maybe this is not needed. Compare it after running benchmarks to the 'not-full' unlock
    #[benchmark]
    fn full_unlock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 2;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        #[extrinsic_call]
        unlock(RawOrigin::Signed(staker.clone()), amount);

        assert_last_event::<T>(
            Event::<T>::Unlocking {
                account: staker,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn claim_unlocked(x: Linear<0, { T::MaxNumberOfStakedContracts::get() }>) {
        // Prepare staker account and lock some amount
        let staker: T::AccountId = whitelisted_caller();
        let amount = (T::MinimumStakeAmount::get() + 1)
            * Into::<Balance>::into(max_number_of_contracts::<T>())
            + Into::<Balance>::into(T::MaxUnlockingChunks::get());
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Move over to the build&earn subperiod to ensure 'non-loyal' staking.
        // This is needed so we can achieve staker entry cleanup after claiming unlocked tokens.
        advance_to_next_subperiod::<T>();
        assert_eq!(
          ActiveProtocolState::<T>::get().subperiod(),
          Subperiod::BuildAndEarn,
          "Sanity check - we need to stake during build&earn for entries to be cleaned up in the next era."
        );

        // Register required number of contracts and have staker stake on them.
        // This is needed to achieve the cleanup functionality.
        for idx in 0..x {
            let smart_contract = T::BenchmarkHelper::get_smart_contract(idx as u32);
            let owner: T::AccountId = account("dapp_owner", idx.into(), SEED);

            assert_ok!(DappStaking::<T>::register(
                RawOrigin::Root.into(),
                owner.clone().into(),
                smart_contract.clone(),
            ));

            assert_ok!(DappStaking::<T>::stake(
                RawOrigin::Signed(staker.clone()).into(),
                smart_contract,
                T::MinimumStakeAmount::get() + 1,
            ));
        }

        // Unlock some amount - but we want to fill up the whole vector with chunks.
        let unlock_amount = 1;
        for _ in 0..T::MaxUnlockingChunks::get() {
            assert_ok!(DappStaking::<T>::unlock(
                RawOrigin::Signed(staker.clone()).into(),
                unlock_amount,
            ));
            run_for_blocks::<T>(One::one());
        }
        assert_eq!(
            Ledger::<T>::get(&staker).unlocking.len(),
            T::MaxUnlockingChunks::get() as usize
        );
        let unlock_amount = unlock_amount * Into::<Balance>::into(T::MaxUnlockingChunks::get());

        // Advance to next period to ensure the old stake entries are cleaned up.
        advance_to_next_period::<T>();

        // Additionally, ensure enough blocks have passed so that the unlocking chunk can be claimed.
        let unlock_block = Ledger::<T>::get(&staker)
            .unlocking
            .last()
            .expect("At least one entry must exist.")
            .unlock_block;
        run_to_block::<T>(unlock_block);

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()));

        assert_last_event::<T>(
            Event::<T>::ClaimedUnlocked {
                account: staker,
                amount: unlock_amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn relock_unlocking() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount =
            T::MinimumLockedAmount::get() * 2 + Into::<Balance>::into(T::MaxUnlockingChunks::get());
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Unlock some amount - but we want to fill up the whole vector with chunks.
        let unlock_amount = 1;
        for _ in 0..T::MaxUnlockingChunks::get() {
            assert_ok!(DappStaking::<T>::unlock(
                RawOrigin::Signed(staker.clone()).into(),
                unlock_amount,
            ));
            run_for_blocks::<T>(One::one());
        }
        let unlock_amount = unlock_amount * Into::<Balance>::into(T::MaxUnlockingChunks::get());

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()));

        assert_last_event::<T>(
            Event::<T>::Relock {
                account: staker,
                amount: unlock_amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn stake() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(staker.clone()),
            smart_contract.clone(),
            amount,
        );

        assert_last_event::<T>(
            Event::<T>::Stake {
                account: staker,
                smart_contract,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn unstake() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() + 1;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(staker.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        let unstake_amount = 1;

        #[extrinsic_call]
        _(
            RawOrigin::Signed(staker.clone()),
            smart_contract.clone(),
            unstake_amount,
        );

        assert_last_event::<T>(
            Event::<T>::Unstake {
                account: staker,
                smart_contract,
                amount: unstake_amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn claim_staker_rewards_past_period(x: Linear<1, { T::EraRewardSpanLength::get() }>) {
        initial_config::<T>();

        // Prepare staker & register smart contract
        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        // Lock some amount by the staker
        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Advance to the era just before a new span entry is created.
        // This ensures that when rewards are claimed, we'll be claiming from the new span.
        //
        // This is convenient because it allows us to control how many rewards are claimed.
        advance_to_era::<T>(T::EraRewardSpanLength::get() - 1);

        // Now ensure the expected amount of rewards are claimable.
        advance_to_era::<T>(
            ActiveProtocolState::<T>::get().era + T::EraRewardSpanLength::get() - x,
        );
        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(staker.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        // This ensures we claim from the past period.
        advance_to_next_period::<T>();

        // For testing purposes
        System::<T>::reset_events();

        #[extrinsic_call]
        claim_staker_rewards(RawOrigin::Signed(staker.clone()));

        // No need to do precise check of values, but predetermiend amount of 'Reward' events is expected.
        let dapp_staking_events = dapp_staking_events::<T>();
        assert_eq!(dapp_staking_events.len(), x as usize);
        dapp_staking_events.iter().for_each(|e| {
            assert_matches!(e, Event::Reward { .. });
        });
    }

    #[benchmark]
    fn claim_staker_rewards_ongoing_period(x: Linear<1, { T::EraRewardSpanLength::get() }>) {
        initial_config::<T>();

        // Prepare staker & register smart contract
        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        // Lock & stake some amount by the staker
        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Advance to the era just before a new span entry is created.
        // This ensures that when rewards are claimed, we'll be claiming from the new span.
        //
        // This is convenient because it allows us to control how many rewards are claimed.
        advance_to_era::<T>(T::EraRewardSpanLength::get() - 1);

        // Now ensure the expected amount of rewards are claimable.
        advance_to_era::<T>(
            ActiveProtocolState::<T>::get().era + T::EraRewardSpanLength::get() - x,
        );
        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(staker.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        // This ensures we move over the entire span.
        advance_to_era::<T>(T::EraRewardSpanLength::get() * 2);

        // For testing purposes
        System::<T>::reset_events();

        #[extrinsic_call]
        claim_staker_rewards(RawOrigin::Signed(staker.clone()));

        // No need to do precise check of values, but predetermiend amount of 'Reward' events is expected.
        let dapp_staking_events = dapp_staking_events::<T>();
        assert_eq!(dapp_staking_events.len(), x as usize);
        dapp_staking_events.iter().for_each(|e| {
            assert_matches!(e, Event::Reward { .. });
        });
    }

    #[benchmark]
    fn claim_bonus_reward() {
        initial_config::<T>();

        // Prepare staker & register smart contract
        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        // Lock & stake some amount by the staker
        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));
        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(staker.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        // Advance to the next period so we can claim the bonus reward.
        advance_to_next_period::<T>();

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), smart_contract.clone());

        // No need to do precise check of values, but last event must be 'BonusReward'.
        assert_matches!(
            dapp_staking_events::<T>().last(),
            Some(Event::BonusReward { .. })
        );
    }

    #[benchmark]
    fn claim_dapp_reward() {
        initial_config::<T>();

        // Register a dApp & stake on it.
        // This is the dApp for which we'll claim rewards for.
        let owner: T::AccountId = whitelisted_caller();
        let smart_contract = T::BenchmarkHelper::get_smart_contract(0);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 1000 * UNIT;
        T::Currency::make_free_balance_be(&owner, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(owner.clone()).into(),
            amount,
        ));
        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(owner.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        // Register & stake up to max number of contracts.
        // The reason is we want to have reward vector filled up to the capacity.
        for idx in 1..T::MaxNumberOfContracts::get() {
            let owner: T::AccountId = account("dapp_owner", idx.into(), SEED);
            let smart_contract = T::BenchmarkHelper::get_smart_contract(idx as u32);
            assert_ok!(DappStaking::<T>::register(
                RawOrigin::Root.into(),
                owner.clone().into(),
                smart_contract.clone(),
            ));

            let staker: T::AccountId = account("staker", idx.into(), SEED);
            T::Currency::make_free_balance_be(&staker, amount);
            assert_ok!(DappStaking::<T>::lock(
                RawOrigin::Signed(staker.clone()).into(),
                amount,
            ));
            assert_ok!(DappStaking::<T>::stake(
                RawOrigin::Signed(staker.clone()).into(),
                smart_contract.clone(),
                amount
            ));
        }

        // This is a hacky part to ensure we accomodate max number of contracts.
        TierConfig::<T>::mutate(|config| {
            let max_number_of_contracts: u16 = T::MaxNumberOfContracts::get().try_into().unwrap();
            config.number_of_slots = max_number_of_contracts;
            config.slots_per_tier[0] = max_number_of_contracts;
            config.slots_per_tier[1..].iter_mut().for_each(|x| *x = 0);
        });

        // Advance enough eras so dApp reward can be claimed.
        advance_to_next_subperiod::<T>();
        advance_to_next_era::<T>();
        let claim_era = ActiveProtocolState::<T>::get().era - 1;

        assert_eq!(
            DAppTiers::<T>::get(claim_era)
                .expect("Must exist since it's from past build&earn era.")
                .dapps
                .len(),
            T::MaxNumberOfContracts::get() as usize,
            "Sanity check to ensure we have filled up the vector completely."
        );

        #[extrinsic_call]
        _(
            RawOrigin::Signed(owner.clone()),
            smart_contract.clone(),
            claim_era,
        );

        // No need to do precise check of values, but last event must be 'DAppReward'.
        assert_matches!(
            dapp_staking_events::<T>().last(),
            Some(Event::DAppReward { .. })
        );
    }

    #[benchmark]
    fn unstake_from_unregistered() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        assert_ok!(DappStaking::<T>::stake(
            RawOrigin::Signed(staker.clone()).into(),
            smart_contract.clone(),
            amount
        ));

        assert_ok!(DappStaking::<T>::unregister(
            RawOrigin::Root.into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), smart_contract.clone());

        assert_last_event::<T>(
            Event::<T>::UnstakeFromUnregistered {
                account: staker,
                smart_contract,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn cleanup_expired_entries(x: Linear<1, { T::MaxNumberOfStakedContracts::get() }>) {
        initial_config::<T>();

        // Move over to the build&earn subperiod to ensure 'non-loyal' staking.
        advance_to_next_subperiod::<T>();

        // Prepare staker & lock some amount
        let staker: T::AccountId = whitelisted_caller();
        let amount = T::MinimumLockedAmount::get()
            * Into::<Balance>::into(T::MaxNumberOfStakedContracts::get());
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Register dApps up the the limit
        for idx in 0..x {
            let owner: T::AccountId = account("dapp_owner", idx.into(), SEED);
            let smart_contract = T::BenchmarkHelper::get_smart_contract(idx as u32);
            assert_ok!(DappStaking::<T>::register(
                RawOrigin::Root.into(),
                owner.clone().into(),
                smart_contract.clone(),
            ));

            assert_ok!(DappStaking::<T>::stake(
                RawOrigin::Signed(staker.clone()).into(),
                smart_contract.clone(),
                T::MinimumStakeAmount::get(),
            ));
        }

        // Move over to the next period, marking the entries as expired since they don't have the loyalty flag.
        advance_to_next_period::<T>();

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()));

        assert_last_event::<T>(
            Event::<T>::ExpiredEntriesRemoved {
                account: staker,
                count: x.try_into().unwrap(),
            }
            .into(),
        );
    }

    #[benchmark]
    fn force() {
        initial_config::<T>();

        let forcing_type = ForcingType::Subperiod;

        #[extrinsic_call]
        _(RawOrigin::Root, forcing_type);

        assert_last_event::<T>(Event::<T>::Force { forcing_type }.into());
    }

    #[benchmark]
    fn on_initialize_voting_to_build_and_earn() {
        initial_config::<T>();

        let state = ActiveProtocolState::<T>::get();
        assert_eq!(state.subperiod(), Subperiod::Voting, "Sanity check.");

        run_to_block::<T>(state.next_era_start - 1);
        DappStaking::<T>::on_finalize(state.next_era_start - 1);
        System::<T>::set_block_number(state.next_era_start);

        #[block]
        {
            DappStaking::<T>::on_initialize(state.next_era_start);
        }

        assert_eq!(
            ActiveProtocolState::<T>::get().subperiod(),
            Subperiod::BuildAndEarn
        );
    }

    // TODO: investigate why the PoV size is so large here, even after removing read of `IntegratedDApps` storage.
    // Relevant file: polkadot-sdk/substrate/utils/frame/benchmarking-cli/src/pallet/writer.rs
    // UPDATE: after some investigation, it seems that PoV size benchmarks are very unprecise
    // - the worst case measured is usually very far off the actual value that is consumed on chain.
    // There's an ongoing item to improve it (mentioned on roundtable meeting).

    /// This benchmark isn't used directly in the runtime code, but it's convenient to do manual analysis of the benchmarked values.
    /// Tier assignment is a PoV heavy operation, and it has to be properly analyzed, independently from other weight items.
    #[benchmark]
    fn dapp_tier_assignment(x: Linear<0, { max_number_of_contracts::<T>() }>) {
        // Prepare init config (protocol state, tier params & config, etc.)
        initial_config::<T>();

        // Register & stake contracts, to prepare for tier assignment.
        prepare_contracts_for_tier_assignment::<T>(x);
        advance_to_next_era::<T>();

        let reward_era = ActiveProtocolState::<T>::get().era;
        let reward_period = ActiveProtocolState::<T>::get().period_number();
        let reward_pool = Balance::from(10_000 * UNIT as u128);

        #[block]
        {
            let dapp_tiers =
                Pallet::<T>::get_dapp_tier_assignment(reward_era, reward_period, reward_pool);
            // TODO: how to move this outside of the 'block'? Cannot declare it outside, and then use it inside.
            assert_eq!(dapp_tiers.dapps.len(), x as usize);
        }
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::benchmarking::tests::new_test_ext(),
        crate::test::mock::Test,
    );
}

#[cfg(test)]
mod tests {
    use crate::test::mock;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExtBuilder::build()
    }
}
