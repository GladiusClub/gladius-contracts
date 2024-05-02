//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeEvent {
    pub admin: Address,
    pub name: String,
    pub symbol: String,
}

pub(crate) fn initialize(
    e: &Env,
    admin: Address,
    name: String,
    symbol: String,
) {
    let event: InitializeEvent = InitializeEvent {
        admin: admin,
        name: name,
        symbol: symbol,
    };
    e.events().publish(("GladiusNFT", symbol_short!("init")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferFromEvent { 
    pub spender: Address,
    pub from: Address,
    pub to: Address,
    pub token_id: u32
}

pub(crate) fn transfer_from(
    e: &Env, 
    spender: Address,
    from: Address,
    to: Address,
    token_id: u32) {
    
    let event: TransferFromEvent = TransferFromEvent {
        spender: spender,
        from: from,
        to: to,
        token_id: token_id
    };
    e.events().publish(("GladiusNFT", symbol_short!("transf")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalEvent {
    pub caller: Address,
    pub token_id: u32,
    pub ttl: u32,
}

pub(crate) fn approve(
    e: &Env,
    caller: Address,
    token_id: u32,
    ttl: u32,
) {
    let event: ApprovalEvent = ApprovalEvent {
        caller: caller,
        token_id: token_id,
        ttl: ttl,
    };
    e.events().publish(("GladiusNFT", symbol_short!("approval")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalForAllEvent {
    pub caller: Address,
    pub owner: Address,
    pub operator: Address,
    pub approved: bool,
    pub ttl: u32,
}

pub(crate) fn set_approval_for_all(
    e: &Env,
    caller: Address,
    owner: Address,
    operator: Address,
    approved: bool,
    ttl: u32,
) {
    let event: ApprovalForAllEvent = ApprovalForAllEvent {
        caller: caller,
        owner: owner,
        operator: operator,
        approved: approved,
        ttl: ttl,
    };
    e.events().publish(("GladiusNFT", symbol_short!("appr_all")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAdminEvent {
    pub new_admin: Address,
}

pub(crate) fn set_admin(
    e: &Env,
    new_admin: Address,
) {
    let event: SetAdminEvent = SetAdminEvent {
        new_admin: new_admin,
    };
    e.events().publish(("GladiusNFT", symbol_short!("set_admin")), event);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    pub to: Address,
    pub token_id: u32,
    pub uri: String,
}

pub(crate) fn mint(
    e: &Env,
    to: Address,
    token_id: u32,
    uri: String,
) {
    let event: MintEvent = MintEvent {
        to: to,
        token_id: token_id,
        uri: uri,
    };
    e.events().publish(("GladiusNFT", symbol_short!("mint")), event);
}

