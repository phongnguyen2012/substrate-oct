#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, OptionQuery, ValueQuery},PalletId,{traits::Currency}};
	use frame_system::{pallet_prelude::{*, OriginFor},weights::WeightInfo};
	use sp_runtime::traits::{AtLeast32Bit,AccountIdConversion,Zero,One,StaticLookup};
	use sp_core::Get;
	use pallet_balances;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type PalletId: Get<PalletId>;
		type AssetId: Parameter + AtLeast32Bit + Default + Copy + MaxEncodedLen;
		type WeightInfo: WeightInfo;

		type Currency: Currency<Self::AccountId>;
	}
	// pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::storage]
	pub(super) type Balances<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, (T::AssetId, T::AccountId), 
		T::Balance, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub type NextAssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub(super) type TotalSupply<T: Config> = StorageMap<
		_, 
		Twox64Concat, T::AssetId, 
		T::Balance, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn creator)]
	pub(super) type Creator<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		T::AccountId, 
		OptionQuery
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Issued(T::AssetId, T::AccountId, T::Balance),
        /// Some assets were issued by the system(e.g. lpt, pool tokens) \[asset_id, total_supply]
        IssuedBySystem(T::AssetId, T::Balance),
        /// Some assets were transferred. \[asset_id, from, to, amount\]
        Transferred(T::AssetId, T::AccountId, T::AccountId, T::Balance),
        TransferredFromSystem(T::AssetId, T::AccountId, T::Balance),
        TransferredToSystem(T::AssetId, T::AccountId, T::Balance),
        /// Some assets were minted. \[asset_id, owner, balance]
        Minted(T::AssetId, T::AccountId, T::Balance),
        /// Some assets were burned. \[asset_id, owner, balance]
        Burned(T::AssetId, T::AccountId, T::Balance),
        /// Some assets were destroyed. \[asset_id, owner, balance\]
		Destroyed(T::AssetId, T::AccountId, T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// Balance should be non-zero
        BalanceZero,
        /// Not the creator of the asset
        NotTheCreator,
        /// Not the approver for the account
        NotApproved,
        /// Created by System
		CreatedBySystem,
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Issue a new class of fungible assets. There are, and will only ever be, `total`
		/// such assets and they'll all belong to the `origin` initially. It will have an
		/// identifier `AssetId` instance: this will be specified in the `Issued` event.
		
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn issue(origin: OriginFor<T>, total: T::Balance) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// save 0 for native currency
			let id = Self::next_asset_id();
			<NextAssetId<T>>::mutate(|id| {
                *id += One::one();
            });

			<Balances<T>>::insert((id, &origin), total);
			<TotalSupply<T>>::insert(id, total);
			<Creator<T>>::insert(id, &origin);

			Self::deposit_event(Event::Issued(id, origin, total));
			Ok(())
		}
		

		/// Mint any assets of `id` owned by `origin`.
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
        pub fn mint(origin: OriginFor<T>,
            id: T::AssetId,
            target: <<T as frame_system::Config>::Lookup as StaticLookup>::Source,
            amount: T::Balance
        )-> DispatchResult{
            let origin = ensure_signed(origin)?;
            let target = T::Lookup::lookup(target)?;
            let creator = <Creator<T>>::get(id).unwrap();
            ensure!(origin == creator, Error::<T>::NotTheCreator);
            ensure!(!amount.is_zero(), Error::<T>::AmountZero);

            <Balances<T>>::mutate((id, target.clone()), |balance| *balance += amount);
            <TotalSupply<T>>::mutate(id, |supply| *supply += amount);
            Self::deposit_event(Event::Minted(id, target, amount));
			Ok(())
        }

		/// Burn any assets of `id` owned by `origin`.
        #[pallet::call_index(3)]
		#[pallet::weight(0)]
        pub fn burn(origin: OriginFor<T>,
        	id: T::AssetId,
           	target: <<T as frame_system::Config>::Lookup as StaticLookup>::Source,
           	amount: T::Balance
       	)-> DispatchResult{
           let origin = ensure_signed(origin)?;
           let origin_account = (id, origin.clone());
           let origin_balance = <Balances<T>>::get(&origin_account);
           ensure!(!amount.is_zero(), Error::<T>::AmountZero);
           ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

           <Balances<T>>::insert(origin_account, origin_balance - amount);
           <TotalSupply<T>>::mutate(id, |supply| *supply -= amount);
           Self::deposit_event(Event::Burned(id, origin, amount));
		   Ok(())
       }

	   /// Move some assets from one holder to another.
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
        pub fn transfer(origin: OriginFor<T>,
			id: T::AssetId,
			target: <<T as frame_system::Config>::Lookup as StaticLookup>::Source,
			amount: T::Balance
		) -> DispatchResult{
			let origin = ensure_signed(origin)?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

			Self::deposit_event(Event::Transferred(id, origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate((id, target), |balance| *balance += amount);
			Ok(())
		}

		/// Destroy any assets of `id` owned by `origin`.
        #[pallet::call_index(5)]
		#[pallet::weight(0)]
        pub fn destroy(origin: OriginFor<T>, id: T::AssetId) -> DispatchResult{
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, &origin));
			ensure!(!balance.is_zero(), Error::<T>::BalanceZero);

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			Self::deposit_event(Event::Destroyed(id, origin, balance));
			Ok(())
		}



	}

	impl<T: Config> Pallet<T> {
		// Module account id
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		// Get the asset `id` balance of `who`.
		pub fn balance(id: T::AssetId, who: T::AccountId) -> T::Balance {
			if id == Zero::zero() {
				return pallet_balances::Pallet::<T>::free_balance(&who);

			}
			<Balances<T>>::get((id, who))
		}
		
		// Get the total supply of an asset `id`.
		// pub fn total_supply(id: T::AssetId) -> T::Balance {
		// 	TotalSupply::<T>::get(id)
		// }

		pub fn mint_from_system(
			id: &T::AssetId,
			target: &T::AccountId,
			amount: &T::Balance,
		) -> DispatchResult {
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			if *id == Zero::zero() {
				let new_free = pallet_balances::Pallet::<T>::free_balance(target) + *amount;
				pallet_balances::Pallet::<T>::mutate_account(target, |account| {
					account.free = new_free;
					account.free
				});
			} else {
				<Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
				<TotalSupply<T>>::mutate(*id, |supply| *supply += *amount);
			}
			Self::deposit_event(Event::Minted(*id, target.clone(), *amount));
			Ok(())
		}

		pub fn burn_from_system(
			id: &T::AssetId,
			target: &T::AccountId,
			amount: &T::Balance,
		) -> DispatchResult {
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			if *id == Zero::zero() {
				let new_free = pallet_balances::Pallet::<T>::free_balance(target) - *amount;
				let _free = pallet_balances::Pallet::<T>::mutate_account(target, |account| {
					account.free = new_free;
	
					account.free
				});
			} else {
				<Balances<T>>::mutate((*id, target.clone()), |balance| *balance -= *amount);
				<TotalSupply<T>>::mutate(*id, |supply| *supply -= *amount);
			}
			Self::deposit_event(Event::Burned(*id, target.clone(), *amount));
			Ok(())
		}

		pub fn transfer_system(
			id: &T::AssetId,
			source: &T::AccountId,
			target: &T::AccountId,
			amount: &T::Balance,
		) -> DispatchResult {
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			Self::deposit_event(Event::Transferred(*id, source.clone(), target.clone(), *amount));
			if *id == Zero::zero() {
				pallet_balances::Pallet::<T>::mutate_account(source, |account| {
					account.free -= *amount;
				});
				pallet_balances::Pallet::<T>::mutate_account(target, |account| {
					account.free += *amount;
				});
			} else {
				<Balances<T>>::mutate((*id, target), |balance| *balance += *amount);
				<Balances<T>>::mutate((*id, source), |balance| *balance -= *amount);
			}
			Ok(())
		}

		pub fn transfer_from_system(
			id: &T::AssetId,
			target: &T::AccountId,
			amount: &T::Balance,
		) -> DispatchResult {
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			let module_account = Self::account_id();
			Self::deposit_event(Event::TransferredFromSystem(*id, target.clone(), *amount));
			if *id == Zero::zero() {
				pallet_balances::Pallet::<T>::mutate_account(&module_account, |account| {
					account.free -= *amount;
				});
				pallet_balances::Pallet::<T>::mutate_account(target, |account| {
					account.free += *amount;
				});
			} else {
				<Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
				<Balances<T>>::mutate((*id, module_account), |balance| *balance -= *amount);
			}
			Ok(())
		}



		pub fn transfer_to_system(
			id: &T::AssetId,
			source: &T::AccountId,
			amount: &T::Balance,
		) -> DispatchResult {
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			let module_account = Self::account_id();
			Self::deposit_event(Event::TransferredToSystem(*id, source.clone(), *amount));
			if *id == Zero::zero() {
				pallet_balances::Pallet::<T>::mutate_account(source, |account| {
					account.free -= *amount;
				});
				pallet_balances::Pallet::<T>::mutate_account(&module_account, |account| {
					account.free += *amount;
				});
			} else {
				<Balances<T>>::mutate((*id, source.clone()), |balance| *balance -= *amount);
				<Balances<T>>::mutate((*id, module_account), |balance| *balance += *amount);
			}
			Ok(())
		}

		pub fn issue_from_system(total: T::Balance) -> DispatchResult {
			let id = Self::next_asset_id();
			let module_account  = Self::account_id();
			<NextAssetId<T>>::mutate(|id| {
				*id += One::one();
			});
			<TotalSupply<T>>::insert(id, total);
			<Balances<T>>::insert((id, module_account), total.clone()); 
			Self::deposit_event(Event::IssuedBySystem(id, total));
			Ok(())
		}
	}
}

