#![cfg_attr(not(feature = "std"), no_std)]

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
use sp_runtime::traits::Hash;
use pallet_timestamp as Timestamp;
use frame_support::traits::{ Get, Currency, Randomness};
use sp_runtime::SaturatedConversion;
// use pallet_randomness_collective_flip as Random;
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


// use frame_support::dispatch::fmt::Debug;
// use pallet_balances ;



#[frame_support::pallet]
pub mod pallet {


pub use super::*;
	#[derive(TypeInfo, Encode, Decode, Clone, RuntimeDebug, PartialEq, Copy, MaxEncodedLen)]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(TypeInfo, Encode, Decode, Clone, RuntimeDebug, PartialEq)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: T::Hash,
		pub price: BalanceOf<T>,
		pub gender: Gender,
		pub owner: T::AccountId,
		pub create_date: T::Moment,
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + Timestamp::Config  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		type MaxOwned: Get<u32>;
		type RandomKitty: Randomness<Self::Hash, Self::BlockNumber>;
		// type Balance: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::Hash, T::MaxOwned>, ValueQuery>;
		// BoundedVec<T::Hash, T::MaxOwned>

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty: T::Hash },
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
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: BalanceOf<T>) -> DispatchResult {
			let convert_price = price.saturated_into::<u64>();
			 ensure!(convert_price > 100u64, Error::<T>::PriceTooLow);
			
			let owner = ensure_signed(origin)?;
			let new_gender = Self::gen_gender(&dna)?;
			let new_dna = Self::gen_random();
			let now = <Timestamp::Pallet<T>>::get();
			let kitty  = Kitty::<T> {
				dna: new_dna.clone(),
				owner: owner.clone(),
				price: price ,
				gender: new_gender,
				create_date: now,
			};

			let maxkitty = T::MaxOwned::get();
			let get_kitty = KittiesOwned::<T>::get(owner.clone());
			ensure!(get_kitty.len() < maxkitty as usize, Error::<T>::TooManyOwned);
			
			//check kitty is not duplicate
			
			ensure!(!Kitties::<T>::contains_key(kitty.dna), Error::<T>::DuplicateKitty);

			let current_id = KittyId::<T>::get();

			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			//why try_append is error when not use BanlanceOf
			KittiesOwned::<T>::try_append(&owner, kitty.dna.clone()).map_err(|_| Error::<T>::TooManyOwned)?;
			// KittiesOwned::<T>::append(&owner, kitty.dna.clone());

			Kitties::<T>::insert(kitty.dna.clone(), kitty);

			KittyId::<T>::put(next_id);
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: T::Hash) -> DispatchResult {
			 
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
			to_owned.try_push(dna.clone()).map_err(|_| Error::<T>::TooManyOwned)?;

			kitty.owner = to.clone();

			Kitties::<T>::insert(dna.clone(), kitty.clone());
			KittiesOwned::<T>::insert(&from, from_owned);
			KittiesOwned::<T>::insert(&to, to_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna.clone() });

			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_price(origin: OriginFor<T>, dna: T::Hash, price: BalanceOf<T>) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NotKitty)?;
			
			ensure!(kitty.owner == owner, Error::<T>::NotOwner);
			kitty.price = price;
			Kitties::<T>::insert(dna.clone(), kitty.clone());
			Ok(())
		}
		
	
	}
}
impl<T: Config> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 == 0 {
			res = Gender::Female;
		} 
		Ok(res)
	}
	fn gen_random() -> T::Hash {
		let (seed, _) = T::RandomKitty::random_seed();
		let block_number= <frame_system::Pallet<T>>::block_number();
		T::Hashing::hash_of(&(seed, block_number))
		
	}
	// pub fn random_dna() -> DispatchResult {
	// 	let (seed, _) = T::RandomKitty::random_seed();
	// 	let block_number= <frame_system::Pallet<T>>::block_number();
	// 	let dna = T::Hashing::hash_of(&(seed, block_number));
	// 	Ok(())
	// }
}
