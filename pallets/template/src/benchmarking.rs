//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_member {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::root(), 42, RawOrigin::root())
	verify {
		frame_support::debug(&Clubs::<T>::get(42, Origin::Root));
		assert_eq!(Clubs::<T>::get(42, RawOrigin:root()), Some(1));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
