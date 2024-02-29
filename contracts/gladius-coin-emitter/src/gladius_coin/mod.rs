
mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;
mod total_supply;


pub use contract::GladiusCoinTokenClient;
pub use contract::GladiusCoinToken;
pub use contract::{internal_mint, internal_burn};
pub use metadata::write_metadata;
pub use admin::{has_administrator, read_administrator, write_administrator};
 