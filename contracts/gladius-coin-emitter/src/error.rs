use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GladiusCoinEmitterError {
    InitializeAlreadyInitialized = 101,
    NotInitialized = 102,
    InitializeDecimalMustFitU8 = 103,
    WrapNegativesNotSupported = 104,
    UnWrapNegativesNotSupported = 105,
}


