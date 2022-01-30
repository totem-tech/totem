use cumulus_primitives_core::ParaId;
use lego_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT, Balance};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public, crypto::{Ss58Codec}};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type LegoChainSpec =
	sc_service::GenericChainSpec<lego_runtime::GenesisConfig, Extensions>;

	
	
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

// Parity

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

// Totem

/// Helper function to convert a string containing an SS58 address to an account ID.
/// Assumes that it is already correctly formatted for ss58 Address types have been included in this file.
/// ****** Will panic if not *********
pub fn get_account_id_from_public(ss58_addr: &str) -> AccountId
{
	match Ss58Codec::from_ss58check(ss58_addr)
	{
		Ok(pubkey) => return pubkey,
		Err(_) => panic!("Invalid address"),
	}
}

/// Generate collator keys from AccountId.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_public(ss58_addr: &str) -> AuraId {
	match Ss58Codec::from_ss58check(ss58_addr)
	{
		Ok(pubkey) => return pubkey,
		Err(_) => panic!("Invalid address"),
	}
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn lego_session_keys(keys: AuraId) -> lego_runtime::SessionKeys {
	lego_runtime::SessionKeys { aura: keys }
}

pub fn lego_development_config() -> LegoChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	LegoChainSpec::from_genesis(
		// Name
		"Lego Development",
		// ID
		"lego-dev",
		ChainType::Development,
		move || {
			lego_genesis(
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
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						1000000000000
					),
				],
				1000.into(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		Vec::new(),
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

pub fn lego_local_config() -> LegoChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	LegoChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			lego_genesis(
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
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						1000000000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						1000000000000
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						1000000000000
					),
				],
				1000.into(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		Vec::new(),
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
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LEGO".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	LegoChainSpec::from_genesis(
		// Name
		"Lego Parachain",
		// ID
		"lego_parachain",
		ChainType::Live,
		move || {
			lego_genesis(
				// initial collators/validators/authorities.
				vec![
					(
						get_account_id_from_public("5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA"),
						get_collator_keys_from_public("5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA"),
					),
					(
						get_account_id_from_public("5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM"),
						get_collator_keys_from_public("5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM"),
					),
				],
				vec![
					(
						get_account_id_from_public("5CcEDRg5DpwhLs8e2gn1A9Yjv9pRKTbgadkYtur7WH6hMSZJ"),
						2360000000000000000
					),
					(
						get_account_id_from_public("5Fxc4SWjmAnJgQtDosmytWevznn2LMDjv2s7AUJ8EeU4CqYA"),
						20000000000000000
					),
					(
						get_account_id_from_public("5F2NU9EgK2tas9yj4ATui2hxoMvLB4Ge8Ws4GWDb7ryHirjM"),
						20000000000000000
					)	
				],
				2000.into(),
				get_account_id_from_public("5CcEDRg5DpwhLs8e2gn1A9Yjv9pRKTbgadkYtur7WH6hMSZJ"),
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
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

fn lego_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
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
			balances: endowed_accounts.iter().cloned().map(|(k, v)| (k, v)).collect(),
		},
		parachain_info: lego_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: lego_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: lego_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
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
		sudo: lego_runtime::SudoConfig { key: root_key },
	}
}
