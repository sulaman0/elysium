#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Get},
	};
	use frame_system::pallet_prelude::*;
	use sp_core::H160;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type MaxSponsoredWallets: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn sponsor_manager_address)]
	pub type SponsorManagerAddress<T> = StorageValue<_, H160, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SponsorManagerAddressSet(H160),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidAddress,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::DbWeight::get().writes(1))]
		pub fn set_sponsor_manager_address(origin: OriginFor<T>, address: H160) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(address != H160::zero(), Error::<T>::InvalidAddress);
			SponsorManagerAddress::<T>::put(address);
			Self::deposit_event(Event::SponsorManagerAddressSet(address));
			Ok(())
		}
	}
}
