// CRATE A


pub struct Acc32;


pub trait IdentifyAccount {
	type AccountId;
	fn into_account(&self) -> Self::AccountId;
}

pub struct SignerY {}

impl IdentifyAccount for SignerY {
	type AccountId = Acc32;
	fn into_account(&self) -> Self::AccountId {
		unimplemented!()
	}
}

pub struct Whatever {}

pub trait Verify {
	type Signer: IdentifyAccount;
	fn verify(&self) -> Self::Signer {
		unimplemented!()
	}
}

impl Verify for Whatever {
	type Signer = SignerY;
}

pub type Signature = Whatever;


// CRATE B

pub type PrimitivesAccountId = 
	<
		<Signature as Verify>::Signer as IdentifyAccount
	>::AccountId;


// CRATE C
pub mod frame_system {
	pub trait Config {
		type AccountId;
	}
}

// CRATE D

/// Removing the associated type specification, `= PrimitivesAccountId`
/// solves the issues in the real codebase.
pub trait Config: frame_system::Config<AccountId = PrimitivesAccountId> {
	type Inner;
}

pub struct Genesis {}

impl frame_system::Config for Genesis {
	type AccountId = PrimitivesAccountId;
}

impl Config for Genesis {
	type Inner = <Self as frame_system::Config>::AccountId;
}
