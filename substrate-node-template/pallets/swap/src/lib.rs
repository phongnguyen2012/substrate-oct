// #![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::pallet_prelude::DispatchResult;
// /// Edit this file to define custom logic or remove it if it is not needed.
// /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// /// <https://docs.substrate.io/reference/frame-pallets/>
// pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// #[frame_support::pallet]
// pub mod pallet {
// 	use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;

// 	#[pallet::pallet]
// 	#[pallet::generate_store(pub(super) trait Store)]
// 	pub struct Pallet<T>(_);

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
// 	}

// 	// The pallet's runtime storage items.
// 	// https://docs.substrate.io/main-docs/build/runtime-storage/
// 	#[pallet::storage]
// 	#[pallet::getter(fn something)]
// 	// Learn more about declaring storage items:
// 	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
// 	pub type Something<T> = StorageValue<_, u32>;

// 	// Pallets use events to inform users when important changes are made.
// 	// https://docs.substrate.io/main-docs/build/events-errors/
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored { something: u32, who: T::AccountId },
// 	}

// 	// Errors inform users that something went wrong.
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		/// Error names should be descriptive.
// 		NoneValue,
// 		/// Errors should have helpful documentation associated with them.
// 		StorageOverflow,
// 	}

// 	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// 	// These functions materialize as "extrinsics", which are often compared to transactions.
// 	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {
// 		/// An example dispatchable that takes a singles value as a parameter, writes the value to
// 		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
// 		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://docs.substrate.io/main-docs/build/origins/
// 			let who = ensure_signed(origin)?;

// 			// Update storage.
// 			<Something<T>>::put(something);

// 			// Emit an event.
// 			Self::deposit_event(Event::SomethingStored { something, who });
// 			// Return a successful DispatchResultWithPostInfo
// 			Ok(())
// 		}

// 		/// An example dispatchable that may throw a custom error.
// 		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
// 		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
// 			let _who = ensure_signed(origin)?;

// 			// Read a value from storage.
// 			match <Something<T>>::get() {
// 				// Return an error if the value has not been set.
// 				None => return Err(Error::<T>::NoneValue.into()),
// 				Some(old) => {
// 					// Increment the value read from storage; will error in the event of overflow.
// 					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
// 					// Update the value in storage with the incremented result.
// 					<Something<T>>::put(new);
// 					Ok(())
// 				},
// 			}
// 		}
// 	}
// }
// //use tighly coupling pallets
// impl <T: Config> Pallet<T> {
// 	// Public immutables can be declared in the pallet implementation block.
// 	// This allows other pallets to query the state of the pallet.
// 	pub fn is_something(something: u32) -> bool {
// 		<Something<T>>::get().is_some()
// 	}
// 	pub fn update_storage(new_value: u32) -> DispatchResult {
// 		Something::<T>::put(new_value);
// 		Ok(())
// 	}
// }
// //use loosely coupling pallets
// pub trait DoSomething {
// 	fn increase_value(value: u32) -> u32;
// } 
// impl <T: Config> DoSomething for Pallet<T> {
// 	fn increase_value(value: u32) -> u32 {
		
// 		let old = <Something<T>>::get().unwrap_or(0);
// 		let new = old.checked_add(value).unwrap_or(0);
// 		<Something<T>>::put(new);
// 		new
		
// 	}
// }
	
	

// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Atomic Swap
//!
//! A pallet for atomically sending funds.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! A pallet for atomically sending funds from an origin to a target. A proof
//! is used to allow the target to approve (claim) the swap. If the swap is not
//! claimed within a specified duration of time, the sender may cancel it.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * [`create_swap`](Call::create_swap) - called by a sender to register a new atomic swap
//! * [`claim_swap`](Call::claim_swap) - called by the target to approve a swap
//! * [`cancel_swap`](Call::cancel_swap) - may be called by a sender after a specified duration

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// mod tests;

use codec::{Decode, Encode};
use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::MaxEncodedLen,
	traits::{BalanceStatus, Currency, Get, ReservableCurrency},
	weights::Weight,
	RuntimeDebugNoBound,
};
use scale_info::TypeInfo;
use sp_io::hashing::blake2_256;
use sp_runtime::RuntimeDebug;
use sp_std::{
	marker::PhantomData,
	ops::{Deref, DerefMut},
	prelude::*,
};

/// Pending atomic swap operation.
#[derive(Clone, Eq, PartialEq, RuntimeDebugNoBound, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct PendingSwap<T: Config> {
	/// Source of the swap.
	pub source: T::AccountId,
	/// Action of this swap.
	pub action: T::SwapAction,
	/// End block of the lock.
	pub end_block: T::BlockNumber,
}

/// Hashed proof type.
pub type HashedProof = [u8; 32];

/// Definition of a pending atomic swap action. It contains the following three phrases:
///
/// - **Reserve**: reserve the resources needed for a swap. This is to make sure that **Claim**
/// succeeds with best efforts.
/// - **Claim**: claim any resources reserved in the first phrase.
/// - **Cancel**: cancel any resources reserved in the first phrase.
pub trait SwapAction<AccountId, T: Config> {
	/// Reserve the resources needed for the swap, from the given `source`. The reservation is
	/// allowed to fail. If that is the case, the the full swap creation operation is cancelled.
	fn reserve(&self, source: &AccountId) -> DispatchResult;
	/// Claim the reserved resources, with `source` and `target`. Returns whether the claim
	/// succeeds.
	fn claim(&self, source: &AccountId, target: &AccountId) -> bool;
	/// Weight for executing the operation.
	fn weight(&self) -> Weight;
	/// Cancel the resources reserved in `source`.
	fn cancel(&self, source: &AccountId);
}

/// A swap action that only allows transferring balances.
#[derive(Clone, RuntimeDebug, Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(C))]
#[codec(mel_bound())]
pub struct BalanceSwapAction<AccountId, C: ReservableCurrency<AccountId>> {
	value: <C as Currency<AccountId>>::Balance,
	_marker: PhantomData<C>,
}

impl<AccountId, C> BalanceSwapAction<AccountId, C>
where
	C: ReservableCurrency<AccountId>,
{
	/// Create a new swap action value of balance.
	pub fn new(value: <C as Currency<AccountId>>::Balance) -> Self {
		Self { value, _marker: PhantomData }
	}
}

impl<AccountId, C> Deref for BalanceSwapAction<AccountId, C>
where
	C: ReservableCurrency<AccountId>,
{
	type Target = <C as Currency<AccountId>>::Balance;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<AccountId, C> DerefMut for BalanceSwapAction<AccountId, C>
where
	C: ReservableCurrency<AccountId>,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

impl<T: Config, AccountId, C> SwapAction<AccountId, T> for BalanceSwapAction<AccountId, C>
where
	C: ReservableCurrency<AccountId>,
{
	fn reserve(&self, source: &AccountId) -> DispatchResult {
		C::reserve(source, self.value)
	}

	fn claim(&self, source: &AccountId, target: &AccountId) -> bool {
		C::repatriate_reserved(source, target, self.value, BalanceStatus::Free).is_ok()
	}

	fn weight(&self) -> Weight {
		T::DbWeight::get().reads_writes(1, 1)
	}

	fn cancel(&self, source: &AccountId) {
		C::unreserve(source, self.value);
	}
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Atomic swap's pallet configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Swap action.
		type SwapAction: SwapAction<Self::AccountId, Self> + Parameter + MaxEncodedLen;
		/// Limit of proof size.
		///
		/// Atomic swap is only atomic if once the proof is revealed, both parties can submit the
		/// proofs on-chain. If A is the one that generates the proof, then it requires that either:
		/// - A's blockchain has the same proof length limit as B's blockchain.
		/// - Or A's blockchain has shorter proof length limit as B's blockchain.
		///
		/// If B sees A is on a blockchain with larger proof length limit, then it should kindly
		/// refuse to accept the atomic swap request if A generates the proof, and asks that B
		/// generates the proof instead.
		#[pallet::constant]
		type ProofLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	pub type PendingSwaps<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::AccountId,
		Blake2_128Concat,
		HashedProof,
		PendingSwap<T>,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Swap already exists.
		AlreadyExist,
		/// Swap proof is invalid.
		InvalidProof,
		/// Proof is too large.
		ProofTooLarge,
		/// Source does not match.
		SourceMismatch,
		/// Swap has already been claimed.
		AlreadyClaimed,
		/// Swap does not exist.
		NotExist,
		/// Claim action mismatch.
		ClaimActionMismatch,
		/// Duration has not yet passed for the swap to be cancelled.
		DurationNotPassed,
	}

	/// Event of atomic swap pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Swap created.
		NewSwap { account: T::AccountId, proof: HashedProof, swap: PendingSwap<T> },
		/// Swap claimed. The last parameter indicates whether the execution succeeds.
		SwapClaimed { account: T::AccountId, proof: HashedProof, success: bool },
		/// Swap cancelled.
		SwapCancelled { account: T::AccountId, proof: HashedProof },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new atomic swap, declaring an intention to send funds from origin to target
		/// on the current blockchain. The target can claim the fund using the revealed proof. If
		/// the fund is not claimed after `duration` blocks, then the sender can cancel the swap.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `target`: Receiver of the atomic swap.
		/// - `hashed_proof`: The blake2_256 hash of the secret proof.
		/// - `balance`: Funds to be sent from origin.
		/// - `duration`: Locked duration of the atomic swap. For safety reasons, it is recommended
		///   that the revealer uses a shorter duration than the counterparty, to prevent the
		///   situation where the revealer reveals the proof too late around the end block.
		#[pallet::weight(T::DbWeight::get().reads_writes(1, 1).ref_time().saturating_add(40_000_000))]
		pub fn create_swap(
			origin: OriginFor<T>,
			target: T::AccountId,
			hashed_proof: HashedProof,
			action: T::SwapAction,
			duration: T::BlockNumber,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			ensure!(
				!PendingSwaps::<T>::contains_key(&target, hashed_proof),
				Error::<T>::AlreadyExist
			);

			action.reserve(&source)?;

			let swap = PendingSwap {
				source,
				action,
				end_block: frame_system::Pallet::<T>::block_number() + duration,
			};
			PendingSwaps::<T>::insert(target.clone(), hashed_proof, swap.clone());

			Self::deposit_event(Event::NewSwap { account: target, proof: hashed_proof, swap });

			Ok(())
		}

		/// Claim an atomic swap.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `proof`: Revealed proof of the claim.
		/// - `action`: Action defined in the swap, it must match the entry in blockchain. Otherwise
		///   the operation fails. This is used for weight calculation.
		#[pallet::weight(
			T::DbWeight::get().reads_writes(1, 1)
				.saturating_add(action.weight())
				.ref_time()
				.saturating_add(40_000_000)
				.saturating_add((proof.len() as u64).saturating_mul(100))
		)]
		pub fn claim_swap(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			action: T::SwapAction,
		) -> DispatchResult {
			ensure!(proof.len() <= T::ProofLimit::get() as usize, Error::<T>::ProofTooLarge);

			let target = ensure_signed(origin)?;
			let hashed_proof = blake2_256(&proof);

			let swap =
				PendingSwaps::<T>::get(&target, hashed_proof).ok_or(Error::<T>::InvalidProof)?;
			ensure!(swap.action == action, Error::<T>::ClaimActionMismatch);

			let succeeded = swap.action.claim(&swap.source, &target);

			PendingSwaps::<T>::remove(target.clone(), hashed_proof);

			Self::deposit_event(Event::SwapClaimed {
				account: target,
				proof: hashed_proof,
				success: succeeded,
			});

			Ok(())
		}

		/// Cancel an atomic swap. Only possible after the originally set duration has passed.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `target`: Target of the original atomic swap.
		/// - `hashed_proof`: Hashed proof of the original atomic swap.
		#[pallet::weight(T::DbWeight::get().reads_writes(1, 1).ref_time().saturating_add(40_000_000))]
		pub fn cancel_swap(
			origin: OriginFor<T>,
			target: T::AccountId,
			hashed_proof: HashedProof,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;

			let swap = PendingSwaps::<T>::get(&target, hashed_proof).ok_or(Error::<T>::NotExist)?;
			ensure!(swap.source == source, Error::<T>::SourceMismatch);
			ensure!(
				frame_system::Pallet::<T>::block_number() >= swap.end_block,
				Error::<T>::DurationNotPassed,
			);

			swap.action.cancel(&swap.source);
			PendingSwaps::<T>::remove(&target, hashed_proof);

			Self::deposit_event(Event::SwapCancelled { account: target, proof: hashed_proof });

			Ok(())
		}
	}
}