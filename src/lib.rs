pub trait IdentifyAccount {
    type A;
}
pub struct RealSigner {}

impl IdentifyAccount for RealSigner {
    type A = u32;
}

pub type RealAccountId = <RealSigner as IdentifyAccount>::A;

pub trait BaseConfig {
    type B;
}

pub trait Config: BaseConfig<B = RealAccountId> {}

pub struct GenesisConfig<T: Config> {
    pub shelves: Vec<<T as BaseConfig>::B>,
}
