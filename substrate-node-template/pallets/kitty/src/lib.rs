#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::DispatchResult;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
pub type Id = u32;
use sp_runtime::ArithmeticError;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;
	#[derive(TypeInfo, Encode, Decode, Clone, RuntimeDebug, PartialEq, Copy, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}
	#[derive(TypeInfo, Encode, Decode, Clone, RuntimeDebug, PartialEq)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u64,
		gender: Gender,
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub(super) type Kitties<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty: Vec<u8> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		TooManyOwned,
		NotKitty,
		NotOwner,
		TransferToSelf,
		PriceTooLow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u64) -> DispatchResult {
			ensure!(price > 100, Error::<T>::PriceTooLow);
			
			let owner = ensure_signed(origin)?;
			let gender = Self::gen_gender(dna.clone())?;

			let kitty  = Kitty::<T> {
				dna: dna.clone(),
				owner: owner.clone(),
				price: price ,
				gender: gender,
			};
			
			//check kitty is not duplicate
			
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			let current_id = KittyId::<T>::get();

			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			KittiesOwned::<T>::append(&owner, kitty.dna.clone());

			Kitties::<T>::insert(kitty.dna.clone(), kitty.clone());

			KittyId::<T>::put(next_id);
			Self::deposit_event(Event::Created { kitty: kitty.dna, owner: kitty.owner });

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: Vec<u8>) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NotKitty)?;
			
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			ensure!(kitty.owner != to, Error::<T>::TransferToSelf);

			let mut from_owned = KittiesOwned::<T>::get(&from);

			if let Some(pos) = from_owned.iter().position(|ids| *ids == dna) {
				from_owned.swap_remove(pos);
			}
			else{
				return Err(Error::<T>::NotKitty.into());
			}

			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.push(dna.clone());

			kitty.owner = to.clone();

			Kitties::<T>::insert(dna.clone(), kitty.clone());
			KittiesOwned::<T>::insert(&from, from_owned);
			KittiesOwned::<T>::insert(&to, to_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna });

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			Ok(())
		}
	}
}
impl<T: Config> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 == 0 {
			res = Gender::Female;
		} else {
			res = Gender::Male;
		}
		Ok(res)
	}
}
