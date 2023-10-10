pub trait IdentifyAccount {
    type AccountId;
}

pub struct RealSigner {}

impl IdentifyAccount for RealSigner {
    type AccountId = u32;
}

pub type RealAccountId = <RealSigner as IdentifyAccount>::AccountId;

pub mod inner {
    pub trait Config {
        type AccountId;
    }
}

pub trait Config: inner::Config<AccountId = RealAccountId> {}

pub struct GenesisConfig<T: Config> {
    pub shelves: Vec<<T as inner::Config>::AccountId>,
}
