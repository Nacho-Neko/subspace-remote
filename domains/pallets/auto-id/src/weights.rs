
//! Autogenerated weights for pallet_auto_id
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `mac-mini.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/subspace-node
// domain
// benchmark
// pallet
// --runtime=./target/release/wbuild/auto-id-domain-runtime/auto_id_domain_runtime.compact.compressed.wasm
// --steps=50
// --repeat=20
// --pallet=pallet_auto_id
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./domains/pallets/auto-id/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::ParityDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_auto_id.
pub trait WeightInfo {
	fn register_issuer_auto_id() -> Weight;
	fn register_leaf_auto_id() -> Weight;
	fn revoke_issuer_auto_id() -> Weight;
	fn revoke_leaf_auto_id() -> Weight;
	fn deactivate_auto_id() -> Weight;
	fn renew_issuer_auto_id() -> Weight;
	fn renew_leaf_auto_id() -> Weight;
}

/// Weights for pallet_auto_id using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36`
		//  Estimated: `3501`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 3501)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:2 w:2)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:0)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `7224`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(41_000_000, 7224)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn revoke_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1248`
		//  Estimated: `4713`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 4713)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:2 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn revoke_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2559`
		//  Estimated: `8499`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 8499)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deactivate_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1248`
		//  Estimated: `4713`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 4713)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn renew_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `4749`
		// Minimum execution time: 39_000_000 picoseconds.
		Weight::from_parts(40_000_000, 4749)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:2 w:2)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn renew_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2595`
		//  Estimated: `8535`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(47_000_000, 8535)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36`
		//  Estimated: `3501`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 3501)
			.saturating_add(ParityDbWeight::get().reads(2_u64))
			.saturating_add(ParityDbWeight::get().writes(1_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:2 w:2)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:0)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `7224`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(41_000_000, 7224)
			.saturating_add(ParityDbWeight::get().reads(4_u64))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn revoke_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1248`
		//  Estimated: `4713`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 4713)
			.saturating_add(ParityDbWeight::get().reads(2_u64))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:2 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn revoke_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2559`
		//  Estimated: `8499`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 8499)
			.saturating_add(ParityDbWeight::get().reads(3_u64))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deactivate_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1248`
		//  Estimated: `4713`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 4713)
			.saturating_add(ParityDbWeight::get().reads(1_u64))
			.saturating_add(ParityDbWeight::get().writes(1_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:1 w:1)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn renew_issuer_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `4749`
		// Minimum execution time: 39_000_000 picoseconds.
		Weight::from_parts(40_000_000, 4749)
			.saturating_add(ParityDbWeight::get().reads(3_u64))
			.saturating_add(ParityDbWeight::get().writes(2_u64))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AutoId::AutoIds` (r:2 w:2)
	/// Proof: `AutoId::AutoIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AutoId::CertificateRevocationList` (r:1 w:1)
	/// Proof: `AutoId::CertificateRevocationList` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn renew_leaf_auto_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2595`
		//  Estimated: `8535`
		// Minimum execution time: 47_000_000 picoseconds.
		Weight::from_parts(47_000_000, 8535)
			.saturating_add(ParityDbWeight::get().reads(4_u64))
			.saturating_add(ParityDbWeight::get().writes(3_u64))
	}
}
