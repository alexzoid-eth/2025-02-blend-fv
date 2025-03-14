use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::constants::Q4W_LOCK_TIME;
use soroban_sdk::{unwrap::UnwrapOptimized, Address, Env};

// All valid state functions in one place
pub fn valid_state_pool_user(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    valid_state_pool_q4w_leq_total_shares(e.clone(), pool.clone(), user.clone())
        && valid_state_user_share_leq_total_pool_shares(e.clone(), pool.clone(), user.clone())
        && valid_state_q4w_sum(e.clone(), pool.clone(), user.clone())
        && valid_state_q4w_expiration(e.clone(), pool.clone(), user.clone())
        && valid_state_nonnegative_pb_shares_tokens(e.clone(), pool.clone(), user.clone())
        && valid_state_nonnegative_ub_shares(e.clone(), pool.clone(), user.clone())
        && valid_state_user_pool_contract_always_zero(e.clone(), pool.clone(), user.clone())
}

// UserBalance shares are non-negative
pub fn valid_state_nonnegative_ub_shares(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);
    if ub.shares.is_negative() { return false; }

    true
}

// PoolBalance shares and tokens are non-negative
pub fn valid_state_nonnegative_pb_shares_tokens(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    if pb.shares.is_negative() || pb.tokens.is_negative() || pb.q4w.is_negative() { return false; }

    true
}

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME
pub fn valid_state_q4w_expiration(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);
    let max_timestamp = e.ledger().timestamp() + Q4W_LOCK_TIME;

    if ub.q4w.len() == 1 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        if q4w_entry0.exp > max_timestamp {
            return false;
        }
    }  
    
    true
}

// The sum of all amounts in the q4w vector must be less than or equal to the user's shares
pub fn valid_state_q4w_sum(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);
    
    // Support zero to two elements in the list for simplicity    
    let mut q4w_sum: i128 = 0;
    if ub.q4w.len() == 1 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount;
    } 
    
    q4w_sum <= ub.shares
}

// q4w (shares queued for withdrawal) should never exceed total shares
pub fn valid_state_pool_q4w_leq_total_shares (
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    if pb.q4w > pb.shares { return false; }
    true
}

// A user's shares cannot exceed the total pool shares
pub fn valid_state_user_share_leq_total_pool_shares(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);
    
    if ub.shares > pb.shares { return false; }
    true
}

// User who equals the pool address or the contract address always has a zero balance in that pool
pub fn valid_state_user_pool_contract_always_zero(
    e: Env, 
    pool: Address, 
    user: Address
) -> bool {
    let user_bal: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if user == pool || user == e.current_contract_address() {
        user_bal.shares == 0
    } else {
        true
    }
}