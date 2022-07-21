#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::dispatch::fmt;
use frame_support::traits::UnixTime;
use frame_support::traits::Randomness;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitties<T:Config> {
		dna: T::Hash,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_date: u64,
	}

	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Timestamp: UnixTime;
		type MyRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		#[pallet::constant]
		type KittyLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// // The pallet's runtime storage items.
	// // https://docs.substrate.io/v3/runtime/storage
	// #[pallet::storage]
	// #[pallet::getter(fn student_id)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// pub type StudentId<T> = StorageValue<_, Id,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_kitties)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TotalKitties<T> = StorageValue<_, u32,ValueQuery>;

	// key : id
	//value : student
	#[pallet::storage]
	#[pallet::getter(fn kitty_dna)]
	pub(super) type KittyDNAs<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitties<T>, OptionQuery>;

	// key : id
	//value : student
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::Hash, T::KittyLimit>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Nonce<T: Config> = StorageValue<_, u32,ValueQuery>;


	// // key : id
	// //value : student
	// #[pallet::storage]
	// #[pallet::getter(fn student)]
	// pub(super) type Student<T: Config> = StorageMap<_, Blake2_128Concat, Id, Students<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyMinted(T::Hash, u32),
		OwnerChanged(T::Hash, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InnvalidOperation,
		NonExistKitty,
		OverKittyLimit,
		NoneValue
	}

	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint_kitty(origin: OriginFor<T>, price: u32) -> DispatchResult {

			let who = ensure_signed(origin)?;

			// ensure!(number_kitties.len() > T::ProofLimit::get() as usize, Error::<T>::ProofTooLarge);
			
			// ensure!(age>20, Error::<T>::TooYoung);
			let dna = Self::create_unique()?;
			let gender = Self::gen_gender(dna)?;
			let kitty = Kitties {
				dna:  dna,
				owner: who.clone(),
				gender: gender,
				price: price,
				created_date: T::Timestamp::now().as_secs(),
			};

			<KittyDNAs<T>>::insert(dna, kitty);
			if <Kitty<T>>::contains_key(who.clone()) {
				<Kitty<T>>::try_mutate(who.clone(), |dnas| match dnas {
					Some(dnas) => dnas.try_push(dna.clone()).map_err(|_| Error::<T>::OverKittyLimit),
					_ => Err(Error::<T>::NoneValue),
				})?;
			} else {
				let mut _dnas = Vec::new();
				_dnas.push(dna);

				// let bounded_dnas : BoundedVec<T::Hash, T::KittyLimit> = BoundedVec::from_vec(dnas);
				// let bounded_dnas = <BoundedVec<T::Hash, T::KittyLimit>>::truncate_from(_dnas);
				let bounded_dnas : BoundedVec<_, _>= _dnas.try_into().unwrap();
				<Kitty<T>>::insert(who.clone(), bounded_dnas);
			}

			let mut total_kitties = <TotalKitties<T>>::get();
			total_kitties +=1;
			TotalKitties::<T>::put(total_kitties);

			Self::deposit_event(Event::KittyMinted(dna, price));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn change_owner(origin: OriginFor<T>, dna: T::Hash, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			ensure!(new_owner != who, Error::<T>::InnvalidOperation);
			
			// Add to new owner
			if <Kitty<T>>::contains_key(new_owner.clone()) {
				<Kitty<T>>::try_mutate(new_owner.clone(), |dnas| match dnas {
					Some(dnas) => dnas.try_push(dna.clone()).map_err(|_| Error::<T>::OverKittyLimit),
					_ => Err(Error::<T>::NoneValue),
				})?;
			} else {
				let mut _dnas = Vec::new();
				_dnas.push(dna);
				// let bounded_dnas : BoundedVec<T::Hash, T::KittyLimit> = BoundedVec::from_vec(dnas).unwrap();
				// let bounded_dnas = <BoundedVec<T::Hash, T::KittyLimit>>::truncate_from(_dnas);
				let bounded_dnas:BoundedVec<_, _>= _dnas.try_into().unwrap();
				<Kitty<T>>::insert(new_owner.clone(), bounded_dnas);
			}

			// Update DNA
			<KittyDNAs<T>>::mutate(dna, | kitty | {
				match kitty {
					Some(kitty) => kitty.owner = new_owner.clone(),
					None =>  {
						// Err(Error::<T>::InnvalidOperation)
					}
				};
			});

			<Kitty<T>>::mutate(who.clone(), |dna_list| {
				match dna_list {
					Some(list) => {

						let copy_list = list.clone();
						let iter = copy_list.iter();

						for val in iter {
							if val == &dna {
								let index = list.iter().position(|r| *r == dna).unwrap();
								list.remove(index);
							}
						}
					},
					None => {
						// Err(Error::<T>::InnvalidOperation)
					}
				}
			});

			Self::deposit_event(Event::OwnerChanged(dna, new_owner));

			Ok(())
		}

	}
}


// helper function

impl<T: Config> Pallet<T> {
	fn gen_gender(dna: T::Hash) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if dna.as_ref().len() % 2 ==0 {
			res = Gender::Female;
		}
		Ok(res)
	}

	fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce.wrapping_add(1));
		nonce.encode()
	}

	fn create_unique() -> Result<T::Hash,Error<T>> {
		// Random value.
		let nonce = Self::get_and_increment_nonce();
		let (randomValue, _) = T::MyRandomness::random(&nonce);

		Ok(randomValue)
	}

}