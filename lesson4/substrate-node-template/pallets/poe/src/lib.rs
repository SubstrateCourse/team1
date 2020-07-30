#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    // 附加题答案
    type MaxClaimLength: Get<u32>;

    type Currency: Currency<Self::AccountId>;

    //hw4
    type MaxNoteLength: Get<u32>;
}

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
        ProofsPrice: map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;

        //hw4
        ProofHash2Detail get(fn ph2d): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Moment, Option<Vec<u8>>);
        Account2ProofHashList get(fn a2phs): map hasher(identity) T::AccountId => Vec<Vec<u8>>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Price = BalanceOf<T>,
        ExpectPrice = BalanceOf<T>,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimPriceCreated(AccountId, Vec<u8>, Price),
        ClaimBought(AccountId, AccountId, Vec<u8>, Price, ExpectPrice),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        ProofTooLong,
        PriceAlreadySet,
        ClaimPriceNotSet,
        PriceTooLow,
        NoteTooLong,
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
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            // 附加题答案
            ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

            Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        //hw4
        #[weight = 0]
        pub fn create_claim_with_note(origin, claim: Vec<u8>, note: Option<Vec<u8>>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            match note.clone() {
                None => (),
                Some(text) => ensure!(T::MaxNoteLength::get() >= text.len() as u32, Error::<T>::NoteTooLong),
            }

            // 附加题答案
            ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

            ProofHash2Detail::<T>::insert(&claim,
                (sender.clone(),
                 system::Module::<T>::block_number(),
                 <timestamp::Module<T>>::now(),
                 note.clone()));

            if Account2ProofHashList::<T>::contains_key(&sender) {
                let mut vec = Account2ProofHashList::<T>::get(&sender);
                match vec.binary_search(&claim) {
                    // If the search succeeds, the caller is already a member, so just return
                    Ok(_) => (),
                    Err(index) => vec.insert(index, claim.clone()),
                };
                Account2ProofHashList::<T>::insert(&sender, vec);
            }
            else {
                let mut vec = Vec::<Vec<u8>>::new();
                vec.push(claim.clone());
                Account2ProofHashList::<T>::insert(&sender, vec);
            }

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        // 第二题答案
        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let dest = T::Lookup::lookup(dest)?;

            Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));

            Ok(())
        }

        #[weight = 0]
        pub fn attach_claim_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            ensure!(!ProofsPrice::<T>::contains_key(&claim), Error::<T>::PriceAlreadySet);

            ProofsPrice::<T>::insert(&claim, price);

            Self::deposit_event(RawEvent::ClaimPriceCreated(sender, claim, price));

            Ok(())
        }

        #[weight = 0]
        pub fn buy_claim(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            ensure!(ProofsPrice::<T>::contains_key(&claim), Error::<T>::ClaimPriceNotSet);

            let expect_price = ProofsPrice::<T>::get(&claim);

            let (owner, _) = Proofs::<T>::get(&claim);

            // 如果出价低于用户A的价格时，则不进行转移，返回错误
            ensure!(price >= expect_price, Error::<T>::PriceTooLow);

            // 当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A
            T::Currency::transfer(&sender, &owner, expect_price, AllowDeath)?;

            // 再将该存证进行转移
            Proofs::<T>::insert(&claim, (&sender, system::Module::<T>::block_number()));
            ProofsPrice::<T>::remove(&claim);

            //Self::deposit_event(RawEvent::ClaimBought(sender, owner, claim, price, expect_price));

            Ok(())
        }
    }
}
