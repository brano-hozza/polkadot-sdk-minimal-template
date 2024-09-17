//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
use weights::WeightInfo;
// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type WeightInfo: WeightInfo;
    }

    pub type Balance = u128;

    #[pallet::storage]
    pub(crate) type TotalIssuance<T: Config> = StorageValue<_, Balance>;

    #[pallet::storage]
    pub(crate) type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Balance>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An unsafe mint that can be called by anyone. Not a great idea.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::mint_unsafe())]
        pub fn mint_unsafe(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            // ensure that this is a signed account, but we don't really check `_anyone`.
            let _anyone = ensure_signed(origin)?;

            // update the balances map. Notice how all `<T: Config>` remains as `<T>`.
            Balances::<T>::mutate(dest, |b| *b = Some(b.unwrap_or(0) + amount));
            // update total issuance.
            TotalIssuance::<T>::mutate(|t| *t = Some(t.unwrap_or(0) + amount));

            Ok(())
        }

        /// Transfer `amount` from `origin` to `dest`.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // ensure sender has enough balance, and if so, calculate what is left after `amount`.
            let sender_balance = Balances::<T>::get(&sender).ok_or("NonExistentAccount")?;
            ensure!(sender_balance >= amount, "InsufficientBalance");

            let remainder = sender_balance - amount;

            // update sender and dest balances.
            Balances::<T>::mutate(dest, |b| *b = Some(b.unwrap_or(0) + amount));
            Balances::<T>::insert(&sender, remainder);

            Ok(())
        }
    }
}
