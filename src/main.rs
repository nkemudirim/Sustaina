#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, dispatch,
    traits::{Currency, ReservableCurrency},
};
use frame_system::ensure_signed;
use sp_runtime::traits::Zero;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

decl_storage! {
    trait Store for Module<T: Config> as CarbonCreditModule {
        pub UserCarbonCredits get(fn user_carbon_credits): map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
        pub TotalCarbonCredits get(fn total_carbon_credits): BalanceOf<T>;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, Balance = BalanceOf<T> {
        CarbonCreditsAdded(AccountId, Balance),
        CarbonCreditsUsed(AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        NotEnoughCredits,
        InvalidAction,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn add_credits(origin, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!amount.is_zero(), Error::<T>::InvalidAction);

            UserCarbonCredits::<T>::mutate(&sender, |credits| *credits += amount);
            TotalCarbonCredits::<T>::mutate(|total| *total += amount);

            Self::deposit_event(RawEvent::CarbonCreditsAdded(sender, amount));
            Ok(())
        }

        #[weight = 10_000]
        pub fn use_credits(origin, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(UserCarbonCredits::<T>::get(&sender) >= amount, Error::<T>::NotEnoughCredits);

            UserCarbonCredits::<T>::mutate(&sender, |credits| *credits -= amount);
            TotalCarbonCredits::<T>::mutate(|total| *total -= amount);

            Self::deposit_event(RawEvent::CarbonCreditsUsed(sender, amount));
            Ok(())
        }
    }
}
