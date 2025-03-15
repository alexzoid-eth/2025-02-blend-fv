use soroban_sdk::{Env, Address};
use cvlr::cvlr_assert;
use crate::backstop::PoolBalance;
use crate::certora_specs::mocks::storage_ghost as storage;

// If shares in the pool balance changed, tokens must change too
pub fn state_trans_pb_shares_tokens_change_together(
    e: &Env, 
    pool: &Address, 
    _user: &Address, 
    _amount: i128,
    call_fn: impl FnOnce()
) {
    let before: PoolBalance = storage::get_pool_balance(&e, pool);
    call_fn();
    let after: PoolBalance = storage::get_pool_balance(&e, pool);
    let shares_changed = before.shares != after.shares;
    let tokens_changed = before.tokens != after.tokens;
    cvlr_assert!(!shares_changed || tokens_changed);
}