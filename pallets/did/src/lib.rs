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

//! DID: Handles decentralized identifiers on chain,
//! adding and removing DIDs.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub mod default_weights;
pub mod origin;
pub mod types;

mod utils;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use default_weights::WeightInfo;

pub use origin::*;
pub use types::*;

pub use pallet::*;

use codec::Encode;

use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	ensure,
	storage::types::StorageMap,
	Parameter,
};
use frame_system::{ensure_signed};
use sp_std::{boxed::Box, fmt::Debug, prelude::Clone};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Type for origin that supports a DID sender.
	pub type Origin<T> = DidRawOrigin<<T as Config>::DidIdentifier>;

	#[pallet::config]
	pub trait Config: frame_system::Config + Debug {
		type Call: Parameter
			+ Dispatchable<Origin = <Self as Config>::Origin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ DeriveDidCallAuthorizationVerificationKeyRelationship;
		type DidIdentifier: Parameter;
		type Origin: From<DidRawOrigin<DidIdentifier<Self>>>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// DIDs stored on chain.
	///
	/// It maps from a DID identifier to the DID details.
	#[pallet::storage]
	#[pallet::getter(fn get_did)]
	pub type Did<T> = StorageMap<_, Blake2_128Concat, DidIdentifier<T>, DidDetails<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new DID has been created.
		/// \[transaction signer, DID identifier\]
		DidCreated(AccountIdentifier<T>, DidIdentifier<T>),
		/// A DID has been updated.
		/// \[transaction signer, DID identifier\]
		DidUpdated(AccountIdentifier<T>, DidIdentifier<T>),
		/// A DID has been deleted.
		/// \[transaction signer, DID identifier\]
		DidDeleted(AccountIdentifier<T>, DidIdentifier<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The DID operation signature is not in the format the verification
		/// key expects.
		InvalidSignatureFormat,
		/// The DID operation signature is invalid for the payload and the
		/// verification key provided.
		InvalidSignature,
		/// The DID with the given identifier is already present on chain.
		DidAlreadyPresent,
		/// No DID with the given identifier is present on chain.
		DidNotPresent,
		/// One or more verification keys referenced are not stored in the set
		/// of verification keys.
		VerificationKeyNotPresent,
		/// The DID operation nonce is not equal to the current DID nonce + 1.
		InvalidNonce,
		/// The URL specified is not ASCII-encoded.
		InvalidUrlEncoding,
		/// The URL specified is not properly formatted.
		InvalidUrlScheme,
		/// The maximum supported value for the DID tx counter has been reached.
		/// No more operations with the DID are allowed.
		MaxTxCounterValue,
		/// The user tries to delete a verification key that is currently being
		/// used as an authentication, delegation, or attestation key, and this
		/// is not allowed.
		CurrentlyActiveKey,
		/// The called extrinsic does not support DID authorization.
		UnsupportedDidAuthorizationCall,
		/// An error that is not supposed to take place, yet it happened.
		InternalError,
	}

	impl<T> From<DidError> for Error<T> {
		fn from(error: DidError) -> Self {
			match error {
				DidError::StorageError(storage_error) => Self::from(storage_error),
				DidError::SignatureError(operation_error) => Self::from(operation_error),
				DidError::UrlError(url_error) => Self::from(url_error),
				DidError::InternalError => Self::InternalError,
			}
		}
	}

	impl<T> From<StorageError> for Error<T> {
		fn from(error: StorageError) -> Self {
			match error {
				StorageError::DidNotPresent => Self::DidNotPresent,
				StorageError::DidAlreadyPresent => Self::DidAlreadyPresent,
				StorageError::DidKeyNotPresent(_) | StorageError::VerificationKeyNotPresent => {
					Self::VerificationKeyNotPresent
				}
				StorageError::MaxTxCounterValue => Self::MaxTxCounterValue,
				StorageError::CurrentlyActiveKey => Self::CurrentlyActiveKey,
			}
		}
	}

	impl<T> From<SignatureError> for Error<T> {
		fn from(error: SignatureError) -> Self {
			match error {
				SignatureError::InvalidSignature => Self::InvalidSignature,
				SignatureError::InvalidSignatureFormat => Self::InvalidSignatureFormat,
				SignatureError::InvalidNonce => Self::InvalidNonce,
			}
		}
	}

	impl<T> From<UrlError> for Error<T> {
		fn from(error: UrlError) -> Self {
			match error {
				UrlError::InvalidUrlEncoding => Self::InvalidUrlEncoding,
				UrlError::InvalidUrlScheme => Self::InvalidUrlScheme,
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stores a new DID on chain, after verifying the signature associated
		/// with the creation operation.
		///
		/// * origin: the Substrate account submitting the transaction (which
		///   can be different from the DID subject)
		/// * operation: the [DidCreationOperation] which contains the details
		///   of the new DID
		/// * signature: the [signature](DidSignature) over the operation that
		///   must be signed with the authentication key provided in the
		///   operation
		#[pallet::weight(T::WeightInfo::submit_did_create_operation())]
		pub fn submit_did_create_operation(
			origin: OriginFor<T>,
			operation: DidCreationOperation<T>,
			signature: DidSignature,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// There has to be no other DID with the same identifier already saved on chain,
			// otherwise generate a DidAlreadyPresent error.
			ensure!(
				!<Did<T>>::contains_key(operation.get_did()),
				<Error<T>>::DidAlreadyPresent
			);

			let did_entry = DidDetails::from(operation.clone());

			Self::verify_payload_signature_with_did_key_type(
				&operation.encode(),
				&signature,
				&did_entry,
				operation.get_verification_key_relationship(),
			)
			.map_err(<Error<T>>::from)?;

			let did_identifier = operation.get_did();
			log::debug!("Creating DID {:?}", did_identifier);
			<Did<T>>::insert(did_identifier, did_entry);

			Self::deposit_event(Event::DidCreated(sender, did_identifier.clone()));

			Ok(None.into())
		}

		/// Updates the information associated with a DID on chain, after
		/// verifying the signature associated with the operation.
		///
		/// * origin: the Substrate account submitting the transaction (which
		///   can be different from the DID subject)
		/// * operation: the [DidUpdateOperation] which contains the new details
		///   of the given DID
		/// * signature: the [signature](DidSignature) over the operation that
		///   must be signed with the authentication key associated with the new
		///   DID. Even in case the authentication key is being updated, the
		///   operation must still be signed with the old one being replaced.
		#[pallet::weight(T::WeightInfo::submit_did_update_operation())]
		pub fn submit_did_update_operation(
			origin: OriginFor<T>,
			operation: DidUpdateOperation<T>,
			signature: DidSignature,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Saved here as it is consumed later when generating the new DidDetails object.
			let did_identifier = operation.get_did().clone();

			let did_details = <Did<T>>::get(&did_identifier).ok_or(<Error<T>>::DidNotPresent)?;

			// Verify the signature and the nonce of the update operation.
			Self::verify_operation_validity_for_did(&operation, &signature, &did_details).map_err(<Error<T>>::from)?;

			// Generate a new DidDetails object by applying the changes in the update
			// operation to the old object (and consuming both).
			let new_did_details = DidDetails::try_from((did_details, operation)).map_err(<Error<T>>::from)?;

			log::debug!("Updating DID {:?}", did_identifier);
			<Did<T>>::insert(&did_identifier, new_did_details);

			Self::deposit_event(Event::DidUpdated(sender, did_identifier));

			Ok(None.into())
		}

		/// Deletes all the information associated with a DID on chain, after
		/// verifying the signature associated with the operation.
		///
		/// * origin: the Substrate account submitting the transaction (which
		///   can be different from the DID subject)
		/// * operation: the [DidDeletionOperation] which includes the DID to
		///   deactivate
		/// * signature: the [signature](DidSignature) over the operation that
		///   must be signed with the authentication key associated with the new
		///   DID.
		#[pallet::weight(T::WeightInfo::submit_did_delete_operation())]
		pub fn submit_did_delete_operation(
			origin: OriginFor<T>,
			operation: DidDeletionOperation<T>,
			signature: DidSignature,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let did_identifier = operation.get_did();

			let did_details = <Did<T>>::get(&did_identifier).ok_or(<Error<T>>::DidNotPresent)?;

			// Verify the signature and the nonce of the delete operation.
			Self::verify_operation_validity_for_did(&operation, &signature, &did_details).map_err(<Error<T>>::from)?;

			log::debug!("Deleting DID {:?}", did_identifier);
			<Did<T>>::remove(&did_identifier);

			Self::deposit_event(Event::DidDeleted(sender, did_identifier.clone()));

			Ok(None.into())
		}

		// TODO: Compute right weights
		/// Submit the execution of another runtime extrinsic conforming to the
		/// [Call] trait that supports DID-based authorization.
		///
		/// * origin: the account submitting this `submit_did_call` extrinsic,
		///   which pays for the transaction fees.
		/// * did_call: the DID-authorized runtime extrinsic operation to call.
		/// * signature: the DID signature over the encoded `did_call` that must
		///   be signed with the expected DID verification key.
		#[pallet::weight(10)]
		pub fn submit_did_call(
			origin: OriginFor<T>,
			did_call: Box<DidAuthorizedCallOperation<T>>,
			signature: DidSignature,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let did_identifier = did_call.did.clone();

			// Compute the right DID verification key to use to verify the operation
			// signature
			let verification_key_relationship = did_call
				.call
				.derive_verification_key_relationship()
				.ok_or(<Error<T>>::UnsupportedDidAuthorizationCall)?;

			// Wrap the operation in the expected structure, specifying the key retrieved
			let wrapped_operation = DidAuthorizedCallOperationWithVerificationRelationship {
				operation: *did_call.clone(),
				verification_key_relationship,
			};

			// Verify the operation signature and increase the nonce if successful.
			Self::verify_operation_validity_and_increase_did_nonce(&wrapped_operation, &signature)
				.map_err(<Error<T>>::from)?;
			log::debug!("Dispatch call from DID {:?}", did_identifier);

			// Dispatch the referenced [Call] instance and return its result
			did_call.call.dispatch(DidRawOrigin { id: did_identifier }.into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Verify the validity (i.e., nonce and signature) of a generic
	/// [DidOperation] and, if valid, update the DID state with the latest
	/// nonce.
	///
	/// * operation: the reference to the [DidOperation] which validity is to be
	///   verified
	/// * signature: a reference to the [signature](DidSignature)
	/// * did: the DID identifier to verify the operation signature for
	pub fn verify_operation_validity_and_increase_did_nonce<O: DidOperation<T>>(
		operation: &O,
		signature: &DidSignature,
	) -> Result<(), DidError> {
		let mut did_details =
			<Did<T>>::get(&operation.get_did()).ok_or(DidError::StorageError(StorageError::DidNotPresent))?;

		Self::verify_operation_validity_for_did(operation, &signature, &did_details)?;

		// Update tx counter in DID details and save to DID pallet
		did_details.increase_tx_counter().map_err(DidError::StorageError)?;
		<Did<T>>::insert(&operation.get_did(), did_details);

		Ok(())
	}

	// Internally verifies the validity of a DID operation nonce and signature.
	fn verify_operation_validity_for_did<O: DidOperation<T>>(
		operation: &O,
		signature: &DidSignature,
		did_details: &DidDetails<T>,
	) -> Result<(), DidError> {
		Self::verify_operation_counter_for_did(operation, did_details)?;
		Self::verify_payload_signature_with_did_key_type(
			&operation.encode(),
			signature,
			did_details,
			operation.get_verification_key_relationship(),
		)
	}

	// Verify the validity of a DID operation nonce.
	// To be valid, the nonce must be equal to the one currently stored + 1.
	// This is to avoid quickly "consuming" all the possible values for the counter,
	// as that would result in the DID being unusable, since we do not have yet any
	// mechanism in place to wrap the counter value around when the limit is
	// reached.
	fn verify_operation_counter_for_did<O: DidOperation<T>>(
		operation: &O,
		did_details: &DidDetails<T>,
	) -> Result<(), DidError> {
		// Verify that the DID has not reached the maximum tx counter value
		ensure!(
			did_details.get_tx_counter_value() < u64::MAX,
			DidError::StorageError(StorageError::MaxTxCounterValue)
		);

		// Verify that the operation counter is equal to the stored one + 1
		let expected_nonce_value = did_details
			.get_tx_counter_value()
			.checked_add(1)
			.ok_or(DidError::InternalError)?;
		ensure!(
			operation.get_tx_counter() == expected_nonce_value,
			DidError::SignatureError(SignatureError::InvalidNonce)
		);

		Ok(())
	}

	// Verify a generic payload signature using a given DID verification key type.
	pub fn verify_payload_signature_with_did_key_type(
		payload: &Payload,
		signature: &DidSignature,
		did_details: &DidDetails<T>,
		key_type: DidVerificationKeyRelationship,
	) -> Result<(), DidError> {
		// Retrieve the needed verification key from the DID details, or generate an
		// error if there is no key of the type required
		let verification_key = did_details
			.get_verification_key_for_key_type(key_type)
			.ok_or(DidError::StorageError(StorageError::DidKeyNotPresent(key_type)))?;

		// Verify that the signature matches the expected format, otherwise generate
		// an error
		let is_signature_valid = verification_key
			.verify_signature(&payload, &signature)
			.map_err(|_| DidError::SignatureError(SignatureError::InvalidSignatureFormat))?;

		ensure!(
			is_signature_valid,
			DidError::SignatureError(SignatureError::InvalidSignature)
		);

		Ok(())
	}
}
