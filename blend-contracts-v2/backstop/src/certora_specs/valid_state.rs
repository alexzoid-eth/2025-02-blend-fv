use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks;
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::certora_specs::valid_state_nonnegative::{
    valid_state_nonnegative_pb_shares,
    valid_state_nonnegative_pb_tokens,
    valid_state_nonnegative_pb_q4w,
    valid_state_nonnegative_ub_shares,
    valid_state_nonnegative_ub_q4w_amount,
};
use crate::certora_specs::valid_state_pb::valid_state_pb_q4w_leq_shares;
use crate::certora_specs::valid_state_ub::{
    valid_state_ub_shares_plus_q4w_sum_eq_pb_shares,
    valid_state_ub_q4w_sum_leq_pb_q4w,
    valid_state_ub_q4w_expiration,
    valid_state_ub_q4w_exp_implies_amount,
};
use soroban_sdk::{Address, Env};

// All valid state functions in one place
pub fn valid_state_pool_user(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    // valid_state_nonnegative
    valid_state_nonnegative_pb_shares(e.clone(), pool.clone(), user.clone())
    && valid_state_nonnegative_pb_tokens(e.clone(), pool.clone(), user.clone())
    && valid_state_nonnegative_pb_q4w(e.clone(), pool.clone(), user.clone())
    && valid_state_nonnegative_ub_shares(e.clone(), pool.clone(), user.clone())
    && valid_state_nonnegative_ub_q4w_amount(e.clone(), pool.clone(), user.clone())

    // valid_state_pb
    && valid_state_pb_q4w_leq_shares(e.clone(), pool.clone(), user.clone())

    // valid_state_ub
    && valid_state_ub_shares_plus_q4w_sum_eq_pb_shares(e.clone(), pool.clone(), user.clone())
    && valid_state_ub_q4w_sum_leq_pb_q4w(e.clone(), pool.clone(), user.clone())
    && valid_state_ub_q4w_expiration(e.clone(), pool.clone(), user.clone())
    && valid_state_ub_q4w_exp_implies_amount(e.clone(), pool.clone(), user.clone())

    // valid_state
    && valid_state_user_not_pool(e.clone(), pool.clone(), user.clone())
    && valid_state_pool_from_factory(e.clone(), pool.clone(), user.clone())
}

// User who equals the pool address or the contract address always has a zero balance in that pool
pub fn valid_state_user_not_pool(
    e: Env, 
    pool: Address, 
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if user == pool || user == e.current_contract_address() {
        ub.shares == 0
    } else {
        true
    }
}

// Only deployed by the Pool Factory pool could have shares 
pub fn valid_state_pool_from_factory(
    e: Env, 
    pool: Address, 
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    let pool_factory_client = mocks::pool_factory::PoolFactoryClient::new(&e, &pool);
    
    if pool_factory_client.is_pool(&pool) == false {
        pb.shares == 0
    } else {
        true
    }
}