#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;

trait IdentifyAccount {
    type AccountId;
}
trait Verify {
    type Signer;
}
pub struct RealSigner {}
impl IdentifyAccount for RealSigner {
    type AccountId = u32;
}
pub struct RealSignature {}
impl Verify for RealSignature {
    type Signer = RealSigner;
}

pub type RealAccountId = <<RealSignature as Verify>::Signer as IdentifyAccount>::AccountId;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config<AccountId = RealAccountId> {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// The initial set of shelves.
        pub shelves: Vec<<T as frame_system::Config>::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                shelves: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }

    #[pallet::event]
    pub enum Event<T: Config> {
        X,
    }
}
