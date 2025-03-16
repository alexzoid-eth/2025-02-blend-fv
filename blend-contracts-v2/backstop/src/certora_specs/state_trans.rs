use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::cvlr_assert;
use crate::backstop::{PoolBalance, UserBalance};
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

    // @note there is a weird issue without casting
    cvlr_assert!(before.shares as i64 == after.shares as i64 
        || before.tokens as i64 != after.tokens as i64
    );
}

// If shares increase, tokens must increase and if shares decrease, tokens must decrease
pub fn state_trans_pb_shares_tokens_directional_change(
    e: &Env, 
    pool: &Address, 
    _user: &Address, 
    _amount: i128,
    call_fn: impl FnOnce()
) {
    let before: PoolBalance = storage::get_pool_balance(&e, pool);
    call_fn();
    let after: PoolBalance = storage::get_pool_balance(&e, pool);

    // If shares increase, tokens must increase
    if after.shares > before.shares {
        cvlr_assert!(after.tokens > before.tokens);
    }
    // If shares decrease, tokens must decrease
    else if after.shares < before.shares {
        cvlr_assert!(after.tokens < before.tokens);
    }
}

// Verify the relationship between q4w changes and user/pool balances
pub fn state_trans_q4w_balance_consistency(
    e: &Env, 
    pool: &Address, 
    user: &Address, 
    _amount: i128,
    call_fn: impl FnOnce()
) {
    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool, user);
    
    // Track initial amounts
    let before_ub_q4w_amount: i128 = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };
    
    call_fn();
    
    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool, user);
    
    // Get amount after operation
    let after_ub_q4w_amount: i128 = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };
    
    // Calculate changes
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    
    // Case 1: Queue for withdrawal
    if pb_q4w_change > 0 {
        // User shares decrease, user q4w amount increases
        cvlr_assert!(ub_shares_change == -pb_q4w_change);
        cvlr_assert!(ub_q4w_amount_change == pb_q4w_change);
    }
    
    // Case 2: Withdraw (both pool q4w and shares decrease)
    if pb_q4w_change < 0 && pb_shares_change < 0 {
        // Both pb.q4w and pb.shares decrease by same amount
        cvlr_assert!(pb_q4w_change == pb_shares_change);
        // User q4w amount must decrease
        cvlr_assert!(ub_q4w_amount_change == pb_q4w_change);
    }
    
    // Case 3: Dequeue (q4w decreases, user shares increase)
    if pb_q4w_change < 0 && ub_shares_change > 0 {
        // User shares increase by same amount as q4w decrease
        cvlr_assert!(ub_shares_change == -pb_q4w_change);
        // User q4w amount must decrease
        cvlr_assert!(ub_q4w_amount_change == pb_q4w_change);
    }
}