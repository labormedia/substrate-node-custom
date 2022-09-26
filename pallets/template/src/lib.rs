#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, debug};
	use frame_system::{pallet_prelude::*};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn check)]
	pub type Clubs<T: Config> = StorageDoubleMap<_,Twox64Concat, u32, Twox64Concat, T::AccountId, T::AccountId, OptionQuery>;
	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		MemberStored(u32, T::AccountId, u32),
		MemberRemoved(u32, T::AccountId),
		MembersEnumerated(u32)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A dispatchable that emits an event with Members Storage enumeration. This is just for testing purposes and it is not considered efficient to implement on every node.
		///  This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn enumerate_members(origin: OriginFor<T>, club: u32) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			// ensure_root(origin)?;
			let _who = ensure_signed(origin)?;

			// Iterate over members for a club.
			for y in Clubs::<T>::iter_key_prefix(club) {
				debug(&y);
			};
			
			Ok(().into())
		}
		/// A dispatchable that takes An accountId as a parameter, removes the value from
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_member(origin: OriginFor<T>, club: u32, candidate: T::AccountId) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			// ensure_root(origin)?;
			let _who = ensure_signed(origin)?;

			// Update storage.
			<Clubs<T>>::remove(club, candidate.clone());

			// Emit an event.
			Self::deposit_event(Event::MemberRemoved(club, candidate));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
		/// A dispatchable that takes An accountId as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_member(origin: OriginFor<T>, club:u32, candidate: T::AccountId) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			// ensure_root(origin)?;
			let who = ensure_signed(origin)?;

			// Update storage.
			<Clubs<T>>::insert(club, candidate.clone(), who);

			// Emit an event.
			Self::deposit_event(Event::MemberStored(club, candidate, 1));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>, club: u32, candidate: T::AccountId) -> DispatchResult {
			// ensure_root(origin)?;
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Clubs<T>>::get(club, candidate) {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(_signer) => {
					Ok(())
				},
			}
		}
	}
}
