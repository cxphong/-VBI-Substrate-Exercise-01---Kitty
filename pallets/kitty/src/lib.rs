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
use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitties<T:Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender
	}
	pub type DNA = Vec<u8>;

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
	pub(super) type KittyDNAs<T: Config> = StorageMap<_, Blake2_128Concat, DNA, Kitties<T>, OptionQuery>;

	// key : id
	//value : student
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<DNA>, OptionQuery>;


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
		KittyMinted(Vec<u8>, u32),
		OwnerChanged(Vec<u8>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InnvalidOperation,
		NonExistKitty
	}

	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {

			let who = ensure_signed(origin)?;
			// ensure!(age>20, Error::<T>::TooYoung);
			let gender = Self::gen_gender(dna.clone())?;
			let kitty = Kitties {
				dna: dna.clone(),
				owner: who.clone(),
				gender: gender,
				price: price,
			};

			<KittyDNAs<T>>::insert(dna.clone(), kitty);
			<Kitty<T>>::append(who.clone(), dna.clone());

			let mut total_kitties = <TotalKitties<T>>::get();
			total_kitties +=1;
			TotalKitties::<T>::put(total_kitties);

			Self::deposit_event(Event::KittyMinted(dna, price));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn change_owner(origin: OriginFor<T>, dna: Vec<u8>, new_owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			ensure!(new_owner != who, Error::<T>::InnvalidOperation);
			
			<Kitty<T>>::append(new_owner.clone(), dna.clone());

			<KittyDNAs<T>>::mutate(dna.clone(), | kitty | {
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
								let index = list.iter().position(|r| *r == dna.clone()).unwrap();
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

impl<T> Pallet<T> {
	fn gen_gender(name: Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if name.len() % 2 ==0 {
			res = Gender::Female;
		}
		Ok(res)
	}
}


// Tóm tắt:
//Custom type: Struct ,Enum
// Sử dụng generic type đối với trait
// helper function
// origin
// một số method cơ bản liên quan tới read/write storage
// giải quuêys một số bug có thể có .