use crate::backstop::{PoolBalance, UserBalance, execute_deposit};
use crate::certora_specs::mocks::storage_ghost as storage;
use soroban_sdk::{Address, Env, unwrap::UnwrapOptimized};
use cvlr::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use cvlr_soroban::nondet_address;

// PoolBalance shares and tokens are non-negative
// @note Violated in `execute_deposit` due overflow issue in `PoolBalance.deposit()` (separate `_violated` config created)
pub fn valid_state_nonnegative_pb_shares(
    e: Env,
    pool: Address,
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);

    pb.shares.is_negative() == false 
}

#[rule]
pub fn valid_state_nonnegative_pb_shares_execute_deposit_violated(
    e: &Env, 
    from: &Address, 
    pool: &Address, 
    amount: i128
) {
    let user = nondet_address();
    cvlr_assume!(valid_state_nonnegative_pb_shares(e.clone(), pool.clone(), user.clone()));
    execute_deposit(e, from, pool, amount);
    cvlr_assert!(valid_state_nonnegative_pb_shares(e.clone(), pool.clone(), user.clone()));
}

// PoolBalance q4w and tokens are non-negative
pub fn valid_state_nonnegative_pb_q4w(
    e: Env,
    pool: Address,
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);

    pb.q4w.is_negative() == false
}

// PoolBalance tokens and tokens are non-negative
pub fn valid_state_nonnegative_pb_tokens(
    e: Env,
    pool: Address,
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);

    pb.tokens.is_negative() == false
}

// UserBalance shares are non-negative
pub fn valid_state_nonnegative_ub_shares(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    ub.shares.is_negative() == false
}

// UserBalance amount in q4w is non-negative
pub fn valid_state_nonnegative_ub_q4w_amount(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if ub.q4w.len() != 0 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();

        entry0.amount.is_negative() == false
    } else {
        true
    }
}
