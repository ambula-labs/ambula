#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_core::crypto::KeyTypeId;
use sp_core::ecdsa::Public;

/// Key type for POI module.
pub const POI_KEY_TYPE: sp_core::crypto::KeyTypeId = KeyTypeId(*b"demo");

mod app {
	use sp_application_crypto::{app_crypto, ecdsa};
	use super::*;
	app_crypto!(ecdsa, POI_KEY_TYPE);
}

sp_application_crypto::with_pair! {
	/// The grandpa crypto scheme defined via the keypair type.
	pub type AuthorityPair = app::Pair;
}

/// Identity of a Grandpa authority.
pub type AuthorityId = Public;


/// An authority discovery authority signature.
pub type AuthoritySignature = app::Signature;
