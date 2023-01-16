#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
mod math;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::{FixedU128,traits::{UniqueSaturatedInto,UniqueSaturatedFrom,CheckedMul, CheckedAdd, CheckedDiv, CheckedSub, Zero, One}};
	use pallet_token as token;
	use sp_core::U256;
	use crate::math;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + token::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
	
	#[pallet::storage]
	#[pallet::getter(fn last_cumulative_price)]
	pub(super) type LastAccumulativePrice<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		(FixedU128, FixedU128), 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn reward)]
	pub(super) type Rewards<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		(T::AssetId, T::AssetId), 
		ValueQuery
	>;


	#[pallet::storage]
	#[pallet::getter(fn reserves)]
	pub(super) type Reserves<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AssetId, 
		(T::Balance, T::Balance), 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn pair)]
	pub(super) type Pairs<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, (T::AssetId,T::AssetId), 
		T::AssetId, 
		OptionQuery
	>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pair between two assets is created. \[token0, token1, lptoken]
		CreatePair(T::AssetId, T::AssetId, T::AssetId),
		/// An asset is swapped to another asset. \[token0, amount_in, token1, amount_out]
		Swap(T::AssetId, T::Balance, T::AssetId, T::Balance),
		/// Liquidity is minted. \[token0, token1, lptoken]
		MintedLiquidity(T::AssetId, T::AssetId, T::AssetId),
		/// Liquidity is burned. \[lptoken, token0, token1]
		BurnedLiquidity(T::AssetId, T::AssetId, T::AssetId),
		/// Sync oracle. \[price0, price1]
        SyncOracle(FixedU128, FixedU128),
	}


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
		/// No value
		NoneValue,
		/// Insufficient balance
		InSufficientBalance,
		/// Pair already exists
		PairExists,
		/// Lp token id already exists
		LptExists,
		/// Invalid pair
		InvalidPair,
		/// Pair with identical identifiers
		IdenticalIdentifier,
		/// Insufficient liquidity minted
		InsufficientLiquidityMinted,
		/// Insufficient liquidity burned
		InsufficientLiquidityBurned,
		/// Insufficient output amount for swap
		InsufficientOutputAmount,
		/// Insufficient amont for swap
		InsufficientAmount,
		/// Insufficiient liquidity for swap
        InsufficientLiquidity,
        /// The ratio does not match from previous K
        K,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
        #[pallet::weight(0)]
		pub fn mint_liquidity(origin: OriginFor<T>, token0: T::AssetId, amount0: T::Balance, token1: T::AssetId, amount1: T::Balance) -> DispatchResult {
            let minimum_liquidity = T::Balance::from(1u32);
            let sender = ensure_signed(origin)?;
            ensure!(token0 != token1, Error::<T>::IdenticalIdentifier);
            // Burn assets from user to deposit to reserves
            token::Pallet::<T>::transfer_to_system(&token0, &sender, &amount0)?;
            token::Pallet::<T>::transfer_to_system(&token1, &sender, &amount1)?;
            match Pairs::<T>::get((token0.clone(), token1.clone())) {
                // create pair if lpt does not exist
                None => {
                    let mut lptoken_amount: T::Balance = math::sqrt::<T>(amount0 * amount1);
                    lptoken_amount = lptoken_amount.checked_sub(&minimum_liquidity).expect("Integer overflow");
                    // Issue LPtoken
                    token::Pallet::<T>::issue_from_system(Zero::zero())?;
                    let mut lptoken_id: T::AssetId = token::NextAssetId::<T>::get();
                    lptoken_id -= One::one();
                    // Deposit assets to the reserve
                    Self::_set_reserves(&token0, &token1, &amount0, &amount1, &lptoken_id);
                    // Set pairs for swap lookup
                    Self::_set_pair(&token0, &token1, &lptoken_id);
                    Self::_set_rewards(&token0, &token1, &lptoken_id);
                    // Mint LPtoken to the sender
                    token::Pallet::<T>::mint_from_system(&lptoken_id, &sender, &lptoken_amount)?;
                    Self::deposit_event(Event::CreatePair(token0, token1, lptoken_id));
                    Ok(())
                },
                // when lpt exists and total supply is bigger than 0
                Some(lpt) if token::Pallet::<T>::total_supply(lpt) > Zero::zero() => {
                    let total_supply = token::Pallet::<T>::total_supply(lpt);
                    let mut reserves = Self::reserves(lpt);
                    if token0 > token1 {
                        ensure!(math::absdiff::<T>(reserves.0/reserves.1 * amount0, amount1) < amount0.checked_div(&T::Balance::from(1000u32)).expect("Divide by zero error"), Error::<T>::K);
                    } else {
                        ensure!(math::absdiff::<T>(reserves.0/reserves.1 * amount1, amount0) < amount0.checked_div(&T::Balance::from(1000u32)).expect("Divide by zero error"), Error::<T>::K);
                    }
                    let left = amount0.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.0).expect("Divide by zero error");
                    let right = amount1.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.1).expect("Divide by zero error");
                    let lptoken_amount = math::min::<T>(left, right);
                    // Deposit assets to the reserve
                    reserves.0 += amount0;
                    reserves.1 += amount1;
                    Self::_set_reserves(&token0, &token1, &reserves.0, &reserves.1, &lpt);
                    // Mint LPtoken to the sender
                    token::Pallet::<T>::mint_from_system(&lpt, &sender, &lptoken_amount)?;
                    Self::deposit_event(Event::MintedLiquidity(token0, token1, lpt));
                    //Self::_update(&lpt)?;
                    Ok(())
                },
                Some(lpt) if token::Pallet::<T>::total_supply(lpt) < T::Balance::from(0u32) => {
                    Err(Error::<T>::InsufficientLiquidityMinted)?
                },
                Some(_) => Err(Error::<T>::NoneValue)?,
			}
		}


		#[pallet::call_index(1)]
        #[pallet::weight(0)]
		pub fn burn_liquidity(origin: OriginFor<T>, lpt: T::AssetId, amount: T::Balance) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut reserves = Self::reserves(lpt);
            let tokens = Self::reward(lpt);
            let total_supply = token::Pallet::<T>::total_supply(lpt);

            // Calculate rewards for providing liquidity with pro-rata distribution
            let reward0 = amount.checked_mul(&reserves.0).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");
            let reward1 = amount.checked_mul(&reserves.1).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");

            // Ensure rewards exist
            ensure!(reward0 > Zero::zero() && reward1 > Zero::zero(), Error::<T>::InsufficientLiquidityBurned);

            // Distribute reward to the sender
            token::Pallet::<T>::burn_from_system(&lpt, &sender, &amount)?;
            token::Pallet::<T>::transfer_from_system(&tokens.0, &sender, &reward0)?;
            token::Pallet::<T>::transfer_from_system(&tokens.1, &sender, &reward1)?;

            // Update reserve when the balance is set
            reserves.0 -= reward0;
            reserves.1 -= reward1;
            Self::_set_reserves(&tokens.0, &tokens.1, &reserves.0, &reserves.1, &lpt);
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(Event::BurnedLiquidity(lpt, tokens.0, tokens.1));
            // Update price
            //Self::_update(&lpt)?;
            Ok(())
		}

		
		#[pallet::call_index(2)]
        #[pallet::weight(0)]
		pub fn swap(origin: OriginFor<T>, from: T::AssetId, amount_in: T::Balance, to: T::AssetId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(amount_in > Zero::zero(), Error::<T>::InsufficientAmount);
            // Find pair
            let lpt = Self::pair((from, to));
            ensure!(lpt.is_some(), Error::<T>::InvalidPair);
            let reserves = Self::reserves(lpt.unwrap());
            ensure!(reserves.0 > Zero::zero() && reserves.1 > Zero::zero(), Error::<T>::InsufficientLiquidity);
            let (mut reserve_in, mut reserve_out) = match from > to {
                true => (reserves.1, reserves.0),
                false => (reserves.0, reserves.1)
            };
            // get amount out
            let amount_out = Self::_get_amount_out(&amount_in, &reserve_in, &reserve_out);
            // transfer amount in to system
            token::Pallet::<T>::transfer_to_system(&from, &sender, &amount_in)?;
            // transfer swapped amount
            token::Pallet::<T>::transfer_from_system(&to, &sender, &amount_out)?;
            // update reserves
            reserve_in += amount_in;
            reserve_out -= amount_out;
            Self::_set_reserves(&from, &to, &reserve_in, &reserve_out, &lpt.unwrap());
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(Event::Swap(from, amount_in, to, amount_out));
            // Update price
            //Self::_update(&lpt.unwrap())?;
            Ok(())
        }
	}


	impl<T: Config> Pallet<T>  {
		// Market methods
		pub fn _set_reserves(
			token0: &T::AssetId,
			token1: &T::AssetId,
			amount0: &T::Balance,
			amount1: &T::Balance,
			lptoken: &T::AssetId,
		) {
			match *token0 > *token1 {
				true => {
					<Reserves<T>>::insert(*lptoken, (*amount1, *amount0));
				}
				_ => {
					<Reserves<T>>::insert(*lptoken, (*amount0, *amount1));
				}
			}
		}

		fn _set_pair(token0: &T::AssetId, token1: &T::AssetId, lptoken: &T::AssetId) {
			<Pairs<T>>::insert((*token0, *token1), *lptoken);
			<Pairs<T>>::insert((*token1, *token0), *lptoken);
		}

		fn _set_rewards(
			token0: &T::AssetId, token1: &T::AssetId, lptoken: &T::AssetId
		) {
			match *token0 > *token1 {
				true => {
					<Rewards<T>>::insert(*lptoken, (*token1, *token0));
				}
				_ => {
					<Rewards<T>>::insert(*lptoken, (*token0, *token1));
				}
			}
		}

		pub fn to_u256(value: &T::Balance) -> U256 {
			U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(*value))
		}

		pub fn _get_amount_out(
			amount_in: &T::Balance,
			reserve_in: &T::Balance,
			reserve_out: &T::Balance,
		) -> T::Balance {
			let amount_in_256 = Self::to_u256(amount_in);
			let reserve_in_256 = Self::to_u256(reserve_in);
			let reserve_out_256 = Self::to_u256(reserve_out);
			let amount_in_with_fee = amount_in_256
				.checked_mul(U256::from(997))
				.expect("Multiplication overflow");
			let numerator = amount_in_with_fee
				.checked_mul(reserve_out_256)
				.expect("Multiplication overflow");
			let denominator = reserve_in_256
				.checked_mul(U256::from(1000))
				.expect("Multiplication overflow")
				.checked_add(amount_in_with_fee)
				.expect("Overflow");
			T::Balance::unique_saturated_from(numerator.checked_div(denominator).expect("divided by zero").as_u128())
		}
	}
}
