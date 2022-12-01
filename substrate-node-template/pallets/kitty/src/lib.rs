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
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;


#[frame_support::pallet]
pub mod pallet {
	

	pub use super::*;
	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male

		}
	}
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender
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

	pub type Id = u32;
	

	#[pallet::storage]
	pub type KittyId<T> = StorageValue<_, Id,ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	pub(super) type OwnerKitty<T: Config> = StorageMap<_, Blake2_128Concat,T::AccountId , Vec<u8>, ValueQuery>;
	

	#[pallet::storage]
	pub(super) type Kittydna<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		// KittyStored(Vec<u8>,u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		
		PricetoLow,
		SenderNotOwner,
		AllreadyOwned,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			ensure!(price > 100, Error::<T>::PricetoLow);
			let who = ensure_signed(origin)?;
			let kitty_id = <KittyId<T>>::get();
			let new_gender = Self::gen_gender(dna.clone())?;
			let new_kitty = Kitty {
				dna: dna.clone(),
				owner: who.clone(),
				price: price.clone(),
				gender: new_gender
			};
			let mut current_kittyid = <KittyId<T>>::get();
			<Kitties<T>>::insert(kitty_id, &new_kitty);
			
			current_kittyid += 1;
			<KittyId<T>>::put(current_kittyid);
			OwnerKitty::<T>::insert(who.clone(),dna.clone());
			Kittydna::<T>::insert(dna.clone(),&new_kitty);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn change_owner_kitty(origin: OriginFor<T>, kitty_id: u32, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty = <Kitties<T>>::get(kitty_id.clone()).unwrap();
			ensure!(kitty.owner == who, Error::<T>::AllreadyOwned);
			
			let new_kitty = Kitty {
				dna: kitty.dna.clone(),
				owner: new_owner.clone(),
				price: kitty.price,
				gender: kitty.gender
			};
			<Kitties<T>>::insert(kitty_id, new_kitty);
			Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn change_owner_dna(origin: OriginFor<T>, dna: Vec<u8>, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty = <Kittydna<T>>::get(dna.clone()).unwrap();
			ensure!(kitty.owner == who, Error::<T>::AllreadyOwned);
			
			let new_kitty = Kitty {
				dna: dna.clone(),
				owner: new_owner.clone(),
				price: kitty.price,
				gender: kitty.gender
			};
			<Kittydna<T>>::insert(dna.clone(), new_kitty);

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
	fn gen_gender(dna: Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if dna.len() % 2 ==0 {
			res = Gender::Female;
		}
		else{
			res= Gender::Male;
		}
		Ok(res)
	}

}