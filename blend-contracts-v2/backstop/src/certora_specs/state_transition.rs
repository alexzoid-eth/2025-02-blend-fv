use soroban_sdk::Env;
use cvlr::{cvlr_assert, clog};
use crate::certora_specs::callable::Call;
use crate::backstop::PoolBalance;
use crate::storage;
use cvlr_soroban::nondet_address;

// If shares in the pool balance changed, tokens must change too
pub fn shares_tokens_sync<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let before: PoolBalance = storage::get_pool_balance(&e, &pool_address);
    c.call(&e);
    let after: PoolBalance = storage::get_pool_balance(&e, &pool_address);
    let shares_changed = before.shares as i64 != after.shares as i64;
    let tokens_changed = before.tokens as i64 != after.tokens as i64;
    cvlr_assert!(!shares_changed || tokens_changed);
}
