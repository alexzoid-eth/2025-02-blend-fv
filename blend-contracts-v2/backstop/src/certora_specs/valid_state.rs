use soroban_sdk::{Env, Address, String};
use soroban_sdk::unwrap::UnwrapOptimized;
use crate::backstop::{PoolBalance, UserBalance};
use crate::constants::Q4W_LOCK_TIME;
use crate::certora_specs::mocks::storage_ghost as storage;

// Bound inputs (pool: 1; users: 2; numbers: -i8^i32; vectors length: 1)

pub const ONE_YEAR: u64 = 365 * 24 * 60 * 60;

pub fn get_static_pool_address(e: &Env) -> Address {
    let constant_str = String::from_str(e, "pool");
    Address::from_string(&constant_str)
}

pub fn get_static_user_address(e: &Env, id: u32) -> Address {
    let seed = match id { 1 => "user1", 2 => "user2", _ => "", };
    let constant_str = String::from_str(e, seed);
    Address::from_string(&constant_str)
}

pub fn bound_pool_user(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {

    let pool_addr: Address = get_static_pool_address(e);
    let user1_addr: Address = get_static_user_address(e, 1);
    let user2_addr: Address = get_static_user_address(e, 2);

    pool == &pool_addr 
        && (user == &user1_addr || user == &user2_addr)
}

pub fn bound_amount(
    amount: i128
) -> bool {
    amount > -i8::MAX as i128 && amount < i32::MAX as i128
}

pub fn bound_pb(
    pb: &PoolBalance,
) -> bool {
    pb.shares > -i8::MAX as i128 && pb.shares < i32::MAX as i128 
        && pb.tokens > -i8::MAX as i128 && pb.tokens < i32::MAX as i128 
        && pb.q4w > -i8::MAX as i128 && pb.q4w < i32::MAX as i128
}

pub fn bound_ub(
    e: &Env,
    ub: &UserBalance
) -> bool {
    let mut q4w0_bounded = true;
    if ub.q4w.len() == 1 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        q4w0_bounded = q4w_entry0.amount > -i8::MAX as i128
            && q4w_entry0.amount < i32::MAX as i128
            && q4w_entry0.exp > e.ledger().timestamp() - ONE_YEAR
            && q4w_entry0.exp < e.ledger().timestamp() + ONE_YEAR;
    }

    ub.shares > -i8::MAX as i128 && ub.shares < i32::MAX as i128 
        && ub.q4w.len() <= 1
        && q4w0_bounded
}

// All valid state functions in one place
pub fn valid_state_pool_user(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    valid_state_pool_q4w_leq_total_shares(e, pool, user)
        && valid_state_user_share_leq_total_pool_shares(e, pool, user)
        && valid_state_q4w_sum(e, pool, user)
        && valid_state_q4w_expiration(e, pool, user)
        && valid_state_nonnegative_pb_shares_tokens(e, pool, user)
        && valid_state_nonnegative_ub_shares(e, pool, user)
        && valid_state_user_pool_contract_always_zero(e, pool, user)
}

// These are bodies of invariants declared in `declarations.rs`

// UserBalance shares are non-negative
pub fn valid_state_nonnegative_ub_shares(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    if ub.shares.is_negative() { return false; }

    true
}

// PoolBalance shares and tokens are non-negative
pub fn valid_state_nonnegative_pb_shares_tokens(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    if pb.shares.is_negative() || pb.tokens.is_negative() || pb.q4w.is_negative() { return false; }

    true
}

/////////////////////////////////////////////////////////////////////////////////////////////

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME
pub fn valid_state_q4w_expiration(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
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
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    
    // Support zero to two elements in the list for simplicity    
    let mut q4w_sum: i128 = 0;
    if ub.q4w.len() == 1 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount;
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
    if pb.q4w > pb.shares { return false; }
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

// User who equals the pool address or the contract address always has a zero balance in that pool
pub fn valid_state_user_pool_contract_always_zero(e: &Env, user: &Address, pool: &Address) -> bool {
    let user_bal: UserBalance = storage::get_user_balance(e, pool, user);

    if user == pool || user == &e.current_contract_address() {
        user_bal.shares == 0
    } else {
        true
    }
}