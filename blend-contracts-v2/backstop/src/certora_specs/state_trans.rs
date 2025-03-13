use soroban_sdk::Env;
use cvlr::{cvlr_assert, clog};
use crate::backstop::PoolBalance;
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::certora_specs::valid_state::valid_state_pool_user;
use crate::certora_specs::declarations::AddressCall;

// If shares in the pool balance changed, tokens must change too
pub fn state_trans_pool_shares_tokens_change_together<C: AddressCall>(e: Env, c: C) {
    let pool: soroban_sdk::Address = c.get_pool_address();
    let user = c.get_user_address();
    valid_state_pool_user(&e, &pool, &user);
    let before: PoolBalance = storage::get_pool_balance(&e, &pool);
    c.call(&e);
    let after: PoolBalance = storage::get_pool_balance(&e, &pool);
    let shares_changed = before.shares as i64 != after.shares as i64;
    let tokens_changed = before.tokens as i64 != after.tokens as i64;
    cvlr_assert!(!shares_changed || tokens_changed);
}
