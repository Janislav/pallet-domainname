#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::traits::{Currency, LockableCurrency, ExistenceRequirement};
use sp_std::vec::Vec;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}

decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
		Domains get(fn domains): map hasher(blake2_128_concat) Vec<u8> => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		Registered(Vec<u8>, AccountId),
		Unregistered(Vec<u8>, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		DomainAlreadyUsed,
		AccountNotFound,
		DomainNotFound,
		NotTheOwnerOfDomain,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn register(origin, domain: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			if Domains::<T>::contains_key(&domain) {
				Err(Error::<T>::DomainAlreadyUsed)?
			}
			Domains::<T>::insert(domain.clone(), sender.clone());
			Self::deposit_event(RawEvent::Registered(domain, sender));
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn send(origin, amount: BalanceOf<T>, domain: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			if !Domains::<T>::contains_key(&domain) {
				Err(Error::<T>::DomainNotFound)?
			}
			let reciver = Domains::<T>::get(&domain);

			T::Currency::transfer(&sender, &reciver, amount, ExistenceRequirement::KeepAlive)
			Ok(())
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn unregister(origin, domain: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			if !Domains::<T>::contains_key(&domain) {
				Err(Error::<T>::DomainNotFound)?
			}
			let owner = Domains::<T>::get(&domain);
			if sender != owner {
				Err(Error::<T>::NotTheOwnerOfDomain)?
			}
			Domains::<T>::remove(&domain);
			Self::deposit_event(RawEvent::Unregistered(domain, sender));
			Ok(())
		}
	}
}
