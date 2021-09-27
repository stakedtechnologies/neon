//! Dapps staking FRAME Pallet.

use super::*;
use frame_support::{
    dispatch::DispatchResult,
    ensure,
    pallet_prelude::*,
    traits::{Currency, Get, LockIdentifier, LockableCurrency, WithdrawReasons},
    weights::Weight,
};
use frame_system::{ensure_root, ensure_signed, pallet_prelude::*};
use sp_runtime::{
    print,
    traits::{Saturating, Zero},
    Perbill,
};
use sp_std::convert::From;

const STAKING_ID: LockIdentifier = *b"dapstake";

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// The balance type of this pallet.
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(crate) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The staking balance.
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

        /// Reword amount per block. Will be divided by DAppsRewardPercentage
        type RewardAmount: Get<BalanceOf<Self>>;

        /// The percentage of the network block reward that goes to this pallet
        type DAppsRewardPercentage: Get<u32>;

        /// Number of blocks per era.
        #[pallet::constant]
        type BlockPerEra: Get<BlockNumberFor<Self>>;

        // TODO: this should be used?
        /// Minimum bonded deposit for new contract registration.
        #[pallet::constant]
        type RegisterDeposit: Get<BalanceOf<Self>>;

        /// Percentage of reward paid to developer.
        #[pallet::constant]
        type DeveloperRewardPercentage: Get<u32>;

        /// Maximum number of unique stakers per contract.
        #[pallet::constant]
        type MaxNumberOfStakersPerContract: Get<u32>;

        /// Minimum amount user must stake on contract.
        /// User can stake less if they already have the minimum staking amount staked on that particular contract.
        #[pallet::constant]
        type MinimumStakingAmount: Get<BalanceOf<Self>>;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::type_value]
    pub(crate) fn HistoryDepthOnEmpty() -> u32 {
        84u32
    }

    /// Map from all (unlocked) "controller" accounts to the info regarding the staking.
    #[pallet::storage]
    #[pallet::getter(fn ledger)]
    pub(crate) type Ledger<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, StakingLedger<BalanceOf<T>>>;

    /// Number of eras to keep in history.
    ///
    /// Information is kept for eras in `[current_era - history_depth; current_era]`.
    ///
    /// Must be more than the number of eras delayed by session otherwise. I.e. active era must
    /// always be in history. I.e. `active_era > current_era - history_depth` must be
    /// guaranteed.
    #[pallet::storage]
    #[pallet::getter(fn history_depth)]
    pub(crate) type HistoryDepth<T> = StorageValue<_, u32, ValueQuery, HistoryDepthOnEmpty>;

    /// The current era index.
    ///
    /// This is the latest planned era, depending on how the Session pallet queues the validator
    /// set, it might be active or not.
    #[pallet::storage]
    #[pallet::getter(fn current_era)]
    pub type CurrentEra<T> = StorageValue<_, EraIndex, ValueQuery>;

    /// Accumulator for block rewards during an era. It is reset at every new era
    #[pallet::storage]
    #[pallet::getter(fn block_reward_accumulator)]
    pub type BlockRewardAccumulator<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::type_value]
    pub fn ForceEraOnEmpty() -> Forcing {
        Forcing::ForceNone
    }
    /// Mode of era forcing.
    #[pallet::storage]
    #[pallet::getter(fn force_era)]
    pub type ForceEra<T> = StorageValue<_, Forcing, ValueQuery, ForceEraOnEmpty>;

    /// Registered developer accounts points to coresponding contract
    #[pallet::storage]
    #[pallet::getter(fn registered_contract)]
    pub(crate) type RegisteredDevelopers<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, SmartContract<T::AccountId>>;

    /// Registered dapp points to the developer who registered it
    #[pallet::storage]
    #[pallet::getter(fn registered_developer)]
    pub(crate) type RegisteredDapps<T: Config> =
        StorageMap<_, Twox64Concat, SmartContract<T::AccountId>, T::AccountId>;

    /// Total block rewards for the pallet per era
    #[pallet::storage]
    #[pallet::getter(fn era_reward_and_stake)]
    pub(crate) type EraRewardsAndStakes<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, EraRewardAndStake<BalanceOf<T>>>;

    /// Stores amount staked and stakers for a contract per era
    #[pallet::storage]
    #[pallet::getter(fn contract_era_stake)]
    pub(crate) type ContractEraStake<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        SmartContract<T::AccountId>,
        Twox64Concat,
        EraIndex,
        EraStakingPoints<T::AccountId, BalanceOf<T>>,
    >;

    /// Marks an Era when a contract is last claimed
    #[pallet::storage]
    #[pallet::getter(fn contract_last_claimed)]
    pub(crate) type ContractLastClaimed<T: Config> =
        StorageMap<_, Twox64Concat, SmartContract<T::AccountId>, EraIndex>;

    /// Marks an Era when a contract is last (un)staked
    #[pallet::storage]
    #[pallet::getter(fn contract_last_staked)]
    pub(crate) type ContractLastStaked<T: Config> =
        StorageMap<_, Twox64Concat, SmartContract<T::AccountId>, EraIndex>;

    // Declare the genesis config (optional).
    //
    // The macro accepts either a struct or an enum; it checks that generics are consistent.
    //
    // Type must implement the `Default` trait.
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {
        _myfield: u32,
    }

    // Declare genesis builder. (This is need only if GenesisConfig is declared)
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    pub enum Event<T: Config> {
        /// Account has bonded and staked funds on a smart contract.
        BondAndStake(T::AccountId, SmartContract<T::AccountId>, BalanceOf<T>),
        /// Account has unbonded, unstaked and withdrawn funds.
        UnbondUnstakeAndWithdraw(T::AccountId, SmartContract<T::AccountId>, BalanceOf<T>),
        /// New contract added for staking, with deposit value
        NewContract(T::AccountId, SmartContract<T::AccountId>),
        /// New dapps staking era. Distribute era rewards to contracts
        NewDappStakingEra(EraIndex),
        /// The contract's reward have been claimed, by an account, from era, until era
        ContractClaimed(
            SmartContract<T::AccountId>,
            T::AccountId,
            EraIndex,
            EraIndex,
        ),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Can not stake with zero value.
        StakingWithNoValue,
        /// Can not stake with value less than minimum staking value
        InsufficientStakingValue,
        /// Number of stakers per contract exceeded.
        MaxNumberOfStakersExceeded,
        /// Targets must be operated contracts
        NotOperatedContract,
        /// Contract isn't staked.
        NotStakedContract,
        /// Unstaking a contract with zero value
        UnstakingWithNoValue,
        /// The contract is already registered by other account
        AlreadyRegisteredContract,
        /// User attempts to register with address which is not contract
        ContractIsNotValid,
        /// Claiming contract with no developer account
        ContractNotRegistered,
        // TODO: make use of this?
        /// Missing deposit for the contract registration
        InsufficientDeposit,
        /// This account was already used to register contract
        AlreadyUsedDeveloperAccount,
        /// Unexpected state error, used to abort transaction. Used for situations that 'should never happen'.
        UnexpectedState,
        /// Report issue on github if this is ever emitted
        UnknownStartStakingData,
        /// Report issue on github if this is ever emitted
        UnknownEraReward,
        /// There are no funds to reward the contract. Or already claimed in that era
        NothingToClaim,
        /// Contract already claimed in this era and reward is distributed
        AlreadyClaimedInThisEra,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
            // Handle dapps staking era
            let block_rewards = Self::block_reward_accumulator();
            BlockRewardAccumulator::<T>::put(block_rewards + T::RewardAmount::get());
            let force_new_era = Self::force_era().eq(&Forcing::ForceNew);
            let blocks_pre_era = T::BlockPerEra::get();

            // Value is compared to 1 since genesis block is ignored
            if now % blocks_pre_era == BlockNumberFor::<T>::from(1u32) || force_new_era {
                let previous_era = Self::current_era();
                Self::reward_balance_snapshoot(previous_era);
                let next_era = previous_era + 1;
                CurrentEra::<T>::put(next_era);
                let zero_balance: BalanceOf<T> = Default::default();
                BlockRewardAccumulator::<T>::put(zero_balance);
                if force_new_era {
                    ForceEra::<T>::put(Forcing::ForceNone);
                }
                Self::deposit_event(Event::<T>::NewDappStakingEra(next_era));
            }

            // just return the weight of the on_finalize.
            T::DbWeight::get().reads(1)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// register contract into staking targets.
        /// contract_id should be ink! or evm contract.
        ///
        /// Any user can call this function.
        /// However, caller have to have deposit amount.
        /// TODO: weight, and add registrationFee
        #[pallet::weight(1_000_000_000)]
        pub fn register(
            origin: OriginFor<T>,
            contract_id: SmartContract<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            let developer = ensure_signed(origin)?;

            ensure!(
                !RegisteredDevelopers::<T>::contains_key(&developer),
                Error::<T>::AlreadyUsedDeveloperAccount
            );
            ensure!(
                !RegisteredDapps::<T>::contains_key(&contract_id),
                Error::<T>::AlreadyRegisteredContract
            );
            ensure!(
                Self::is_contract_valid(&contract_id),
                Error::<T>::ContractIsNotValid
            );

            RegisteredDapps::<T>::insert(contract_id.clone(), developer.clone());
            RegisteredDevelopers::<T>::insert(&developer, contract_id.clone());

            Self::deposit_event(Event::<T>::NewContract(developer, contract_id));

            Ok(().into())
        }

        /// Lock up and stake balance of the origin account.
        ///
        /// `value` must be more than the `minimum_balance` specified by `T::Currency`
        /// unless account already has bonded value equal or more than 'minimum_balance'.
        ///
        /// The dispatch origin for this call must be _Signed_ by the staker's account.
        ///
        /// Effects of staking will be felt at the beginning of the next era.
        ///
        /// TODO: Weight!
        #[pallet::weight(10_000_000)]
        pub fn bond_and_stake(
            origin: OriginFor<T>,
            contract_id: SmartContract<T::AccountId>,
            #[pallet::compact] value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            ensure!(
                RegisteredDapps::<T>::contains_key(&contract_id),
                Error::<T>::NotOperatedContract
            );

            // Get the staking ledger or create an entry if it doesn't exist.
            let mut ledger = Self::ledger(&staker).unwrap_or_default();

            // Ensure that staker has enough balance to bond & stake.
            let free_balance = T::Currency::free_balance(&staker);
            // Remove already locked funds from the free balance
            let available_balance = free_balance.saturating_sub(ledger.total);
            let value_to_stake = value.min(available_balance);
            ensure!(!value_to_stake.is_zero(), Error::<T>::StakingWithNoValue);

            // update the ledger value by adding the newly bonded funds
            ledger.total += value_to_stake;
            ledger.active += value_to_stake;

            // Get the latest era staking point info or create it if contract hasn't been staked yet so far.
            let era_when_contract_last_staked = Self::contract_last_staked(&contract_id);
            let mut latest_era_staking_points =
                if let Some(last_stake_era) = era_when_contract_last_staked.clone() {
                    // No era staking points struct available even though we have information that contract was staked before. This is a bug!
                    Self::contract_era_stake(&contract_id, &last_stake_era)
                        .ok_or(Error::<T>::UnexpectedState)?
                } else {
                    EraStakingPoints {
                        total: Zero::zero(),
                        stakers: BTreeMap::<T::AccountId, BalanceOf<T>>::new(),
                        former_staked_era: 0 as EraIndex,
                    }
                };

            // Ensure that we can add additional staker for the contract.
            if !latest_era_staking_points.stakers.contains_key(&staker) {
                ensure!(
                    latest_era_staking_points.stakers.len()
                        < T::MaxNumberOfStakersPerContract::get() as usize,
                    Error::<T>::MaxNumberOfStakersExceeded
                );
            }

            // Increment the staked amount.
            latest_era_staking_points.total += value_to_stake;
            let entry = latest_era_staking_points
                .stakers
                .entry(staker.clone())
                .or_insert(Zero::zero());
            *entry += value_to_stake;

            ensure!(
                *entry >= T::MinimumStakingAmount::get(),
                Error::<T>::InsufficientStakingValue
            );

            let current_era = Self::current_era();

            // Update total staked value in era.
            let mut reward_and_stake_for_era =
                Self::era_reward_and_stake(current_era).ok_or(Error::<T>::UnexpectedState)?;
            reward_and_stake_for_era.staked += value_to_stake;
            EraRewardsAndStakes::<T>::insert(current_era, reward_and_stake_for_era);

            // Update ledger and payee
            Self::update_ledger(&staker, &ledger);

            // Update staked information for contract in current era
            if let Some(last_staked_era) = era_when_contract_last_staked.clone() {
                latest_era_staking_points.former_staked_era = last_staked_era;
            } else {
                latest_era_staking_points.former_staked_era = current_era;
            }
            ContractEraStake::<T>::insert(
                contract_id.clone(),
                current_era,
                latest_era_staking_points,
            );

            // If contract wasn't claimed nor staked yet, insert current era as last claimed era.
            // When calculating reward, this will provide correct information to the algorithm since nothing exists
            // for this contract prior to the current era.
            if !era_when_contract_last_staked.is_some() {
                ContractLastClaimed::<T>::insert(contract_id.clone(), current_era);
            }

            // Check if we need to update era in which contract was last changed. Can avoid one write.
            let contract_last_staked_change_needed =
                if let Some(previous_era) = era_when_contract_last_staked {
                    // if values aren't different, no reason to do another write
                    previous_era != current_era
                } else {
                    true
                };
            if contract_last_staked_change_needed {
                ContractLastStaked::<T>::insert(&contract_id, current_era);
            }

            Self::deposit_event(Event::<T>::BondAndStake(
                staker,
                contract_id,
                value_to_stake,
            ));

            Ok(().into())
        }

        /// Unbond, unstake and withdraw balance from the contract.
        ///
        /// Value will be unlocked for the user.
        ///
        /// In case remaining staked balance on contract is below minimum staking amount,
        /// entire stake for that contract will be unstaked.
        ///
        /// # <weight>
        /// TODO!
        /// </weight>
        #[pallet::weight(10_000_000)]
        pub fn unbond_unstake_and_withdraw(
            origin: OriginFor<T>,
            contract_id: SmartContract<T::AccountId>,
            #[pallet::compact] value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let staker = ensure_signed(origin)?;
            ensure!(
                RegisteredDapps::<T>::contains_key(&contract_id),
                Error::<T>::NotOperatedContract
            );
            ensure!(value > Zero::zero(), Error::<T>::UnstakingWithNoValue);

            // Get the latest era staking points for the contract.
            let era_when_contract_last_staked =
                Self::contract_last_staked(&contract_id).ok_or(Error::<T>::NotStakedContract)?;
            let mut era_staking_points = Self::contract_era_stake(&contract_id, &era_when_contract_last_staked).ok_or_else(|| {
                print("No era staking points for contract even though information exists that it was staked. This is a bug!");
                Error::<T>::UnexpectedState
            })?;

            // Ensure that the staker actually has this contract staked.
            let staked_value = *era_staking_points
                .stakers
                .get(&staker)
                .ok_or(Error::<T>::NotStakedContract)?;

            // Calculate the value which will be unstaked.
            let mut value_to_unstake = value.min(staked_value);
            let remaining_staked_value = staked_value.saturating_sub(value_to_unstake);
            if remaining_staked_value < T::MinimumStakingAmount::get() {
                // if staked value would fall below threshold, unstake everything
                era_staking_points.stakers.remove(&staker);
                value_to_unstake = staked_value;
            } else {
                era_staking_points
                    .stakers
                    .insert(staker.clone(), remaining_staked_value);
            }
            let value_to_unstake = value_to_unstake; // make it immutable
            era_staking_points.total = era_staking_points.total.saturating_sub(value_to_unstake);
            era_staking_points.former_staked_era = era_when_contract_last_staked;

            // Get the staking ledger and update it
            let mut ledger = Self::ledger(&staker).ok_or(Error::<T>::UnexpectedState)?;
            ledger.total = ledger.total.saturating_sub(value_to_unstake);
            ledger.active = ledger.active.saturating_sub(value_to_unstake);
            Self::update_ledger(&staker, &ledger);

            let current_era = Self::current_era();

            // Update the era staking points
            ContractEraStake::<T>::insert(contract_id.clone(), current_era, era_staking_points);

            // Update total staked value in era.
            let mut era_reward_and_stake =
                Self::era_reward_and_stake(current_era).ok_or(Error::<T>::UnexpectedState)?;
            era_reward_and_stake.staked =
                era_reward_and_stake.staked.saturating_sub(value_to_unstake);
            EraRewardsAndStakes::<T>::insert(current_era, era_reward_and_stake);

            // Check if we need to update era in which contract was last changed. Can avoid one write.
            if era_when_contract_last_staked != current_era {
                ContractLastStaked::<T>::insert(&contract_id, current_era);
            }

            Self::deposit_event(Event::<T>::UnbondUnstakeAndWithdraw(
                staker,
                contract_id,
                value_to_unstake,
            ));

            Ok(().into())
        }

        /// claim the rewards earned by contract_id.
        /// All stakers and developer for this contract will be paid out with single call.
        /// claim is valid for all unclaimed eras but not longer than history_depth().
        /// Any reward older than history_depth() will go to Treasury.
        /// Any user can call this function.
        #[pallet::weight(1_000_000)]
        pub fn claim(
            origin: OriginFor<T>,
            contract_id: SmartContract<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            let claimer = ensure_signed(origin)?;

            // check if this contract is registered
            let developer = Self::registered_developer(&contract_id)
                .ok_or(Error::<T>::ContractNotRegistered)?;

            // check if it was ever staked on this contract.
            let last_staked_era =
                Self::contract_last_staked(&contract_id).ok_or(Error::<T>::NothingToClaim)?;

            // check if the contract is already claimed in this era
            let current_era = Self::current_era();
            let last_claim_era =
                Self::contract_last_claimed(&contract_id).unwrap_or(current_era.clone());
            ensure!(
                current_era != last_claim_era,
                Error::<T>::AlreadyClaimedInThisEra
            );

            // oldest era to start with collecting rewards for devs and stakers
            let last_allowed_era = current_era.saturating_sub(Self::history_depth());

            // initialize rewards for stakers, developer and unclaimed rewards accumulator
            let mut rewards_for_stakers_map: BTreeMap<T::AccountId, BalanceOf<T>> =
                Default::default();
            let mut reward_for_developer: BalanceOf<T> = Zero::zero();
            let mut unclaimed_rewards: BalanceOf<T> = Zero::zero();

            // Next we iterate of periods between staking points.
            // Since each era staking point struct has information about the former era when staking information
            // was changed, we start from top and move to bottom.
            // E.g.:
            // [last_staked_era, current_era>,
            // [last_staked_era.former_stake_era, last_staked_era>,
            //  ...

            let mut lower_bound_era = last_staked_era;
            let mut upper_bound_era = current_era;
            let mut contract_staking_info =
                Self::contract_era_stake(&contract_id, &lower_bound_era)
                    .ok_or(Error::<T>::UnknownStartStakingData)?;
            loop {
                // accumulate rewards for this period
                for era in lower_bound_era..upper_bound_era {
                    let reward_and_stake_for_era =
                        Self::era_reward_and_stake(era).ok_or(Error::<T>::UnknownEraReward)?;

                    // Calculate the contract reward for this era.
                    let reward_particle = Perbill::from_rational(
                        contract_staking_info.total,
                        reward_and_stake_for_era.staked,
                    );
                    let contract_reward_in_era = reward_particle * reward_and_stake_for_era.rewards;

                    // First arm refers to situations where both dev and staker are eligible for rewards
                    if era >= last_allowed_era {
                        // divide reward between stakers and the developer of the contract
                        let contract_staker_reward = Perbill::from_rational(
                            (100 - T::DeveloperRewardPercentage::get()) as u64,
                            100,
                        ) * contract_reward_in_era;
                        let contract_developer_reward =
                            Perbill::from_rational(T::DeveloperRewardPercentage::get() as u64, 100)
                                * contract_reward_in_era;

                        // accumulate rewards for the stakers
                        Self::stakers_era_reward(
                            &mut rewards_for_stakers_map,
                            &contract_staking_info,
                            contract_staker_reward,
                        );
                        // accumulate rewards for the developer
                        reward_for_developer += contract_developer_reward;
                    } else {
                        // This arm refers to situations where dev and staker are 'penalized' since they didn't collect rewards in time.
                        unclaimed_rewards += contract_reward_in_era;
                    }
                }
                upper_bound_era = lower_bound_era;
                lower_bound_era = contract_staking_info.former_staked_era;

                // Check if this is the last unprocessed era staking point. If it is, stop.
                if lower_bound_era == upper_bound_era {
                    // update struct so it reflects that it's the last known staking point value
                    contract_staking_info.former_staked_era = current_era;
                    break;
                }


                contract_staking_info = Self::contract_era_stake(&contract_id, &lower_bound_era)
                    .ok_or(Error::<T>::UnknownStartStakingData)?;
            }

            // send rewards to stakers
            Self::payout_stakers(&rewards_for_stakers_map);
            // send rewards to developer
            T::Currency::deposit_into_existing(&developer, reward_for_developer).ok();
            // if !unclaimed_rewards.is_zero() { TODO!
            //     T::Currency::deposit_into_existing(&treasury, unclaimed_rewards).ok();
            // }

            // Remove all previous records of staking for this contract,
            // they have already been processed and won't be needed anymore.
            ContractEraStake::<T>::remove_prefix(&contract_id, None);
            // create contract era stake data in current era for further staking and claiming
            ContractEraStake::<T>::insert(&contract_id, current_era, contract_staking_info);

            // move contract pointers to current era
            ContractLastClaimed::<T>::insert(&contract_id, current_era);
            ContractLastStaked::<T>::insert(&contract_id, current_era);

            Self::deposit_event(Event::<T>::ContractClaimed(
                contract_id,
                claimer,
                last_claim_era.max(last_allowed_era),
                current_era,
            ));

            Ok(().into())
        }

        // =============== Era ==================

        /// Force there to be no new eras indefinitely.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// Thus the election process may be ongoing when this is called. In this case the
        /// election will continue until the next era is triggered.
        ///
        /// # <weight>
        /// - No arguments.
        /// - Weight: O(1)
        /// - Write: ForceEra
        /// # </weight>
        #[pallet::weight(T::WeightInfo::force_no_eras())]
        pub fn force_no_eras(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceNone);
            Ok(())
        }

        /// Force there to be a new era at the end of the next session. After this, it will be
        /// reset to normal (non-forced) behaviour.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// If this is called just before a new era is triggered, the election process may not
        /// have enough blocks to get a result.
        ///
        /// # <weight>
        /// - No arguments.
        /// - Weight: O(1)
        /// - Write ForceEra
        /// # </weight>
        #[pallet::weight(T::WeightInfo::force_new_era())]
        pub fn force_new_era(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceNew);
            Ok(())
        }

        /// Force there to be a new era at the end of blocks indefinitely.
        ///
        /// The dispatch origin must be Root.
        ///
        /// # Warning
        ///
        /// The election process starts multiple blocks before the end of the era.
        /// If this is called just before a new era is triggered, the election process may not
        /// have enough blocks to get a result.
        ///
        /// # <weight>
        /// - Weight: O(1)
        /// - Write: ForceEra
        /// # </weight>
        #[pallet::weight(T::WeightInfo::force_new_era_always())]
        pub fn force_new_era_always(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceAlways);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Update the ledger for a staker. This will also update the stash lock.
        /// This lock will lock the entire funds except paying for further transactions.
        fn update_ledger(staker: &T::AccountId, ledger: &StakingLedger<BalanceOf<T>>) {
            if ledger.active.is_zero() {
                Ledger::<T>::remove(&staker);
                T::Currency::remove_lock(STAKING_ID, &staker);
            } else {
                T::Currency::set_lock(STAKING_ID, &staker, ledger.total, WithdrawReasons::all());
                Ledger::<T>::insert(staker, ledger);
            }
        }

        /// Checks if there is a valid smart contract for the provided address
        fn is_contract_valid(address: &SmartContract<T::AccountId>) -> bool {
            match address {
                SmartContract::Wasm(_account) => {
                    //     <pallet_contracts::ContractInfoOf<T>>::get(&account).is_some()
                    false
                }
                SmartContract::Evm(_account) => {
                    // pallet_evm::Module::<T>::account_codes(&account).len() > 0 TODO remove comment after EVM mege
                    true
                }
            }
        }

        /// Calculate rewards for all stakers for this era
        fn stakers_era_reward(
            staker_map: &mut BTreeMap<T::AccountId, BalanceOf<T>>,
            points: &EraStakingPoints<T::AccountId, BalanceOf<T>>,
            reward_for_contract: BalanceOf<T>,
        ) {
            let staker_part = Perbill::from_rational(reward_for_contract, (*points).total);

            for (s, b) in &points.stakers {
                let reward = staker_map.entry(s.clone()).or_insert(Default::default());
                *reward += staker_part * *b;
            }
        }

        /// Execute payout for stakers
        fn payout_stakers(staker_map: &BTreeMap<T::AccountId, BalanceOf<T>>) {
            for (s, b) in staker_map {
                T::Currency::deposit_into_existing(&s, *b).ok();
            }
        }

        /// The block rewards are accumulated on the pallets's account during an era.
        /// This function takes a snapshot of the pallet's balance accured during current era
        /// and stores it for future distribution
        ///
        /// This is called just at the beginning of an era.
        fn reward_balance_snapshoot(ending_era: EraIndex) {
            let reward = Perbill::from_percent(T::DAppsRewardPercentage::get())
                * Self::block_reward_accumulator();

            // Get the reward and stake information for previous era
            let mut reward_and_stake = Self::era_reward_and_stake(ending_era).unwrap_or_default();

            // Prepare info for the next era
            EraRewardsAndStakes::<T>::insert(
                ending_era + 1,
                EraRewardAndStake {
                    rewards: Zero::zero(),
                    staked: reward_and_stake.staked.clone(),
                },
            );

            // Set the reward for the previous era.
            reward_and_stake.rewards = reward;
            EraRewardsAndStakes::<T>::insert(ending_era, reward_and_stake);
        }
    }
}
