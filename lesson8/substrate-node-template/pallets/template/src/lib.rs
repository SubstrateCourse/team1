#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, dispatch::DispatchResult, traits::Get};
use frame_system::{
    self as system, ensure_signed,
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer, 
    },
};

use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;
use core::{convert::TryInto};

use sp_runtime::{
    transaction_validity::{
        TransactionPriority
    }
};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;

    pub type AuthorityId = Public;

    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };

    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;
    // implemented for ocw-runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
        {
            type RuntimeAppPublic = Public;
            type GenericSignature = sp_core::sr25519::Signature;
            type GenericPublic = sp_core::sr25519::Public;
        }
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;

    /// The type to sign and send transactions.
    type UnsignedPriority: Get<TransactionPriority>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Numbers get(fn numbers): Vec<u32>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        NumberSaved(Option<AccountId>, u32),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        SignedSubmitNumberError,
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

        #[weight = 10_000]
        pub fn save_number(origin, number: u32) -> dispatch::DispatchResult {
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;

            /*******
             * 学员们在这里追加逻辑
             *******/
            Self::do_save_number(Some(who), number)
        }

        fn offchain_worker(block_number: T::BlockNumber) {
            debug::info!("Entering off-chain workers");

            /*******
             * 学员们在这里追加逻辑
             *******/
            let result = Self::signed_submit_number(block_number);
            if let Err(e) = result {
                debug::error!("Error: {:?}", e);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    /// Add a new number to the list.
    fn do_save_number(who: Option<T::AccountId>, number: u32) -> DispatchResult {
        Numbers::mutate(|numbers| {
            numbers.push(number);
            debug::info!("Success push numbers {}", number);
        });

        Self::deposit_event(RawEvent::NumberSaved(who, number));
        Ok(())
    }

    fn signed_submit_number(block_number: T::BlockNumber) -> Result<(), Error<T>> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            debug::error!("No local account available");
            return Err(<Error<T>>::SignedSubmitNumberError);
        }

        let mut b_number: u32 = block_number.try_into().ok().unwrap() as u32;
        let mut total = 0;
        b_number = b_number+1;
        while b_number != 0 {
            total = total + b_number * b_number;
            b_number = b_number - 1;
        }
        let results = signer.send_signed_transaction(|_acct| {
            Call::save_number(total)
        });

        for (acc, res) in &results {
            match res {
                Ok(()) => {
                    debug::native::info!(
                        "off-chain send_signed: acc: {:?}| number: {}",
                        acc.id,
                        total 
                    );
                }
                Err(e) => {
                    debug::error!("[{:?}] Failed in signed_submit_number: {:?}", acc.id, e);
                    return Err(<Error<T>>::SignedSubmitNumberError);
                }
            };
        }
        Ok(())
    }
}
