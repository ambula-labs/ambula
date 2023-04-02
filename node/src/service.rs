//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use async_trait::async_trait;
use futures::{executor::block_on, StreamExt};
use node_template_runtime::{self, opaque::Block, RuntimeApi};
use pow::*;
pub use sc_executor::NativeElseWasmExecutor;
use sc_keystore::LocalKeystore;
use sc_network::{Event, NetworkEventStream};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_api::{Encode, ProvideRuntimeApi};
use sp_authority_discovery::AuthorityDiscoveryApi;
use sp_core::{sr25519, U256, H512};
use sp_keystore::SyncCryptoStore;
use sp_runtime::{
	key_types::AUTHORITY_DISCOVERY as AUTHORITY_DISCOVERY_KEY_TYPE, traits::Block as BlockT,
};
use std::{sync::Arc, thread, time::Duration};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		node_template_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		node_template_runtime::native_version()
	}
}

pub(crate) type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub struct CreateInherentDataProviders;

#[async_trait]
impl sp_inherents::CreateInherentDataProviders<Block, ()> for CreateInherentDataProviders {
	type InherentDataProviders = sp_timestamp::InherentDataProvider;

	async fn create_inherent_data_providers(
		&self,
		_parent: <Block as BlockT>::Hash,
		_extra_args: (),
	) -> Result<Self::InherentDataProviders, Box<dyn std::error::Error + Send + Sync>> {
		Ok(sp_timestamp::InherentDataProvider::from_system_time())
	}
}

pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_consensus_pow::PowBlockImport<
				Block,
				Arc<FullClient>,
				FullClient,
				FullSelectChain,
				MinimalSha3Algorithm<FullClient>,
				CreateInherentDataProviders,
			>,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other("Remote Keystores are not supported.".into()))
	}

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	// let can_author_with =
	// sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

	let pow_algorithm = pow::MinimalSha3Algorithm::new(client.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		pow_algorithm.clone(),
		0, // check inherents starting at block 0
		select_chain.clone(),
		CreateInherentDataProviders,
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import.clone()),
		None,
		pow_algorithm.clone(),
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
	)?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		import_queue,
		keystore_container,
		task_manager,
		transaction_pool,
		select_chain,
		other: (pow_block_import, telemetry),
	})
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	let sc_service::PartialComponents {
		client,
		backend,
		import_queue,
		mut keystore_container,
		mut task_manager,
		transaction_pool,
		select_chain,
		other: (pow_block_import, mut telemetry),
	} = new_partial(&config)?;

	if let Some(url) = &config.keystore_remote {
		match remote_keystore(url) {
			Ok(k) => keystore_container.set_remote_keystore(k),
			Err(e) =>
				return Err(ServiceError::Other(format!(
					"Error hooking up remote keystore for {}: {}",
					url, e
				))),
		};
	}

	let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();

		Box::new(move |deny_unsafe, _| {
			let deps =
				crate::rpc::FullDeps { client: client.clone(), pool: pool.clone(), deny_unsafe };
			crate::rpc::create_full(deps).map_err(Into::into)
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_builder: rpc_extensions_builder,
		backend,
		system_rpc_tx,
		tx_handler_controller,
		sync_service: sync_service.clone(),
		config,
		telemetry: telemetry.as_mut(),
	})?;

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let dht_event_stream =
			network.event_stream("authority-discovery").filter_map(|e| async move {
				match e {
					Event::Dht(e) => Some(e),
					_ => None,
				}
			});

		let (mut _discovery_worker, mut _discovery_service) =
			sc_authority_discovery::new_worker_and_service(
				client.clone(),
				network.clone(),
				Box::pin(dht_event_stream),
				sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore()),
				None,
			);

		task_manager.spawn_essential_handle().spawn_blocking(
			"authority_discovery",
			Some("peer-discovery"),
			Box::pin(_discovery_worker.run()),
		);

		let pow_algorithm = pow::MinimalSha3Algorithm::new(client.clone());

		let (_worker, worker_task) = sc_consensus_pow::start_mining_worker(
			Box::new(pow_block_import),
			client.clone(),
			select_chain,
			pow_algorithm.clone(),
			proposer_factory,
			sync_service.clone(),
			sync_service.clone(),
			None,
			CreateInherentDataProviders,
			// time to wait for a new block before starting to mine a new one
			Duration::from_secs(10),
			// how long to take to actually build the block (i.e. executing extrinsics)
			Duration::from_secs(10),
		);

		task_manager.spawn_essential_handle().spawn_blocking(
			"pow",
			Some("block-authoring"),
			worker_task,
		);

		// Get node authority-discovery public session key from keystore
		let authority_discovery_pubkey: Vec<sr25519::Public> = SyncCryptoStore::sr25519_public_keys(
			&*keystore_container.sync_keystore(),
			AUTHORITY_DISCOVERY_KEY_TYPE,
		)
		.iter()
		.map(|k| sr25519::Public::from(k.clone()))
		.collect();
		
		// Use authority-discovery session key to sign a message (should use a different ECDSA session key KEY_TYPE instead)
		let signature = SyncCryptoStore::sign_with(
			&*keystore_container.sync_keystore(),
			AUTHORITY_DISCOVERY_KEY_TYPE,
			&authority_discovery_pubkey[0].into(),
			"My Signed Message".as_bytes()
		).unwrap();

		match signature {
			Some(sig) => println!("Signature: {:?}", H512::from_slice(&sig)),
			_ => {},
		};

		// Start Mining
		let mut nonce: U256 = U256::from(0);
		thread::spawn(move || loop {
			let worker = _worker.clone();
			let metadata = worker.metadata();

			if let Some(metadata) = metadata {
				// Get the list of authorities from autority-discovery pallet at a specific bloc
				let mut authorities =
					client.clone().runtime_api().authorities(metadata.best_hash).unwrap();

				let authorities_len = authorities.len();

				// Sort authorities to have the same order accross nodes
				authorities.sort();

				// Print number of authorities
				println!("[Authorities] (length = {})", authorities_len);

				// Print all authorities libp2p multiaddr with local node filtered out
				for (i, authority) in authorities.iter().filter(|&x| *x != authority_discovery_pubkey[0].into()).enumerate() {
					println!(
						"Authority {} : {:?} - {:?}",
						i,
						block_on(
							_discovery_service.get_addresses_by_authority_id(authority.clone())
						),
						authority.clone()
					);
				}

				let compute =
					Compute { difficulty: metadata.difficulty, pre_hash: metadata.pre_hash, nonce };
				let seal = compute.compute();
				if hash_meets_difficulty(&seal.work, seal.difficulty) {
					nonce = U256::from(0);
					block_on(worker.submit(seal.encode()));
				} else {
					nonce = nonce.saturating_add(U256::from(1));
					if nonce == U256::MAX {
						nonce = U256::from(0);
					}
				}
				thread::sleep(Duration::new(1, 0));
			} else {
				thread::sleep(Duration::new(1, 0));
			}
		});
	}

	network_starter.start_network();
	Ok(task_manager)
}
