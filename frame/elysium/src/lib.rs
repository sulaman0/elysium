// Re-export pallet items so that they can be accessed from the crate namespace.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{*, ValueQuery};
    use frame_support::traits::{Currency, Randomness};
    use frame_system::pallet_prelude::*;
    // use sp_std::vec::Vec;

    use sp_runtime::{
        traits::{CheckedAdd, Saturating, Zero},
        DispatchResult,
    };

    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
        #[pallet::constant]
        type MaximumSupply: Get<BalanceOf<Self>>;
        #[pallet::constant]
        type CurrentVersion: Get<&'static str>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config>
    {
        UserCoinsIssued(T::AccountId, BalanceOf<T>),
        SignerRemoved(T::AccountId),
        SignerAdded(T::AccountId),

        WalletAdded(T::AccountId),
        WalletRemoved(T::AccountId),
    }

    #[pallet::error]   // <-- Step 4. code block will replace this.
    pub enum Error<T> {
        WalletNotMatched,

        AlreadySigner,

        NotSigner,

        LowBalanceToBurn,

        TooManyCoinsToAllocate,

        // for WalletFoundation
        AlreadyWalletAdded,
        WalletAdded,
        WalletRemoved,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);


    #[pallet::storage]
    #[pallet::getter(fn balance)]
    pub(super) type CoinsAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn main_account)]
    pub(super) type MainAccount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn signers)]
    //pub(super) type Signers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;
    pub(super) type Signers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool>;

    #[pallet::storage]
    #[pallet::getter(fn foundation_wallet)]
    pub(super) type FoundationWallet<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        // pub balance: u128,
        pub balance: BalanceOf<T>,
        pub main_account: Vec<(T::AccountId, BalanceOf<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                balance: Default::default(),
                main_account: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <CoinsAmount<T>>::put(&self.balance);
            for (a, b) in &self.main_account {
                log::info!("New signer with ID: {:?}.", a);
                <MainAccount<T>>::insert(a, b);
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn generate_users_coins(origin: OriginFor<T>, who: T::AccountId, coins: BalanceOf<T>) -> DispatchResult {
            log::info!("generating coins...");
            let mut full_issuance: BalanceOf<T> = Zero::zero();
            let signer = ensure_signed(origin)?;
            log::info!("Signer Details with ID: {:?}.", signer);
            ensure!(Signers::<T>::contains_key(&signer), Error::<T>::NotSigner);
            log::info!("signer key verified.");
            let current_supply = T::Currency::total_issuance();
            full_issuance = full_issuance
                .checked_add(&coins)
                .ok_or(Error::<T>::LowBalanceToBurn)?;
            log::info!("current_supply. {:?}",current_supply);
            log::info!("new coins to generate {:?}",coins);
            ensure!(
				current_supply.saturating_add(full_issuance) <= T::MaximumSupply::get(),
				Error::<T>::TooManyCoinsToAllocate
			);
            match T::Currency::deposit_into_existing(&who, coins) {
                Ok(_) => {
                    Self::deposit_event(Event::UserCoinsIssued(who, coins));
                }
                Err(_) => {
                    log::error!("Wallet doesn't match to deposit coins");
                }
            };
            Ok(())
        }
        //
        #[pallet::call_index(1)]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn add_signer(origin: OriginFor<T>, signer: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(!Signers::<T>::contains_key(&signer),Error::<T>::AlreadySigner);
            Signers::<T>::insert(&signer, true);
            log::info!("insert successful");
            Self::deposit_event(Event::SignerAdded(signer));
            Ok(())
        }
        #[pallet::call_index(2)]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn remove_signer(origin: OriginFor<T>, signer: T::AccountId) -> DispatchResult {
            log::info!("removing signer...");
            ensure_root(origin)?;
            ensure!(Signers::<T>::contains_key(&signer), Error::<T>::NotSigner);
            Signers::<T>::remove(&signer);
            Self::deposit_event(Event::SignerRemoved(signer));
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn add_wallet(origin: OriginFor<T>, signer: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            FoundationWallet::<T>::set(core::prelude::v1::Some(signer));
            log::info!("insert add_wallet successful");
            // Self::deposit_event(Event::WalletAdded(signer));
            Ok(())
        }
    }
}