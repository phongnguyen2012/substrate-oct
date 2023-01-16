#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, OptionQuery}, Blake2_128Concat};
	use frame_system::pallet_prelude::{*, OriginFor};
	use sp_core::U256;
	use pallet_market as market;
	use pallet_token as token;
	use pallet_oracle as oracle;
	use frame_support::PalletId;
	use sp_runtime::traits::UniqueSaturatedInto;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CDP {
		/// Percentage of liquidator who liquidate the cdp \[numerator, denominator]
		liquidation_fee: (U256, U256),
		/// Maximum collaterization rate \[numerator, denominator]
		max_collateraization_rate: (U256, U256),
		/// Fee paid for stability \[numerator, denominator]
		stability_fee: (U256, U256)
	}	

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + market::Config + token::Config + oracle::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		 /// The Module account for burning assets
		type VaultPalletId: Get<PalletId>;
	}

	
	#[pallet::storage]
	#[pallet::getter(fn vault)]
	pub type Vault<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, 
		(T::AccountId, T::AssetId),
		(T::Balance, T::Balance), 
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn position)]
	pub(super) type Positions<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		CDP, 
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn circulating_supply)]
	pub type CirculatingSupply<T: Config> = StorageValue<
		_, 
		T::Balance, 
		ValueQuery
	>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A vault is created with the collateral. \[who, collateral, collateral_amount, meter_amount]
		UpdateVault(T::AccountId, T::AssetId, T::Balance, T::Balance), 
		/// A vault is liquidated \[collateral, collateral_amount]
		Liquidate(T::AssetId, T::Balance),
		/// Close vault by paying back meter. \[collateral, collateral_amount, paid_meter_amount]
		CloseVault(T::AssetId, T::Balance, T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// No value
		NoneValue,
        /// Collateral is not supported
        CollateralNotSupported,
        /// Invalid CDP
        InvalidCDP,
        /// Unavailable to Liquidate
        Unavailable
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn generate(
            origin: OriginFor<T>,
            request_amount: T::Balance,
            collateral_id: T::AssetId, 
            collateral_amount: T::Balance) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            // Get position for the collateral
            let position = Self::position(collateral_id);
            //ensure!(position.is_some(), Error::<T>::CollateralNotSupported);
            // Get price from oracles
            let collateral_price = oracle::Pallet::<T>::price(collateral_id)?;
            let mtr_price = oracle::Pallet::<T>::price(T::AssetId::from(1u32))?;
            // Get vault from sender and divide cases
            let (total_collateral, total_request) = match Self::vault((origin.clone(), collateral_id)) {
                // vault exists for the sender
                Some(x) => {
                    // Add collateral and mtr amount from existing vault
                    let collateral_total = collateral_amount + x.0;
                    let request_total = request_amount + x.1;  
                    (collateral_total, request_total)
                },
                // vault does not exist for the sender
                None => {
                    (collateral_amount, request_amount)
                }
            };

            let result = Self::is_cdp_valid(&position.unwrap(), &collateral_price, &total_collateral, &mtr_price, &total_request);
            // Check whether CDP is valid
            ensure!(result, Error::<T>::InvalidCDP);
            
            // Send collateral to Standard Protocol
            token::Pallet::<T>::transfer_to_system(&collateral_id, &origin, &collateral_amount)?;

            // Update CDP
            <Vault<T>>::mutate((origin.clone(), collateral_id), |vlt|{
                *vlt = Some((total_collateral, total_request));
            });

            // Send mtr to sender
            token::Pallet::<T>::transfer_to_system(&T::AssetId::from(1u32), &origin, &request_amount)?;

            // deposit event
            Self::deposit_event(Event::UpdateVault(origin, collateral_id, total_collateral, request_amount));
			Ok(())
        }


		//Chỗ này code trong code tham khảo chưa hoàn thiện.
		// #[weight=0]
        // fn liquidate(origin, #[compact] account: T::AccountId, #[compact] collateral_id: T::AssetId) {
        //     let origin = ensure_signed(origin)?;
        //     let vault = 
        // }
	}

	impl<T: Config> Pallet<T> {
		fn is_cdp_valid(position: &CDP, collateral_price: &T::Balance, collateral_amount: &T::Balance, request_price: &T::Balance, request_amount: &T::Balance) -> bool {
			let collateral_price_256 = Self::to_u256(&collateral_price);
			let mtr_price_256 = Self::to_u256(&request_price);
			let total_collateral_256 = Self::to_u256(&collateral_amount);
			let collateral = collateral_price_256.checked_mul(total_collateral_256).expect("Multiplication overflow");
			let total_request_256 = Self::to_u256(&request_amount);
			let request = mtr_price_256.checked_mul(total_request_256).expect("Multiplication overflow");
			let determinant = collateral.checked_div(position.max_collateraization_rate.1).expect("divided by zero").checked_mul(position.max_collateraization_rate.0).unwrap_or(U256::max_value());
			request < determinant
		}
		
		pub fn to_u256(value: &T::Balance) -> U256 {
			U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(*value))
		}
	}
}
