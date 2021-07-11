  //! # Domainname
//! The module provides implementations for enable you blockchain to use domainnames.
//!
//! ## Overview
//!
//! This module provides basic functions to create and manager
//! NFT(non fungible token) such as `create_class`, `transfer`, `mint`, `burn`.

//! ### Module Extrinsics
//!
//! - `register` - Registers a domainname
//! - `unregister` - Unregisters a domainname
//! - `send` - Sends funds by given domainname

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::traits::{Currency, ExistenceRequirement};
use sp_std::vec::Vec;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[cfg(test)]
mod tests;

/// Domain name pallet config
pub trait Config: frame_system::Config {
	/// Event trait
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// Currency trait
	type Currency: Currency<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as TemplateModule {
		/// Domains exisiting on the chain
		Domains get(fn domains): map hasher(blake2_128_concat) Vec<u8> => T::AccountId;
	}
	add_extra_genesis {
		build(|_config| {
		})
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// A domain was registered
		Registered(Vec<u8>, AccountId),
		/// A domain was unregistered
		Unregistered(Vec<u8>, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Domain is already used
		DomainAlreadyUsed,
		/// The account has no registered domain
		AccountNotFound,
		/// The domain dosen't exist
		DomainNotFound,
		/// The user is not the owner of the domain
		NotTheOwnerOfDomain,
		/// An error occured during the transfer of funds
		TransferError,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Registers a domain name for a given user
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

		/// Sends funds to an account by a given domain name
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn send(origin, amount: BalanceOf<T>, domain: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			if !Domains::<T>::contains_key(&domain) {
				Err(Error::<T>::DomainNotFound)?
			}
			let reciver = Domains::<T>::get(&domain);
			T::Currency::transfer(&sender, &reciver, amount, ExistenceRequirement::KeepAlive)
		}

		/// Unregisters a domain name
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
