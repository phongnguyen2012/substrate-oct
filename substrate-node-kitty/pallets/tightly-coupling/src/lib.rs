#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::pallet_prelude::*;
use pallet_template::Something;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_template::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		AccessStorage(u32),
		UpdateStorage
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn access_storage(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let data = pallet_template::Pallet::<T>::something().unwrap();
			
			Self::deposit_event(Event::AccessStorage(data));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn modify_storage(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let old_value = pallet_template::Pallet::<T>::something().unwrap();
			let newvalue = old_value + new_value;
			pallet_template::Pallet::<T>::update_storage(newvalue);
			Something::<T>::put(newvalue);
			Self::deposit_event(Event::UpdateStorage);

			Ok(())
		}
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			Ok(())
		}
	}
}
