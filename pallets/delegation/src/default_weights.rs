// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! //! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-11, STEPS: [10, ], REPEAT: 4, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE:
//! 128

// Executed Command:
// /home/willi/mashnet-node/target/release/mashnet-node
// benchmark
// --execution=wasm
// --wasm-execution=Interpreted
// --heap-pages=4096
// -e
// *
// -p
// delegation
// -s
// 10
// -r
// 4
// --output
// ../../pallets/delegation/src/default_weights.rs
// --template
// ../../.maintain/weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for delegation.
pub trait WeightInfo {
	fn submit_delegation_root_creation_operation() -> Weight;
	fn submit_delegation_creation_operation() -> Weight;
	fn submit_delegation_root_revocation_operation(r: u32) -> Weight;
	fn submit_delegation_revocation_operation(r: u32) -> Weight;
	fn revoke_delegation_leaf(r: u32) -> Weight;
}

/// Weights for delegation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn submit_delegation_root_creation_operation() -> Weight {
		(100_067_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn submit_delegation_creation_operation() -> Weight {
		(272_160_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn submit_delegation_root_revocation_operation(r: u32) -> Weight {
		(99_623_000_u64)
			// Standard Error: 1_205_000
			.saturating_add((140_205_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn submit_delegation_revocation_operation(r: u32) -> Weight {
		(32_579_000_u64)
			// Standard Error: 799_000
			.saturating_add((139_299_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().reads_writes(1, 1))
	}
	fn revoke_delegation_leaf(r: u32) -> Weight {
		(151_834_000_u64)
			// Standard Error: 981_000
			.saturating_add((43_149_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn submit_delegation_root_creation_operation() -> Weight {
		(100_067_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn submit_delegation_creation_operation() -> Weight {
		(272_160_000_u64)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn submit_delegation_root_revocation_operation(r: u32) -> Weight {
		(99_623_000_u64)
			// Standard Error: 1_205_000
			.saturating_add((140_205_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn submit_delegation_revocation_operation(r: u32) -> Weight {
		(32_579_000_u64)
			// Standard Error: 799_000
			.saturating_add((139_299_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().reads_writes(1, 1))
	}
	fn revoke_delegation_leaf(r: u32) -> Weight {
		(151_834_000_u64)
			// Standard Error: 981_000
			.saturating_add((43_149_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
