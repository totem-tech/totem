// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus and Totem Accounting.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

// Respecting the above licence Totem Accounting has made some code enhancements for its parachains.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
// use rococo_parachain_runtime::{AccountId, AuraId, Signature};
use lego_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use wapex_runtime::*;
use kapex_runtime::*;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// use lego_runtime::FundingConfig;
// use wapex_runtime::FundingConfig;
// use kapex_runtime::FundingConfig;

/// Specialized `ChainSpec` for the normal parachain runtime.
// pub type ChainSpec = sc_service::GenericChainSpec<rococo_parachain_runtime::GenesisConfig, Extensions>;

/// Totem Parachain Generator
pub type LegoChainSpec = sc_service::GenericChainSpec<lego_runtime::GenesisConfig, Extensions>;
pub type WapexChainSpec = sc_service::GenericChainSpec<wapex_runtime::GenesisConfig, Extensions>;
pub type KapexChainSpec = sc_service::GenericChainSpec<kapex_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
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

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

pub fn get_chain_spec() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions { relay_chain: "westend".into(), para_id: 1000 },
	)
}

pub fn get_lego_chain_spec() -> LegoChainSpec {
	LegoChainSpec::from_genesis(
		"Lego Parachain",
		"lego",
		ChainType::Local,
		move || lego_genesis(2000.into()),
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions { relay_chain: "rococo".into(), para_id: 2000 },
	)
}

pub fn get_wapex_chain_spec() -> WapexChainSpec {
	WapexChainSpec::from_genesis(
		"Wapex Parachain",
		"wapex",
		ChainType::Local,
		move || wapex_genesis(2107.into()),
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions { relay_chain: "westend".into(), para_id: 2107 },
	)
}

pub fn get_kapex_chain_spec() -> KapexChainSpec {
	KapexChainSpec::from_genesis(
		"Kapex Parachain",
		"kapex",
		ChainType::Live,
		move || kapex_genesis(2007.into()),
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions { relay_chain: "polkadot".into(), para_id: 2007 },
	)
}

pub fn staging_test_net() -> ChainSpec {
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
				vec![
					// $secret//one
					hex!["aad9fa2249f87a210a0f93400b7f90e47b810c6d65caa0ca3f5af982904c2a33"]
						.unchecked_into(),
					// $secret//two
					hex!["d47753f0cca9dd8da00c70e82ec4fc5501a69c49a5952a643d18802837c88212"]
						.unchecked_into(),
				],
				vec![
					hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into()
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions { relay_chain: "westend".into(), para_id: 1000 },
	)
}
// for the staging test net
// fn testnet_genesis(
// 	root_key: AccountId,
// 	initial_authorities: Vec<AuraId>,
// 	endowed_accounts: Vec<AccountId>,
// 	id: ParaId,
// ) -> rococo_parachain_runtime::GenesisConfig {
// 	rococo_parachain_runtime::GenesisConfig {
// 		system: rococo_parachain_runtime::SystemConfig {
// 			code: rococo_parachain_runtime::WASM_BINARY
// 				.expect("WASM binary was not build, please build it!")
// 				.to_vec(),
// 		},
// 		balances: rococo_parachain_runtime::BalancesConfig {
// 			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
// 		},
// 		sudo: rococo_parachain_runtime::SudoConfig { key: Some(root_key) },
// 		parachain_info: rococo_parachain_runtime::ParachainInfoConfig { parachain_id: id },
// 		aura: rococo_parachain_runtime::AuraConfig { authorities: initial_authorities },
// 		aura_ext: Default::default(),
// 		parachain_system: Default::default(),
// 	}
// }

fn lego_testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> lego_runtime::GenesisConfig {
	lego_runtime::GenesisConfig {
		system: lego_runtime::SystemConfig {
			code: lego_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: lego_runtime:BalancesConfig {
			balances: endowed_accounts
			.iter()
			.cloned()
			.map()
			.collect(),
		},
		sudo: parachain_totem_lego_runtime::SudoConfig { key: Some(root_key) },
		parachain_info: parachain_totem_lego_runtime::ParachainInfoConfig { parachain_id: id },
		aura: parachain_totem_lego_runtime::AuraConfig { authorities: initial_authorities },
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn lego_session_keys(keys: AuraId) -> lego_runtime::SessionKeys {
	lego_runtime::SessionKeys { aura: keys }
}

pub fn lego_development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 2007.into());

	ChainSpec::from_genesis(
		// Name
		"Lego Development",
		// ID
		"lego_dev",
		ChainType::Development,
		move || {
			lego_testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("lego-dev"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn lego_local_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 2007.into());

	ChainSpec::from_genesis(
		// Name
		"Lego Local Testnet",
		// ID
		"lego_local_testnet",
		ChainType::Local,
		move || {
			lego_testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("lego-local"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn lego_config() -> LegoChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2007.into());
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());

	LegoChainSpec::from_genesis(
		// Name
		"Lego",
		// ID
		"lego",
		ChainType::Live,
		move || {
			lego_genesis(
				// initial collators.
				vec![
					(
						"5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA".into(),
						"5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA".into(),
					),
					(
						"5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM".into(),
						"5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM".into(),
					),
				],
				// Endowed accounts
				vec![
					[
					  "5CcEDRg5DpwhLs8e2gn1A9Yjv9pRKTbgadkYtur7WH6hMSZJ".into(),
					  2360000000000000000.into(),
					],
					[
					  "5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA".into(),
					  20000000000000000.into(),
					],
					[
					  "5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM".into(),
					  20000000000000000.into(),
					]
				  ],
				// Sudo
				"5CcEDRg5DpwhLs8e2gn1A9Yjv9pRKTbgadkYtur7WH6hMSZJ".into(),	
				//ParaId
				2000u32.into(),
			)
		},
		// Bootnodes
		vec![
			"/ip4/159.89.1.153/tcp/31436/ws/p2p/12D3KooWCwPGSVmQSLTRMgaU1GaK6oHXijKdFcMiCBzzKygxVcXL".parse().unwrap(),
			"/dns4/l-boot-1.kapex.network/tcp/31436/ws/p2p/12D3KooWNpTmqisnMwdAUtjRHCxdb6EQcymdwQvdQFNuFfgBX1K3".parse().unwrap(),
		],
		// Telemetry
		None,
		// Protocol ID
		Some("lego"),
		// Properties
		Some(properties),
		// Extensions
		Extensions { relay_chain: "rococo-local".into(), para_id: 2000u32 },
	)
}

fn lego_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
	root_key: AccountId,
) -> lego_runtime::GenesisConfig {
	lego_runtime::GenesisConfig {
		system: lego_runtime::SystemConfig {
			code: lego_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: lego_runtime::BalancesConfig {
			balances: endowed_accounts
			.iter()
			.cloned()
			// .map(|k| (k, 1 << 60))
			.map()
			.collect(),
		},
		sudo: lego_runtime::SudoConfig { key: root_key },
		parachain_info: lego_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: lego_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: 1_600_000,
			..Default::default()
		},
		session: lego_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                  // account id
						acc,                          // validator id
						lego_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		// funding: FundingConfig::default(),
	}
}
