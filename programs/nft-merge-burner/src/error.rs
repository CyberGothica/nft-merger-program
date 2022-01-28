use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MergeError {
    #[error("Not enough nfts")]
    NotEnoughNftsError,
    #[error("NFT from external collection was found")]
    NotCyberGothicaNft,
    #[error("Mint address mismatch")]
    MintMismatch,
    #[error("Wrong account owner")]
    WrongOwner,
    #[error("Too many accounts were sent")]
    SuspiciousAccounts
}

impl PrintProgramError for MergeError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MergeError> for ProgramError {
    fn from(e: MergeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MergeError {
    fn type_of() -> &'static str {
        "Merge Error"
    }
}