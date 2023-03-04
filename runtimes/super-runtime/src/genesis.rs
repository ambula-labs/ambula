//! Helper module to build a genesis configuration for the super-runtime

use super::{AccountId, AuthorityDiscoveryId, BalancesConfig, GenesisConfig, Signature, SudoConfig, SystemConfig, SessionConfig, AuthorityDiscoveryConfig};
use sp_core::{sr25519, Pair};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPair: Pair>(seed: &str) -> TPair::Public {
	TPair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn account_id_from_seed<TPair: Pair>(seed: &str) -> AccountId
where
	AccountPublic: From<TPair::Public>,
{
	AccountPublic::from(get_from_seed::<TPair>(seed)).into_account()
}

fn session_keys(authority_discovery: AuthorityDiscoveryId) -> SessionKeys {
	SessionKeys { authority_discovery }
}

pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuthorityDiscoveryId) {
	(
		account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn dev_genesis(wasm_binary: &[u8]) -> GenesisConfig {
	testnet_genesis(
		wasm_binary,
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		// Root Key
		account_id_from_seed::<sr25519::Pair>("Alice"),
		// Endowed Accounts
		vec![
			account_id_from_seed::<sr25519::Pair>("Alice"),
			account_id_from_seed::<sr25519::Pair>("Bob"),
			account_id_from_seed::<sr25519::Pair>("Mike"),
			account_id_from_seed::<sr25519::Pair>("John"),
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
