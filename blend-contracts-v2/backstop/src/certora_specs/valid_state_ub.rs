use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::constants::Q4W_LOCK_TIME;
use soroban_sdk::{Address, Env, unwrap::UnwrapOptimized};

// A user's shares plus all q4w amounts equal to the total pool shares
pub fn valid_state_ub_shares_plus_q4w_sum_eq_pb_shares(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);
    
    let mut q4w_sum = 0;
    if ub.q4w.len() == 1 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();

        q4w_sum = entry0.amount;
    } 

    ub.shares + q4w_sum == pb.shares
}

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME
pub fn valid_state_ub_q4w_expiration(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if ub.q4w.len() == 1 {
        let max_timestamp = e.ledger().timestamp() + Q4W_LOCK_TIME;
        let entry0 = ub.q4w.get(0).unwrap_optimized();

        entry0.exp <= max_timestamp
    } else {
        true
    } 
}

// The sum of all amounts in the q4w vector must be less than pool's q4w
pub fn valid_state_ub_q4w_sum_leq_pb_q4w(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if ub.q4w.len() == 1 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();
        let q4w_sum: i128 = entry0.amount;

        pb.q4w == q4w_sum
    } else {
        pb.q4w == 0
    }
}

// If a Q4W entry has a non-zero expiration time, it must have a non-zero amount
// @note violated in `queue_withdrawal`
pub fn valid_state_ub_q4w_exp_implies_amount(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if ub.q4w.len() == 1 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();
        // If expiration is set (non-zero), amount must also be set (non-zero)
        if entry0.exp > 0 && entry0.amount == 0 {
            return false;
        }
    } 

    true
}