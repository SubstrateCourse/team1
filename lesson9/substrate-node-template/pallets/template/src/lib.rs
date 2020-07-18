#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{
    self as system, ensure_signed,
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,
    },
};
use core::{convert::TryInto};

use sp_runtime::{
    offchain as rt_offchain,
    offchain::storage::StorageValueRef,
    transaction_validity::{
        InvalidTransaction, TransactionPriority, TransactionSource, TransactionValidity,
        ValidTransaction,
    },
};

use sp_std::prelude::*; //Vec need
use sp_std::str;

// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};
use serde_json::json;
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ruan");

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

pub const HTTP_REMOTE_01_CRYPTOCOMPARE_REQUEST_BYTES: &[u8] = b"https://min-api.cryptocompare.com/data/price?fsym=eth&tsyms=USD";

pub const HTTP_REMOTE_02_COINMARKETCAP_REQUEST_BYTES: &[u8] = b"https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=ETH";
pub const HTTP_HEADER_02_COINMARKETCAP_API_KEY_BYTES: &[u8] = b"6d479e2a-cd6d-489a-9697-b69f31ac5830";

enum ChannelType {
    ChannelTypeCC,    // https://min-api.cryptocompare.com
        ChannelTypeCMC,   // https://pro-api.coinmarketcap.com
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

pub fn de_float_to_integer<'de, D>(de: D) -> Result<u32, D::Error>
where D: Deserializer<'de> {
    let f: f32 = Deserialize::deserialize(de)?;
    Ok((f * 100.0) as u32)
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct ETHPriceInfo01CC {
    #[serde(rename(deserialize = "USD"), deserialize_with = "de_float_to_integer")]
    usd: u32,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct Status {
    #[serde(deserialize_with = "de_string_to_bytes")]
    timestamp: Vec<u8>,
    error_code: u32,
    //error_message: String,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct Data {
    #[serde(rename = "ETH")]
    eth: ETH,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct ETH {
    quote: Quote,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct Quote {
    #[serde(rename = "USD")]
    usd: USD,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct USD {
    #[serde(deserialize_with = "de_float_to_integer")]
    price: u32,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct ETHPriceInfo02CMC {
    status: Status,
    data: Data,
}


/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>>{
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    type Call: From<Call<Self>>;

}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        Prices get(fn prices): Vec<u32>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        NewPrice(u32, AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        // 01 from https://min-api.cryptocompare.com
        HttpFetching01Error,
        // 02 from https://pro-api.coinmarketcap.com
        HttpFetching02Error,
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
        pub fn submit_price(origin, price: u32) -> dispatch::DispatchResult {
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;
            Self::add_price(who, price);
            Ok(())
        }

        fn offchain_worker(block_number: T::BlockNumber) {
            debug::info!("Entering off-chain workers");

            match block_number.try_into().ok().unwrap() % 10 {
                0 => {
                    let res = Self::fetch_price_and_send_signed();
                    if let Err(e) = res {
                        debug::error!("Error: {}", e);
                    }
                },
                _ => debug::info!("ignore..."),
            
            }
        }
    }
}

impl<T: Trait> Module<T> {

    fn add_price(who: T::AccountId, price: u32) {
        debug::info!("Adding eth price: {}", price);
        Prices::mutate(|prices| {
            prices.push(price);
        });

        Self::deposit_event(RawEvent::NewPrice(price, who));
    }

    /// A helper function to fetch the price and send signed transaction.
    fn fetch_price_and_send_signed() -> Result<(), &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC."
            )?
        }
        // Make an external HTTP request to fetch the current price.
        // Note this call will block until response is received.
        let price1 = Self::fetch_eth_price(ChannelType::ChannelTypeCC).map_err(|_| "Failed to fetch eth price from CC")?;
        let price2 = Self::fetch_eth_price(ChannelType::ChannelTypeCMC).map_err(|_| "Failed to fetch eth price from CMC")?;
        let price = (price1 + price2)/2;
        debug::info!("average eth price in usd cents: {}", price);

        // Using `send_signed_transaction` associated type we create and submit a transaction
        // representing the call, we've just created.
        // Submit signed will return a vector of results for all accounts that were found in the
        // local keystore with expected `KEY_TYPE`.
        let results = signer.send_signed_transaction(
            |_account| {
                // Received price is wrapped into a call to `submit_price` public function of this pallet.
                // This means that the transaction, when executed, will simply call that function passing
                // `price` as an argument.
                Call::submit_price(price)
        }
        );

        for (acc, res) in &results {
            match res {
                Ok(()) => debug::info!("[{:?}] Submitted price of {} cents", acc.id, price),
                Err(e) => debug::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
            }
        }

        Ok(())
    }

    fn fetch_eth_price(channel_type : ChannelType) -> Result<u32, Error<T>> {
        match channel_type {
            ChannelType::ChannelTypeCC => {
                let resp_bytes = Self::fetch_from_remote_01_cryptocompare().map_err(|e| {
                    debug::error!("fetch_from_remote error: {:?}", e);
                    <Error<T>>::HttpFetching01Error
                })?;

                //debug::info!("fetch_from_remote_01_cryptocompare success, and parse it");

                match Self::parse_eth_price_01_cryptocompare_resp(resp_bytes) {

                    Ok(price) => {
                        debug::info!("<cryptocompare> eth usd in cents: {}", price.usd);
                        Ok(price.usd)
                    }
                    Err(err) => {
                        debug::error!("parse error: {:?}", err);
                        Err(err)
                    }

                }
            },
            ChannelType::ChannelTypeCMC => {
                let resp_bytes = Self::fetch_from_remote_02_coinmarketcap().map_err(|e| {
                    debug::error!("fetch_from_remote error: {:?}", e);
                    <Error<T>>::HttpFetching02Error
                })?;

                //debug::info!("fetch_from_remote_01_cryptocompare success, and parse it");

                match Self::parse_eth_price_02_coinmarketcap_resp(resp_bytes) {

                    Ok(price) => {
                        let price_usd_cents = price.data.eth.quote.usd.price;
                        debug::info!("<coinmarketcap> eth usd in cents: {}", price_usd_cents);
                        Ok(price_usd_cents)
                    }
                    Err(err) => {
                        debug::error!("parse error: {:?}", err);
                        Err(err)
                    }
                }
            }
        }
    }

    fn fetch_from_remote_01_cryptocompare() -> Result<Vec<u8>, Error<T>> {
        let remote_url_bytes = HTTP_REMOTE_01_CRYPTOCOMPARE_REQUEST_BYTES.to_vec();
        //let user_agent = HTTP_HEADER_USER_AGENT.to_vec();
        let remote_url =
            str::from_utf8(&remote_url_bytes).map_err(|_| <Error<T>>::HttpFetching01Error)?;

        debug::info!("sending request to <01>cryptocompare: {}", remote_url);

        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = rt_offchain::http::Request::get(remote_url);

        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

        // For github API request, we also need to specify `user-agent` in http request header.
        //   See: https://developer.github.com/v3/#user-agent-required
        let pending = request
            //.add_header(
            //    "User-Agent",
            //    str::from_utf8(&user_agent).map_err(|_| <Error<T>>::HttpFetching01Error)?,
            //)
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetching01Error)?;

        // By default, the http request is async from the runtime perspective. So we are asking the
        //   runtime to wait here.
        // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
        //   ref: https://substrate.dev/rustdocs/v2.0.0-rc2/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
        let response = pending
            .try_wait(timeout)
            .map_err(|_| <Error<T>>::HttpFetching01Error)?
            .map_err(|_| <Error<T>>::HttpFetching01Error)?;

        if response.code != 200 {
            debug::error!("Unexpected http request from <01>cryptocompare status code: {}", response.code);
            return Err(<Error<T>>::HttpFetching01Error);
        }

        // Next we fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }

    fn parse_eth_price_01_cryptocompare_resp(resp_bytes : Vec<u8>) -> Result<ETHPriceInfo01CC, Error<T>> {
        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetching01Error)?;
        let eth_info: ETHPriceInfo01CC =
            serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetching01Error)?;
        Ok(eth_info)
    }

    fn fetch_from_remote_02_coinmarketcap() -> Result<Vec<u8>, Error<T>> {
        let remote_url_bytes = HTTP_REMOTE_02_COINMARKETCAP_REQUEST_BYTES.to_vec();
        let cmc_pro_api_key = HTTP_HEADER_02_COINMARKETCAP_API_KEY_BYTES.to_vec();
        let remote_url =
            str::from_utf8(&remote_url_bytes).map_err(|_| <Error<T>>::HttpFetching02Error)?;

        debug::info!("sending request to <02>coinmarketcap: {}", remote_url);

        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = rt_offchain::http::Request::get(remote_url);

        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

        // For github API request, we also need to specify `user-agent` in http request header.
        //   See: https://developer.github.com/v3/#user-agent-required
        let pending = request
            .add_header(
                "X-CMC_PRO_API_KEY",
                str::from_utf8(&cmc_pro_api_key).map_err(|_| <Error<T>>::HttpFetching02Error)?,
            )
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetching02Error)?;

        // By default, the http request is async from the runtime perspective. So we are asking the
        //   runtime to wait here.
        // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
        //   ref: https://substrate.dev/rustdocs/v2.0.0-rc2/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
        let response = pending
            .try_wait(timeout)
            .map_err(|_| <Error<T>>::HttpFetching02Error)?
            .map_err(|_| <Error<T>>::HttpFetching02Error)?;

        if response.code != 200 {
            debug::error!("Unexpected http request from <02>coinmarketcap status code: {}", response.code);
            return Err(<Error<T>>::HttpFetching02Error);
        }

        // Next we fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }

    fn parse_eth_price_02_coinmarketcap_resp(resp_bytes : Vec<u8>) -> Result<ETHPriceInfo02CMC, Error<T>> {
        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetching02Error)?;
        let eth_info: ETHPriceInfo02CMC =
            serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetching02Error)?;
        Ok(eth_info)
    }
}
