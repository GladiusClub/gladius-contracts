
mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;

pub use contract::GladiusCoinTokenClient;
pub use contract::GladiusCoinToken; 
pub use metadata::write_metadata;
pub use admin::{has_administrator, read_administrator, write_administrator};
pub use balance::{receive_balance, spend_balance};
