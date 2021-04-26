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

#![allow(clippy::from_over_into)]

use crate as did;
use crate::*;

use frame_support::{parameter_types, weights::constants::RocksDbWeight};
use kilt_primitives::{AccountId, Signature};
use sp_core::*;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};

pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Did: did::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 38;
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = ();

	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl did::Config for Test {
	type Event = ();
	type Call = Call;
	type WeightInfo = ();
	type DidIdentifier = AccountId;
}

pub type TestDidIdentifier = <Test as did::Config>::DidIdentifier;

#[cfg(test)]
pub(crate) const DEFAULT_ACCOUNT: AccountId = AccountId::new([0u8; 32]);

pub const ALICE_DID: TestDidIdentifier = AccountId::new([1u8; 32]);
pub const BOB_DID: TestDidIdentifier = AccountId::new([2u8; 32]);
pub const CHARLIE_DID: TestDidIdentifier = AccountId::new([3u8; 32]);
const DEFAULT_AUTH_SEED: [u8; 32] = [4u8; 32];
const ALTERNATIVE_AUTH_SEED: [u8; 32] = [40u8; 32];
const DEFAULT_ENC_SEED: [u8; 32] = [5u8; 32];
const ALTERNATIVE_ENC_SEED: [u8; 32] = [50u8; 32];
const DEFAULT_ATT_SEED: [u8; 32] = [6u8; 32];
const ALTERNATIVE_ATT_SEED: [u8; 32] = [60u8; 32];
const DEFAULT_DEL_SEED: [u8; 32] = [7u8; 32];
const ALTERNATIVE_DEL_SEED: [u8; 32] = [70u8; 32];

pub fn get_ed25519_authentication_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_AUTH_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_AUTH_SEED)
	}
}

pub fn get_sr25519_authentication_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_AUTH_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_AUTH_SEED)
	}
}

pub fn get_x25519_encryption_key(default: bool) -> PublicEncryptionKey {
	if default {
		PublicEncryptionKey::X25519(DEFAULT_ENC_SEED)
	} else {
		PublicEncryptionKey::X25519(ALTERNATIVE_ENC_SEED)
	}
}

pub fn get_ed25519_attestation_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_ATT_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_ATT_SEED)
	}
}

pub fn get_sr25519_attestation_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_ATT_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_ATT_SEED)
	}
}

pub fn get_ed25519_delegation_key(default: bool) -> ed25519::Pair {
	if default {
		ed25519::Pair::from_seed(&DEFAULT_DEL_SEED)
	} else {
		ed25519::Pair::from_seed(&ALTERNATIVE_DEL_SEED)
	}
}

pub fn get_sr25519_delegation_key(default: bool) -> sr25519::Pair {
	if default {
		sr25519::Pair::from_seed(&DEFAULT_DEL_SEED)
	} else {
		sr25519::Pair::from_seed(&ALTERNATIVE_DEL_SEED)
	}
}

// Given a DID identifier and an authentication key, it returns a
// DidCreationOperation that would successfully write the new DID on chain using
// a default key agreement key.
pub fn generate_base_did_creation_operation(
	did: TestDidIdentifier,
	new_auth_key: did::PublicVerificationKey,
) -> did::DidCreationOperation<Test> {
	DidCreationOperation {
		did,
		new_auth_key,
		new_key_agreement_key: get_x25519_encryption_key(true),
		new_attestation_key: None,
		new_delegation_key: None,
		new_endpoint_url: None,
	}
}

// Given a DID identifier, it returns a DidUpdateOperation
// that does not update any information for the DID.
pub fn generate_base_did_update_operation(did: TestDidIdentifier) -> did::DidUpdateOperation<Test> {
	DidUpdateOperation {
		did,
		new_auth_key: None,
		new_key_agreement_key: None,
		attestation_key_update: DidVerificationKeyUpdateAction::default(),
		delegation_key_update: DidVerificationKeyUpdateAction::default(),
		new_endpoint_url: None,
		verification_keys_to_remove: None,
		tx_counter: 1,
	}
}

// Given a DID identifier, it returns a DidDeletionOperation
// that would remove the DID from chain.
pub fn generate_base_did_delete_operation(did: TestDidIdentifier) -> did::DidDeletionOperation<Test> {
	DidDeletionOperation { did, tx_counter: 1 }
}

// Given an authentication key, it generates a DidDetails object with the given
// key and a default key agreement key.
pub fn generate_base_did_details(auth_key: did::PublicVerificationKey) -> did::DidDetails<Test> {
	did::DidDetails {
		auth_key,
		key_agreement_key: get_x25519_encryption_key(true),
		attestation_key: None,
		delegation_key: None,
		endpoint_url: None,
		last_tx_counter: 0,
		verification_keys: BTreeMap::new(),
	}
}

pub fn generate_attestation_key_id(
	key: did::PublicVerificationKey,
	tx_counter: u64,
) -> <Test as frame_system::Config>::Hash {
	let mut vec = key.encode();
	vec.extend_from_slice(did::DidVerificationKeyType::AssertionMethod.encode().as_slice());
	vec.extend_from_slice(tx_counter.encode().as_slice());

	let hasher = Blake2Hasher {};
	sp_core::Blake2Hasher::hash(&vec)
}

// A test DID operation which can be crated to require any DID verification key
// type.
#[derive(Clone, Decode, Debug, Encode, PartialEq)]
pub struct TestDidOperation {
	pub did: TestDidIdentifier,
	pub verification_key_type: DidVerificationKeyType,
	pub tx_counter: u64,
}

impl DidOperation<Test> for TestDidOperation {
	fn get_verification_key_type(&self) -> DidVerificationKeyType {
		self.verification_key_type.clone()
	}

	fn get_did(&self) -> &TestDidIdentifier {
		&self.did
	}

	fn get_tx_counter(&self) -> u64 {
		self.tx_counter
	}
}

#[derive(Clone)]
pub struct ExtBuilder {
	dids_stored: Vec<(TestDidIdentifier, did::DidDetails<Test>)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { dids_stored: vec![] }
	}
}

impl ExtBuilder {
	pub fn with_dids(mut self, dids: Vec<(TestDidIdentifier, did::DidDetails<Test>)>) -> Self {
		self.dids_stored = dids;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		let mut ext = sp_io::TestExternalities::new(storage);

		if !self.dids_stored.is_empty() {
			ext.execute_with(|| {
				self.dids_stored.iter().for_each(|did| {
					did::Did::<Test>::insert(did.0.clone(), did.1.clone());
				})
			});
		}

		ext
	}
}
