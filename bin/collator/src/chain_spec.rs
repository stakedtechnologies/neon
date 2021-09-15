//! Chain specifications.

use cumulus_primitives_core::ParaId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use shibuya_runtime::{
    wasm_binary_unwrap, AccountId, AuraConfig, AuraId, Balance, BalancesConfig,
    CollatorSelectionConfig, EVMConfig, GenesisConfig, ParachainInfoConfig, SessionConfig,
    SessionKeys, ShibuyaNetworkPrecompiles, Signature, SudoConfig, SystemConfig, VestingConfig,
    SDN,
};
use sp_core::{sr25519, Pair, Public};

use sp_runtime::traits::{IdentifyAccount, Verify};

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

/// Specialized `ChainSpec` for Shiden Network.
pub type ShidenChainSpec = sc_service::GenericChainSpec<shiden_runtime::GenesisConfig, Extensions>;

/// Specialized `ChainSpec` for Shibuya testnet.
pub type ShibuyaChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(aura: AuraId) -> SessionKeys {
    SessionKeys { aura }
}

/// Gen Shibuya chain specification for given parachain id.
pub fn get_chain_spec(para_id: u32) -> ShibuyaChainSpec {
    // Alice as default
    let sudo_key = get_account_id_from_seed::<sr25519::Public>("Alice");

    ShibuyaChainSpec::from_genesis(
        "Shibuya Testnet",
        "shibuya",
        ChainType::Live,
        move || {
            make_genesis(
                crate::balances::SHIDEN_HOLDERS.clone(),
                sudo_key.clone(),
                para_id.into(),
            )
        },
        vec![],
        None,
        None,
        None,
        Extensions {
            relay_chain: "tokyo".into(),
            para_id,
        },
    )
}

/// Helper function to create GenesisConfig.
fn make_genesis(
    balances: Vec<(AccountId, Balance)>,
    root_key: AccountId,
    parachain_id: ParaId,
) -> GenesisConfig {
    use sp_core::crypto::Ss58Codec;
    let authorities = vec![
        (
            AccountId::from_ss58check("5HbAP8GczDDfGL6K2BvbsDyCyL3qY2GSRrJAPNXSUnd95mRM").unwrap(),
            get_from_seed::<AuraId>("Alice"),
        ),
        (
            AccountId::from_ss58check("5Fhdbkg89StmVoMNrbukWpcD7ZgLc9gBYNyPBDi3zokPWC48").unwrap(),
            get_from_seed::<AuraId>("Bob"),
        ),
    ];

    // This is supposed the be the simplest bytecode to revert without returning any data.
    // We will pre-deploy it under all of our precompiles to ensure they can be called from
    // within contracts.
    // (PUSH1 0x00 PUSH1 0x00 REVERT)
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        sudo: SudoConfig { key: root_key },
        parachain_info: ParachainInfoConfig { parachain_id },
        balances: BalancesConfig { balances },
        vesting: VestingConfig { vesting: vec![] },
        session: SessionConfig {
            keys: authorities
                .iter()
                .map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone())))
                .collect::<Vec<_>>(),
        },
        aura: AuraConfig {
            authorities: vec![],
        },
        aura_ext: Default::default(),
        collator_selection: CollatorSelectionConfig {
            desired_candidates: 200,
            candidacy_bond: 32_000 * SDN,
            invulnerables: vec![],
        },
        evm: EVMConfig {
            // We need _some_ code inserted at the precompile address so that
            // the evm will actually call the address.
            accounts: ShibuyaNetworkPrecompiles::<()>::used_addresses()
                .map(|addr| {
                    (
                        addr,
                        pallet_evm::GenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: revert_bytecode.clone(),
                        },
                    )
                })
                .collect(),
        },
        ethereum: Default::default(),
    }
}

/// Development config
pub fn development_config() -> ShibuyaChainSpec {
    ShibuyaChainSpec::from_genesis(
        "Shibuya Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn development_config_genesis() -> GenesisConfig {
    const ENDOWMENT: Balance = 5_000_000 * SDN;
    let balances: Vec<(AccountId, Balance)> = {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        ]
        .iter()
        .cloned()
        .map(|acc| (acc, ENDOWMENT))
        .collect()
    };
    let sudo_key = get_account_id_from_seed::<sr25519::Public>("Alice");
    let para_id = 1000;

    make_genesis(balances, sudo_key, para_id.into())
}
