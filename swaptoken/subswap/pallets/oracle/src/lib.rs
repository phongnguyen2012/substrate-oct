#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, ValueQuery, OptionQuery};
	use frame_system::pallet_prelude::{*, OriginFor};
	use sp_std::vec::Vec;


	use pallet_token as token;

	// Uniquely identify a request's specification understood by an Operator
	pub type SpecIndex = Vec<u8>;
	// Uniquely identify a request for a considered Operator
	pub type RequestIdentifier = u64;
	// The version of the serialized data format
	pub type DataVersion = u64;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + token::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// the result of the oracle call
	#[pallet::storage]
	#[pallet::getter(fn get_result)]
	pub type Result<T> = StorageValue<_, i128, ValueQuery>;

	// A set of all registered Operator
	#[pallet::storage]
	#[pallet::getter(fn operator)]
	pub(super) type Operators<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		bool, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn asset_price)]
	pub(super) type Prices<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		T::Balance, 
		OptionQuery
	>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A request has been accepted. Corresponding fee paiement is reserved
		OracleRequest(T::AccountId, SpecIndex, RequestIdentifier, T::AccountId, DataVersion, Vec<u8>, Vec<u8>, T::Balance),

		// A request has been answered. Corresponding fee paiement is transfered
		OracleAnswer(T::AccountId, RequestIdentifier, T::AccountId, Vec<u8>, T::Balance),

		// A new operator has been registered
		OperatorRegistered(T::AccountId),

		// An existing operator has been unregistered
		OperatorUnregistered(T::AccountId),

		// A request didn't receive any result in time
		KillRequest(RequestIdentifier),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
        StorageOverflow,
        // Manipulating an unknown operator
		UnknownOperator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected operator
		WrongOperator,
		// An operator is already registered.
		OperatorAlreadyRegistered,
		// Callback cannot be deserialized
		UnknownCallback,
		// Fee provided does not match minimum required fee
		InsufficientFee,
		// Price does not exist
		PriceDoesNotExist,
	}
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// REVIEW: Use `///` instead of `//` to make these doc comments that are part of the crate documentation.
		// Register a new Operator.
		// Fails with `OperatorAlreadyRegistered` if this Operator (identified by `origin`) has already been registered.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn register_operator(origin: OriginFor<T>) -> DispatchResult {
			let who: T::AccountId = ensure_signed(origin)?;

			ensure!(!<Operators<T>>::get(&who), Error::<T>::OperatorAlreadyRegistered);

			Operators::<T>::insert(&who, true);

			Self::deposit_event(Event::OperatorRegistered(who));

			Ok(())
		}


		// Unregisters an existing Operator
		// TODO check weight
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn unregister_operator(origin: OriginFor<T>) -> DispatchResult {
			let who: T::AccountId = ensure_signed(origin)?;

			if Operators::<T>::take(who.clone()) {
				Self::deposit_event(Event::OperatorUnregistered(who));
				Ok(())
			} else {
				Err(Error::<T>::UnknownOperator.into())
			}
        }

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
        pub fn report(origin: OriginFor<T>, id: T::AssetId, price: T::Balance) -> DispatchResult {
            let who: T::AccountId = ensure_signed(origin)?;
			ensure!(Operators::<T>::contains_key(who), Error::<T>::WrongOperator);
			Prices::<T>::insert(id, price);
			Ok(())
        }
	}

	// The main implementation block for the module.
	impl<T: Config> Pallet<T>{
		pub fn price(id: T::AssetId) -> sp_std::result::Result<T::Balance, DispatchError> {
			match Self::asset_price(id) {
				Some(x) => {
					return Ok(x)
				},
				None => {
					return Err(DispatchError::from(crate::Error::<T>::PriceDoesNotExist).into());
				}
			}
			
		}
	}
}
