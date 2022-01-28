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
pub enum ErrorCode {
    #[error("Incorrect merge: burn instruction was not commited!")]
    NoBurnInstruction,
    #[error("Account does not have correct owner!")]
    IncorrectOwner,
    #[error("Account is not initialized!")]
    Uninitialized,
    #[error("Mint Mismatch!")]
    MintMismatch,
    #[error("Index greater than length!")]
    IndexGreaterThanLength,
    #[error("Numerical overflow error!")]
    NumericalOverflowError,
    #[error("Can only provide up to 4 creators to candy machine (because candy machine is one)!")]
    TooManyCreators,
    #[error("Uuid must be exactly of 6 length")]
    UuidMustBeExactly6Length,
    #[error("Not enough tokens to pay for this minting")]
    NotEnoughTokens,
    #[error("Not enough SOL to pay for this minting")]
    NotEnoughSOL,
    #[error("Token transfer failed")]
    TokenTransferFailed,
    #[error("Candy machine is empty!")]
    CandyMachineEmpty,
    #[error("Candy machine is not live!")]
    CandyMachineNotLive,
    #[error("Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order")]
    HiddenSettingsConfigsDoNotHaveConfigLines,
    #[error("Cannot change number of lines unless is a hidden config")]
    CannotChangeNumberOfLines,
    #[error("Derived key invalid")]
    DerivedKeyInvalid,
    #[error("Public key mismatch")]
    PublicKeyMismatch,
    #[error("No whitelist token present")]
    NoWhitelistToken,
    #[error("Token burn failed")]
    TokenBurnFailed,
    #[error("Missing gateway app when required")]
    GatewayAppMissing,
    #[error("Missing gateway token when required")]
    GatewayTokenMissing,
    #[error("Invalid gateway token expire time")]
    GatewayTokenExpireTimeInvalid,
    #[error("Missing gateway network expire feature when required")]
    NetworkExpireFeatureMissing,
    #[error("Unable to find an unused config line near your random number index")]
    CannotFindUsableConfigLine,
    #[error("Invalid string")]
    InvalidString,
    #[error("Suspicious transaction detected")]
    SuspiciousTransaction,
    #[error("Cannot Switch to Hidden Settings after items available is greater than 0")]
    CannotSwitchToHiddenSettings,
}

impl PrintProgramError for ErrorCode {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ErrorCode {
    fn type_of() -> &'static str {
        "Error"
    }
}