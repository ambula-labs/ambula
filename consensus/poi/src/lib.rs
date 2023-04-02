use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use parity_scale_codec::{Decode, Encode};
use sc_consensus_pow::{Error, PowAlgorithm};
use sha3::{Digest, Sha3_256};
use sp_api::ProvideRuntimeApi;
use sp_consensus_pow::{DifficultyApi, Seal as RawSeal};
use sp_authority_discovery::{AuthorityDiscoveryApi, AuthorityId};
use sp_core::{H256, U256};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

/// Determine whether the given hash satisfies the given difficulty.
/// The test is done by multiplying the two together. If the product
/// overflows the bounds of U256, then the product (and thus the hash)
/// was too high.
pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
	let num_hash = U256::from(&hash[..]);
	let (_, overflowed) = num_hash.overflowing_mul(difficulty);

	!overflowed
}

/// A Seal struct that will be encoded to a Vec<u8> as used as the
/// `RawSeal` type.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Seal {
	pub difficulty: U256,
	pub signature: H256,
	pub peer: U256,
}

/// A not-yet-computed attempt to solve the proof of work. Calling the
/// compute method will compute the hash and return the seal.
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Interact {
	pub message: H256,
	pub peer: H256,
}

fn _client(url: &str) -> Result<Client, simple_http::Error> {
	let t = SimpleHttpTransport::builder().url(url)?.build();

	Ok(Client::with_transport(t))
}

impl Interact {
	pub fn compute<C, B: BlockT<Hash = H256>>(self, sc_client: Arc<C>, at: B::Hash) -> Seal
	where
		C: ProvideRuntimeApi<B>,
		C::Api: AuthorityDiscoveryApi<B>,
	{
		let work = H256::from_slice(Sha3_256::digest(&self.encode()[..]).as_slice());

		let authorities = sc_client
			.runtime_api().authorities(at).unwrap();

		// println!("PoW authorities: {:?}", authorities[0]);

		// let client =
		// 	_client("http://api.random.org/json-rpc/1/invoke").expect("failed to create client");
		// let request = client.build_request("uptime", &[]);
		// let response = client.send_request(request).expect("send_request failed");

		// // For other commands this would be a struct matching the returned json.
		// let result: u64 = response
		// 	.result()
		// 	.expect("response is an error, use check_error");

		// println!("bitcoind uptime: {}", result);

		Seal {
			nonce: self.nonce,
			difficulty: self.difficulty,
			work,
			authorities,
		}
	}
}

/// A minimal PoW algorithm that uses Sha3 hashing.
/// Difficulty is fixed at 1_000_000
pub struct MinimalSha3Algorithm<C> {
	client: Arc<C>,
}

impl<C> MinimalSha3Algorithm<C> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

// Manually implement clone. Deriving doesn't work because
// it'll derive impl<C: Clone> Clone for Sha3Algorithm<C>. But C in practice isn't Clone.
impl<C> Clone for MinimalSha3Algorithm<C> {
	fn clone(&self) -> Self {
		Self::new(self.client.clone())
	}
}

// Here we implement the general PowAlgorithm trait for our concrete Sha3Algorithm
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for MinimalSha3Algorithm<C> {
	type Difficulty = U256;

	fn difficulty(&self, _parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
		// Fixed difficulty hardcoded here
		Ok(U256::from(100))
	}

	fn verify(
		&self,
		_parent: &BlockId<B>,
		_pre_hash: &H256,
		_pre_digest: Option<&[u8]>,
		seal: &RawSeal,
		difficulty: Self::Difficulty,
	) -> Result<bool, Error<B>> {
		// Try to construct a seal object by decoding the raw seal given
		let seal = match Seal::decode(&mut &seal[..]) {
			Ok(seal) => seal,
			Err(_) => return Ok(false),
		};

		// See whether the hash meets the difficulty requirement. If not, fail fast.
		if !hash_meets_difficulty(&seal.work, difficulty) {
			return Ok(false);
		}

		// Make sure the provided work actually comes from the correct pre_hash
		// let compute = Compute {
		// 	difficulty,
		// 	pre_hash: *pre_hash,
		// 	nonce: seal.nonce,
		// };

		// if compute.compute().await != seal {
		// 	return Ok(false);
		// }

		Ok(true)
	}
}

/// A complete PoW Algorithm that uses Sha3 hashing.
/// Needs a reference to the client so it can grab the difficulty from the runtime.
pub struct Sha3Algorithm<C> {
	client: Arc<C>,
}

impl<C> Sha3Algorithm<C> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

// Manually implement clone. Deriving doesn't work because
// it'll derive impl<C: Clone> Clone for Sha3Algorithm<C>. But C in practice isn't Clone.
impl<C> Clone for Sha3Algorithm<C> {
	fn clone(&self) -> Self {
		Self::new(self.client.clone())
	}
}

// Here we implement the general PowAlgorithm trait for our concrete Sha3Algorithm
impl<B: BlockT<Hash = H256>, C> PowAlgorithm<B> for Sha3Algorithm<C>
where
	C: ProvideRuntimeApi<B>,
	C::Api: DifficultyApi<B, U256>,
{
	type Difficulty = U256;

	fn difficulty(&self, parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
		self.client
			.runtime_api()
			.difficulty(parent)
			.map_err(|err| {
				sc_consensus_pow::Error::Environment(format!(
					"Fetching difficulty from runtime failed: {:?}",
					err
				))
			})
	}

	fn verify(
		&self,
		_parent: &BlockId<B>,
		_pre_hash: &H256,
		_pre_digest: Option<&[u8]>,
		seal: &RawSeal,
		difficulty: Self::Difficulty,
	) -> Result<bool, Error<B>> {
		// Try to construct a seal object by decoding the raw seal given
		let seal = match Seal::decode(&mut &seal[..]) {
			Ok(seal) => seal,
			Err(_) => return Ok(false),
		};

		// See whether the hash meets the difficulty requirement. If not, fail fast.
		if !hash_meets_difficulty(&seal.work, difficulty) {
			return Ok(false);
		}

		// Make sure the provided work actually comes from the correct pre_hash
		// let compute = Compute {
		// 	difficulty,
		// 	pre_hash: *pre_hash,
		// 	nonce: seal.nonce,
		// };

		// if compute.compute() != seal {
		// 	return Ok(false);
		// }

		Ok(true)
	}
}
