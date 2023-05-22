pub use pallet_template_runtime_api::TemplateApi as TemplateRuntimeApi;
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_core::{sr25519, H512};
use sp_runtime::{key_types::AUTHORITY_DISCOVERY as AUTHORITY_DISCOVERY_KEY_TYPE};

#[rpc(client, server)]
pub trait TemplateApi<BlockHash> {
	#[method(name = "template_getValue")]
	fn get_value(&self, at: Option<BlockHash>) -> RpcResult<u32>;

	#[method(name = "sign")]
	fn sign(&self, msg: String) -> RpcResult<String>;
}

/// A struct that implements the `TemplateApi`.
pub struct TemplatePallet<C, Block> {
	// If you have more generics, no need to TemplatePallet<C, M, N, P, ...>
	// just use a tuple like TemplatePallet<C, (M, N, P, ...)>
	client: Arc<C>,
	keystore: SyncCryptoStorePtr,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TemplatePallet<C, Block> {
	/// Create new `TemplatePallet` instance with the given reference to the client.
	pub fn new(client: Arc<C>, keystore: SyncCryptoStorePtr) -> Self {
		Self { client, keystore, _marker: Default::default() }
	}
}

impl<C, Block> TemplateApiServer<<Block as BlockT>::Hash> for TemplatePallet<C, Block>
where
Block: BlockT,
C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
C::Api: TemplateRuntimeApi<Block>,
{
	fn get_value(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
		let api = self.client.runtime_api();
		// let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));
		let at = at.unwrap_or_else(||self.client.info().best_hash);

		api.get_value(at).map_err(runtime_error_into_rpc_err)
	}

	fn sign(&self, msg: String) -> RpcResult<String> {
		// Get node authority-discovery public session key from keystore
		let authority_discovery_pubkey: Vec<sr25519::Public> = SyncCryptoStore::sr25519_public_keys(
			&*self.keystore,
			AUTHORITY_DISCOVERY_KEY_TYPE,
		)
		.iter()
		.map(|k| sr25519::Public::from(k.clone()))
		.collect();

		if authority_discovery_pubkey.is_empty(){
			return Err(runtime_error_into_rpc_err("The list of public keys is empty"))
		}

		// Use authority-discovery session key to sign a message (should use a different ECDSA session key KEY_TYPE instead)
		let signature = SyncCryptoStore::sign_with(
			&*self.keystore,
			AUTHORITY_DISCOVERY_KEY_TYPE,
			&authority_discovery_pubkey[0].into(),
			msg.as_bytes()
		).map_err(runtime_error_into_rpc_err);

		match signature {
			Ok(Some(sig)) => Ok(std::format!("{:?}", H512::from_slice(&sig))),
			_ => Err(runtime_error_into_rpc_err("Couldn't sign message"))
		}
	}
}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Runtime error",
		Some(format!("{:?}", err)),
	))
	.into()
}
