pub trait IdentifyAccount {
    type AccountId;
}
pub trait Verify {
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

pub mod inner {
    pub trait Config {
        type AccountId;
    }
}

pub trait Config: inner::Config<AccountId = RealAccountId> {
}

pub struct GenesisConfig<T: Config> {
    pub shelves: Vec<<T as inner::Config>::AccountId>,
}
