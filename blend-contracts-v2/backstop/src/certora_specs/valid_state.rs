use soroban_sdk::{Env, Address};
use cvlr::{cvlr_assume, cvlr_assert, cvlr_satisfy, clog};
use crate::certora_specs::callable::Call;
use crate::backstop::PoolBalance;
use crate::storage;
use cvlr_soroban::nondet_address;

// @todo check non-negative, move invariants separately
pub fn pool_shares_backed(e: &Env, pool_address: &Address) -> bool {
    let pool_balance: PoolBalance = storage::get_pool_balance(e, &pool_address);
    // Check the invariant: if shares != 0, then tokens != 0
    pool_balance.shares as i64 == 0 || pool_balance.tokens as i64 != 0
}
pub fn inv_pool_shares_backed<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    cvlr_assume!(pool_shares_backed(&e, &pool_address));
    c.call(&e);
    cvlr_assert!(pool_shares_backed(&e, &pool_address));
}