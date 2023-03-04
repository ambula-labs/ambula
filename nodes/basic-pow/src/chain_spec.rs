use runtime::{
	genesis::{get_account_id_from_seed, dev_genesis, testnet_genesis, authority_keys_from_seed},
	GenesisConfig, WASM_BINARY,
};
use sp_core::sr25519;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate `ChainSpec` type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn dev_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		sc_service::ChainType::Development,
		move || dev_genesis(wasm_binary),
		vec![],
		None,
		None,
		None,
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		sc_service::ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Root Key
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
			)
		},
		vec![],
		None,
		None,
		None,
		None,
	))
}
