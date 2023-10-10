pub trait IdentifyAccount {
    type AccountId;
}
pub struct RealSigner {}

impl IdentifyAccount for RealSigner {
    type AccountId = u32;
}

pub type RealAccountId = <RealSigner as IdentifyAccount>::AccountId;

pub trait BaseConfig {
    type AccountId;
}

pub trait Config: BaseConfig<AccountId = RealAccountId> {}

pub struct GenesisConfig<T: Config> {
    pub shelves: Vec<<T as BaseConfig>::AccountId>,
}
