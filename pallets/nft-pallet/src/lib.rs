#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use orml_nft;
use sp_std::vec::Vec;

pub type CID = Vec<u8>;

pub trait Trait: frame_system::Trait + orml_nft::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        Something get(fn something): Option<u32>;
        NClassId get(fn nft_class_id): map hasher(twox_64_concat) T::AccountId => T::ClassId;
        NTokenMetaData get(fn user_nft_token_metadata):  map hasher(twox_64_concat) T::AccountId =>  CID;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        <T as orml_nft::Trait>::ClassId,
        <T as orml_nft::Trait>::TokenId,
    {
        SomethingStored(u32, AccountId),
        TokenIssuedBy(AccountId),
        TokenMinted(AccountId),
        BurnedToken(AccountId, ClassId, TokenId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        NoneValue,
        StorageOverflow,
    }
}

// Modified from acala repo
// docs; https://github.com/AcalaNetwork/Acala/blob/d919a4e3f9f5ed8617f23ade0d7b5302f863e2a9/runtime/mandala/src/lib.rs
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Mint NFT token
        ///
        /// - `to`: the token owner's account
        /// - `class_id`: token belong to the class id
        /// - `metadata`: external metadata
        /// - `quantity`: token quantity
        ///
        /// # <weight>
        /// - Preconditions:
        /// 	- T::Currency is orml_currencies
        /// - Complexity: `O(1)`
        /// - Db reads: 4
        /// - Db writes: 4
        /// -------------------
        /// Base Weight:
        ///		- best case: 202 µs
        ///		- worst case: 208 µs
        /// # </weight>
        #[weight = 1_000 + T::DbWeight::get().writes(1)]
        pub fn create_nft(origin, metadata: CID, data: <T as orml_nft::Trait>::ClassData) -> dispatch::DispatchResult{
            let who = ensure_signed(origin)?;
            let res = <orml_nft::Module<T>>::create_class(&who,metadata.clone(),data);


            <NClassId<T>>::insert(&who,res.unwrap());
            <NTokenMetaData<T>>::insert(&who,metadata);
            Self::deposit_event(RawEvent::TokenIssuedBy(who));


            Ok(())
        }

        /// Transfer NFT token to another account
        ///
        /// - `to`: the token owner's account
        /// - `token`: (class_id, token_id)
        ///
        /// # <weight>
        /// - Preconditions:
        /// 	- T::Currency is orml_currencies
        /// - Complexity: `O(1)`
        /// - Db reads: 3
        /// - Db writes: 3
        /// -------------------
        /// Base Weight:
        ///		- best case: 97.81 µs
        ///		- worst case: 99.99 µs
        /// # </weight>
        #[weight = 1_000 + T::DbWeight::get().writes(1)]
        pub fn mint_nft(origin, data: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult{
            let who = ensure_signed(origin)?;
            let metadata = <NTokenMetaData<T>>::get(&who);
            let classid = <NClassId<T>>::get(&who);

            let _res = <orml_nft::Module<T>>::mint(&who,classid,metadata,data);

            Self::deposit_event(RawEvent::TokenMinted(who));
            Ok(())
        }

        /// Burn NFT token
        ///
        /// - `token`: (class_id, token_id)
        ///
        /// # <weight>
        /// - Preconditions:
        /// 	- T::Currency is orml_currencies
        /// - Complexity: `O(1)`
        /// - Db reads: 5
        /// - Db writes: 5
        /// -------------------
        /// Base Weight:
        ///		- best case: 261.2 µs
        ///		- worst case: 261.4 µs
        /// # </weight>
        #[weight = 1_000 + T::DbWeight::get().writes(1)]
        pub fn burn(origin, token: (<T as orml_nft::Trait>::ClassId, <T as orml_nft::Trait>::TokenId)) {
            let who = ensure_signed(origin)?;

            orml_nft::Module::<T>::burn(&who, token)?;


            Self::deposit_event(RawEvent::BurnedToken(who,token.0,token.1));
        }


    }
}
