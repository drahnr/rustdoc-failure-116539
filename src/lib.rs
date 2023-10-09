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

pub struct Pallet<T>(std::marker::PhantomData<T>);


pub struct BaseEvent<T: ?Sized> {
	_x: std::marker::PhantomData<T>,
}

pub mod inner {
	pub trait Config {
		type RuntimeEvent;
		type AccountId;
	}
}
pub trait IsType<T>: From<T> + Into<T> {}

pub trait Config: inner::Config<AccountId = RealAccountId> {
	type RuntimeEvent: From<BaseEvent<Self>>
		+ IsType<<Self as inner::Config>::RuntimeEvent>;
}

pub struct GenesisConfig<T: Config> {
	pub shelves: Vec<<T as inner::Config>::AccountId>,
}
