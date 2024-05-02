use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GladiusNFTError {
    NotOwner = 300,
    NotNFT = 301,
    NotAuthorized = 302,
    OutOfBounds = 303,
}


