//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::vec;

benchmarks! { 
	// tên của benchmark
	transfer_kitty {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		Kitties::<T>::create_kitty(caller_origin, 1000);
		let dna = *KittyOwner::<T>::get(caller.clone()).expect("None").get(0).unwrap();
		let receiver: T::AccountId = account("receiver", 0, 0);

	}: change_owner (RawOrigin::Signed(caller), dna, receiver.clone())

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(TotalKitties::<T>::get(), 1);
		assert_eq!(KittyOwner::<T>::get(receiver).expect("None").len(), 1);
	}
 
	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
