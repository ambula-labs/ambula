//! Helper module to build a genesis configuration for the super-runtime

use super::{AccountId, AuthorityDiscoveryId, BalancesConfig, GenesisConfig, SessionKeys, Signature, SudoConfig, SystemConfig, SessionConfig, AuthorityDiscoveryConfig};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}


fn session_keys(authority_discovery: AuthorityDiscoveryId) -> SessionKeys {
	SessionKeys { authority_discovery }
}


pub fn dev_genesis(wasm_binary: &[u8]) -> GenesisConfig {
	testnet_genesis(
		wasm_binary,
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		// Root Key
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		// Endowed Accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Mike"),
			get_account_id_from_seed::<sr25519::Public>("John"),
		],
	)
}

/// Helper function to build a genesis configuration
pub fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
			.iter()
			.map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(x.1.clone()))
			}).collect::<Vec<_>>(),
		}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		pallet_sudo: Some(SudoConfig { key: root_key }),
	}
}
