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

pub struct Pallet<T>(std::marker::PhantomData<(T)>);

pub trait Config: frame_system::Config<AccountId = RealAccountId> {
	type RuntimeEvent: From<frame_system::Event<Self>>
		+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

pub struct GenesisConfig<T: Config> {
	pub shelves: Vec<<T as frame_system::Config>::AccountId>,
}
