soroban_sdk::contractimport!(
    file = "../gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.optimized.wasm"
);
pub type GladiusCoinEmitterClient<'a> = Client<'a>;