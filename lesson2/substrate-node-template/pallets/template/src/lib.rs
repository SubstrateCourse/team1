#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap};
use frame_system::{self as system, ensure_signed};
use sp_core::H256;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
        Proofs: map hasher(blake2_128_concat) H256 => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, H256),
		ClaimRevoked(AccountId, H256),
		ClaimTransfer(AccountId, AccountId, H256),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
		ProofOutOfbound,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

        #[weight = 0]
		fn create_claim(origin, proof: H256){
			
			let sender = ensure_signed(origin)?;
			
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <system::Module<T>>::block_number();

			Proofs::<T>::insert(&proof, (&sender, current_block));

			Self::deposit_event(RawEvent::ClaimCreated(sender, proof));

		}
        #[weight = 0 ]
		fn revoke_claim(origin, proof: H256){
			
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);

			ensure!(owner == sender, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(RawEvent::ClaimRevoked(sender,proof));

		}

		#[weight = 0]
		fn transfer_claim(origin, proof:H256, receiver: T::AccountId){
			
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);

			ensure!(owner == sender, Error::<T>::NotProofOwner);
			
			Proofs::<T>::remove(&proof);

			let current_block = <system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof, (&receiver, current_block));

			Self::deposit_event(RawEvent::ClaimTransfer(sender, receiver, proof));
		}
	}
}
