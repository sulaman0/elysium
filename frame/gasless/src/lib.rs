#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::BuildGenesisConfig};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn is_gasless)]
    pub type IsGasless<T> = StorageValue<_, bool, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn set_is_gasless(origin: OriginFor<T>, is_gasless: bool) -> DispatchResult {
            ensure_root(origin)?;
            IsGasless::<T>::put(is_gasless);
            Ok(())
        }
    }

    // Add the helper function here
    impl<T: Config> Pallet<T> {
        pub fn set_gasless_flag(is_gasless: bool) {
            IsGasless::<T>::put(is_gasless);
        }
    }

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub is_gasless: bool,
        #[serde(skip)]
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            IsGasless::<T>::put(self.is_gasless);
        }
    }
}