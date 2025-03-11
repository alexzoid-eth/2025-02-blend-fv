use soroban_sdk::{Env, Address, String};
use soroban_sdk::unwrap::UnwrapOptimized;
use crate::backstop::{PoolBalance, UserBalance};
use crate::storage;
use crate::constants::Q4W_LOCK_TIME;

pub fn bound_pool(
    e: &Env,
    pool: &Address
) -> bool {
    let constant_str1 = String::from_str(e, "1");
    let constant_address = Address::from_string(&constant_str1);
    
    pool == &constant_address
}

pub fn bound_user(
    e: &Env,
    user: &Address
) -> bool {
    let constant_str2 = String::from_str(e, "2");
    let constant_str3 = String::from_str(e, "3");
    let constant_address1 = Address::from_string(&constant_str2);
    let constant_address2 = Address::from_string(&constant_str3);
    
    user == &constant_address1 || user == &constant_address2
}

pub fn bound_amount(
    amount: i128
) -> bool {
    amount < i64::MAX as i128
}

pub fn bound_user_q4w_len(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);

    ub.q4w.len() <= 2
}

pub fn valid_state_pool_user(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    bound_pool(e, pool)
    && bound_user(e, user)
    && bound_user_q4w_len(e, pool, user)
    && valid_state_pool_q4w_leq_total_shares(e, pool, user)
    && valid_state_user_share_leq_total_pool_shares(e, pool, user)
    && valid_state_q4w_sum(e, pool, user)
    && valid_state_q4w_expiration(e, pool, user)
    && valid_state_nonnegative(e, pool, user)
    && valid_state_user_pool_contract_always_zero(e, pool, user)
}

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME
pub fn valid_state_q4w_expiration(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    let max_timestamp = e.ledger().timestamp() + Q4W_LOCK_TIME;

    // Support zero to two elements in the list for simplicity    
    if ub.q4w.len() > 0 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        if q4w_entry0.exp > max_timestamp {
            return false;
        }

        if ub.q4w.len() == 2 {
            let q4w_entry1 = ub.q4w.get(1).unwrap_optimized();
            if q4w_entry1.exp > max_timestamp {
                return false;
            }
        } else {
            return false;
        }
    }  
    
    true
}

// The sum of all amounts in the q4w vector must be less than or equal to the user's shares
pub fn valid_state_q4w_sum(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    
    // Support zero to two elements in the list for simplicity    
    let mut q4w_sum: i128 = 0;
    if ub.q4w.len() == 1 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount;
    } else if ub.q4w.len() == 2 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount + ub.q4w.get(1).unwrap_optimized().amount;
    } else {
        return false;
    }
    
    q4w_sum <= ub.shares
}

// q4w (shares queued for withdrawal) should never exceed total shares
pub fn valid_state_pool_q4w_leq_total_shares (
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    if pb.q4w as i64 > pb.shares as i64 { return false; }
    true
}

// A user's shares cannot exceed the total pool shares
pub fn valid_state_user_share_leq_total_pool_shares(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    
    if ub.shares > pb.shares { return false; }
    true
}

// All important storage variables are non-negative
pub fn valid_state_nonnegative(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    if pb.shares < 0 || pb.tokens < 0 || pb.q4w < 0 { return false; }

    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    if ub.shares < 0 { return false; }
    for q in ub.q4w.iter() {
        if q.amount < 0 { return false; }
    }
    true
}

// User who equals the pool address or the contract address always has a zero balance in that pool
pub fn valid_state_user_pool_contract_always_zero(e: &Env, user: &Address, pool: &Address) -> bool {
    let user_bal: UserBalance = storage::get_user_balance(e, pool, user);

    // If user == pool or user == contract, balance must be zero
    if user == pool || user == &e.current_contract_address() {
        user_bal.shares == 0 && user_bal.q4w.iter().all(|q| q.amount == 0)
    } else {
        true
    }
}
