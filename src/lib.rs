// CRATE A


pub struct AccountId32;


pub trait IdentifyAccount {
	type AccountId;
	fn into_account(&self) -> Self::AccountId;
}

pub struct EcdsaSigner {}

impl IdentifyAccount for EcdsaSigner {
	type AccountId = AccountId32;
	fn into_account(&self) -> Self::AccountId {
		unimplemented!()
	}
}

pub trait Verify {
	type Signer: IdentifyAccount;
	fn verify(&self) -> Self::Signer {
		unimplemented!()
	}
}


pub struct EcdsaSignature {}

impl Verify for EcdsaSignature {
	type Signer = EcdsaSigner;
}

pub type Signature = EcdsaSignature;


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
