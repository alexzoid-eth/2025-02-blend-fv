use soroban_sdk::{Env, Address};
use soroban_sdk::unwrap::UnwrapOptimized;
use cvlr::{cvlr_assume, cvlr_assert, clog};
use crate::certora_specs::callable::Call;
use crate::backstop::{PoolBalance, UserBalance};
use crate::storage;
use cvlr_soroban::nondet_address;
use crate::constants::Q4W_LOCK_TIME;

pub fn valid_state_pool_address(
    e: &Env,
    pool_address: &soroban_sdk::Address
) -> bool {
    _valid_state_pool_q4w_leq_total_shares(e, pool_address)
}

pub fn valid_state_pool_address_user_address(
    e: &Env,
    pool_address: &soroban_sdk::Address,
    user_address: &soroban_sdk::Address
) -> bool {
    _valid_state_user_share_leq_total_pool_shares(e, pool_address, user_address)
    && _valid_state_q4w_sum(e, pool_address, user_address)
    && _valid_state_q4w_expiration(e, pool_address, user_address)
    && _valid_state_nonnegative(e, pool_address, user_address)
    && _valid_state_user_pool_contract_always_zero(e, pool_address, user_address)
}

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME
fn _valid_state_q4w_expiration(
    e: &Env,
    pool_address: &Address,
    user_address: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool_address, user_address);
    let max_timestamp = e.ledger().timestamp() + Q4W_LOCK_TIME;

    // Support zero to two elements in the list for simplicity
    cvlr_assume!(ub.q4w.len() <= 2);
    
    if ub.q4w.len() == 1 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        if q4w_entry0.exp > max_timestamp {
            return false;
        }
    } else if ub.q4w.len() == 2 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        let q4w_entry1 = ub.q4w.get(1).unwrap_optimized();
        if q4w_entry0.exp > max_timestamp || q4w_entry1.exp > max_timestamp {
            return false;
        }
    } 
    
    true
}

pub fn valid_state_q4w_expiration<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let user_address = nondet_address();
    cvlr_assume!(valid_state_pool_address_user_address(&e, &pool_address, &user_address));
    cvlr_assume!(_valid_state_q4w_expiration(&e, &pool_address, &user_address));
    c.call(&e);
    cvlr_assert!(_valid_state_q4w_expiration(&e, &pool_address, &user_address));
}

// The sum of all amounts in the q4w vector must be less than or equal to the user's shares
fn _valid_state_q4w_sum(
    e: &Env,
    pool_address: &Address,
    user_address: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool_address, user_address);
    
    // Support zero or one element in the list
    cvlr_assume!(ub.q4w.len() <= 2);

    let mut q4w_sum: i128 = 0;
    if ub.q4w.len() == 1 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount;
    } else if ub.q4w.len() == 2 {
        q4w_sum = ub.q4w.get(0).unwrap_optimized().amount + ub.q4w.get(1).unwrap_optimized().amount;
    }
    
    q4w_sum <= ub.shares
}

pub fn valid_state_q4w_sum<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let user_address = nondet_address();
    cvlr_assume!(valid_state_pool_address_user_address(&e, &pool_address, &user_address));
    cvlr_assume!(_valid_state_q4w_sum(&e, &pool_address, &user_address));
    c.call(&e);
    cvlr_assert!(_valid_state_q4w_sum(&e, &pool_address, &user_address));
}

// A user's shares cannot exceed the total pool shares
fn _valid_state_user_share_leq_total_pool_shares(
    e: &Env,
    pool_address: &soroban_sdk::Address,
    user_address: &soroban_sdk::Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool_address);
    let ub: UserBalance = storage::get_user_balance(e, pool_address, user_address);
    
    if ub.shares > pb.shares { return false; }
    true
}

pub fn valid_state_user_share_leq_total_pool_shares<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let user_address = nondet_address();
    cvlr_assume!(valid_state_pool_address_user_address(&e, &pool_address, &user_address));
    cvlr_assume!(_valid_state_user_share_leq_total_pool_shares(&e, &pool_address, &user_address));
    c.call(&e);
    cvlr_assert!(_valid_state_user_share_leq_total_pool_shares(&e, &pool_address, &user_address));
}

// q4w (shares queued for withdrawal) should never exceed total shares
fn _valid_state_pool_q4w_leq_total_shares (
    e: &Env,
    pool_address: &soroban_sdk::Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool_address);
    if pb.q4w as i64 > pb.shares as i64 { return false; }
    true
}

pub fn valid_state_pool_q4w_leq_total_shares<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    cvlr_assume!(valid_state_pool_address(&e, &pool_address));
    cvlr_assume!(_valid_state_pool_q4w_leq_total_shares(&e, &pool_address));
    c.call(&e);
    cvlr_assert!(_valid_state_pool_q4w_leq_total_shares(&e, &pool_address));
}

// All important storage variables are non-negative
fn _valid_state_nonnegative(
    e: &Env,
    pool_address: &soroban_sdk::Address,
    user_address: &soroban_sdk::Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool_address);
    if pb.shares < 0 || pb.tokens < 0 || pb.q4w < 0 { return false; }

    let ub: UserBalance = storage::get_user_balance(e, pool_address, user_address);
    if ub.shares < 0 { return false; }
    for q in ub.q4w.iter() {
        if q.amount < 0 { return false; }
    }
    true
}

pub fn valid_state_nonnegative<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let user_address = nondet_address();
    cvlr_assume!(valid_state_pool_address_user_address(&e, &pool_address, &user_address));
    cvlr_assume!(_valid_state_nonnegative(&e, &pool_address, &user_address));
    c.call(&e);
    cvlr_assert!(_valid_state_nonnegative(&e, &pool_address, &user_address));
}

// User who equals the pool address or the contract address always has a zero balance in that pool
fn _valid_state_user_pool_contract_always_zero(e: &Env, user: &Address, pool: &Address) -> bool {
    let user_bal: UserBalance = storage::get_user_balance(e, pool, user);

    // If user == pool or user == contract, balance must be zero
    if user == pool || user == &e.current_contract_address() {
        user_bal.shares == 0 && user_bal.q4w.iter().all(|q| q.amount == 0)
    } else {
        true
    }
}

pub fn valid_state_user_pool_contract_always_zero<C: Call>(e: Env, c: C) {
    let pool_address = nondet_address();
    let user_address = nondet_address();
    cvlr_assume!(valid_state_pool_address_user_address(&e, &pool_address, &user_address));
    cvlr_assume!(_valid_state_user_pool_contract_always_zero(&e, &pool_address, &user_address));
    c.call(&e);
    cvlr_assert!(_valid_state_user_pool_contract_always_zero(&e, &pool_address, &user_address));
}