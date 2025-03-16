use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;
use soroban_sdk::{Address, Env, unwrap::UnwrapOptimized};

// PoolBalance shares and tokens are non-negative
// @note Violated in `execute_deposit` due overflow issue in `PoolBalance.deposit()`
pub fn valid_state_nonnegative_pb_shares(
    e: Env,
    pool: Address,
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);

    pb.shares.is_negative() == false 
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
