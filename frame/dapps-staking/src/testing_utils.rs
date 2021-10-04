use super::*;
use frame_support::assert_ok;
use mock::{EraIndex, *};
use sp_runtime::traits::Zero;
use sp_runtime::Perbill;

/// Used to register contract for staking and assert success.
pub(crate) fn register_contract(developer: AccountId, contract: &MockSmartContract<AccountId>) {
    assert_ok!(mock::DappsStaking::enable_developer_pre_approval(
        Origin::root(),
        false
    ));
    assert_ok!(DappsStaking::register(
        Origin::signed(developer),
        contract.clone()
    ));
}

/// Used to get total dapps reward for an era.
pub(crate) fn get_total_reward_per_era() -> Balance {
    BLOCK_REWARD * BLOCKS_PER_ERA as Balance
}

/// Used to perform bond_and_stake with success assertion.
pub(crate) fn bond_and_stake_with_verification(
    staker_id: AccountId,
    contract_id: &MockSmartContract<AccountId>,
    value: Balance,
) {
    assert_ok!(DappsStaking::bond_and_stake(
        Origin::signed(staker_id),
        contract_id.clone(),
        value,
    ));
}

/// Used to perform unbond_unstake_and_withdraw with success assertion.
pub(crate) fn unbond_unstake_and_withdraw_with_verification(
    staker_id: AccountId,
    contract_id: &MockSmartContract<AccountId>,
    value: Balance,
) {
    assert_ok!(DappsStaking::unbond_unstake_and_withdraw(
        Origin::signed(staker_id),
        contract_id.clone(),
        value,
    ));
}

/// Used to verify ledger content.
pub(crate) fn verify_ledger(staker_id: AccountId, staked_value: Balance) {
    // Verify that ledger storage values are as expected.
    let ledger = Ledger::<TestRuntime>::get(staker_id);
    assert_eq!(staked_value, ledger);
}

/// Used to verify era staking points content.
pub(crate) fn verify_era_staking_points(
    contract_id: &MockSmartContract<AccountId>,
    total_staked_value: Balance,
    era: crate::EraIndex,
    stakers: Vec<(AccountId, Balance)>,
) {
    // Verify that era staking points are as expected for the contract
    let era_staking_points = ContractEraStake::<TestRuntime>::get(&contract_id, era).unwrap();
    assert_eq!(total_staked_value, era_staking_points.total);
    assert_eq!(stakers.len(), era_staking_points.stakers.len());

    for (staker_id, staked_value) in stakers {
        assert_eq!(
            staked_value,
            *era_staking_points.stakers.get(&staker_id).unwrap()
        );
    }
}

/// Used to verify pallet era staked value.
pub(crate) fn verify_pallet_era_staked(era: crate::EraIndex, total_staked_value: Balance) {
    // Verify that total staked amount in era is as expected
    let era_rewards = EraRewardsAndStakes::<TestRuntime>::get(era).unwrap();
    assert_eq!(total_staked_value, era_rewards.staked);
}

/// Used to verify pallet era staked and reward values.
pub(crate) fn verify_pallet_era_staked_and_reward(
    era: crate::EraIndex,
    total_staked_value: Balance,
    total_reward_value: Balance,
) {
    // Verify that total staked amount in era is as expected
    let era_rewards = EraRewardsAndStakes::<TestRuntime>::get(era).unwrap();
    assert_eq!(total_staked_value, era_rewards.staked);
    assert_eq!(total_reward_value, era_rewards.rewards);
}

/// Used to verify storage content after claim() is successfuly executed.
pub(crate) fn verify_contract_history_is_cleared(
    contract: MockSmartContract<AccountId>,
    from_era: EraIndex,
    to_era: EraIndex,
) {
    // check claim era is changed
    assert_eq!(
        mock::DappsStaking::contract_last_claimed(&contract).unwrap_or(Zero::zero()),
        to_era
    );

    // check last staked era changed
    assert_eq!(
        mock::DappsStaking::contract_last_staked(&contract).unwrap_or(Zero::zero()),
        to_era
    );

    // check new ContractEraStaked
    let era_staking_points = mock::DappsStaking::contract_era_stake(contract, to_era).unwrap();
    assert_eq!(era_staking_points.former_staked_era, to_era);

    // check history storage is cleared
    for era in from_era..to_era {
        assert!(mock::DappsStaking::contract_era_stake(contract, era).is_none());
    }
}

/// Used to perform claim with success assertion
pub(crate) fn claim(
    claimer: AccountId,
    contract: MockSmartContract<AccountId>,
    start_era: EraIndex,
    claim_era: EraIndex,
) {
    assert_ok!(DappsStaking::claim(Origin::signed(claimer), contract));
    // check the event for claim
    System::assert_last_event(mock::Event::DappsStaking(crate::Event::ContractClaimed(
        contract, claimer, start_era, claim_era,
    )));
}

/// Used to calculate the expected reward for the staker
pub(crate) fn calc_expected_staker_reward(
    era_reward: mock::Balance,
    era_stake: mock::Balance,
    contract_stake: mock::Balance,
    staker_stake: mock::Balance,
) -> mock::Balance {
    let contract_reward = Perbill::from_rational(contract_stake, era_stake) * era_reward;
    let contract_staker_part =
        Perbill::from_percent(100 - DEVELOPER_REWARD_PERCENTAGE) * contract_reward;

    Perbill::from_rational(contract_staker_part, contract_stake) * staker_stake
}

/// Used to calculate the expected reward for the developer
pub(crate) fn calc_expected_developer_reward(
    era_reward: mock::Balance,
    era_stake: mock::Balance,
    contract_stake: mock::Balance,
) -> mock::Balance {
    let contract_reward = Perbill::from_rational(contract_stake, era_stake) * era_reward;
    Perbill::from_percent(DEVELOPER_REWARD_PERCENTAGE) * contract_reward
}

/// Check staker/dev Balance after reward distribution.
/// Check that claimed rewards for staker/dev are updated.
pub(crate) fn check_rewards_on_balance_and_storage(
    contract: &MockSmartContract<AccountId>,
    user: &AccountId,
    free_balance: mock::Balance,
    eras: EraIndex,
    expected_era_reward: mock::Balance,
) {
    let total_reward_per_era = expected_era_reward * eras as u128;
    assert_eq!(
        <mock::TestRuntime as Config>::Currency::free_balance(user),
        free_balance + total_reward_per_era
    );
    assert_eq!(
        mock::DappsStaking::rewards_claimed(contract, user),
        total_reward_per_era
    );
}

/// Check that claimed rewards on this contract are updated
pub(crate) fn check_paidout_rewards_for_contract(
    contract: &MockSmartContract<AccountId>,
    expected_contract_reward: mock::Balance,
) {
    let era_last_claimed = mock::DappsStaking::contract_last_claimed(contract).unwrap_or(0);
    let contract_staking_info =
        mock::DappsStaking::contract_era_stake(contract, era_last_claimed).unwrap_or_default();
    assert_eq!(
        contract_staking_info.claimed_rewards,
        expected_contract_reward
    )
}

/// Used to verify that storage is cleared of all contract related values after unregistration.
pub(crate) fn verify_storage_after_unregister(
    developer: &AccountId,
    contract_id: &MockSmartContract<AccountId>,
    current_era: EraIndex,
) {
    assert!(!ContractEraStake::<TestRuntime>::contains_key(
        contract_id,
        &current_era
    ));
    assert!(!RegisteredDapps::<TestRuntime>::contains_key(contract_id));
    assert!(!RegisteredDevelopers::<TestRuntime>::contains_key(
        developer
    ));
    assert!(!ContractLastStaked::<TestRuntime>::contains_key(
        contract_id
    ));
    assert!(!ContractLastClaimed::<TestRuntime>::contains_key(
        contract_id
    ));

    // T::Currency::unreserve(&developer, T::RegisterDeposit::get()); // TODO: verify this somehow?
}
