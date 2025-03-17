use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::{cvlr_assume, cvlr_assert};
use crate::backstop::{PoolBalance, UserBalance, execute_deposit};
use cvlr_soroban_derive::rule;
use crate::init_verification;
use crate::certora_specs::FV_MAX_Q4W_VEC_LEN;
use crate::certora_specs::valid_state::valid_state_pool_user;
use crate::certora_specs::base::clear_upper_bits;

#[cfg(feature = "certora_storage_ghost")] 
use crate::certora_specs::mocks::storage_ghost as storage;
#[cfg(not(feature = "certora_storage_ghost"))]
use crate::storage;

// Verify that execute_deposit returns shares correctly converted from tokens
#[rule]
pub fn high_level_deposit_returns_converted_shares(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
        
    let expected_shares = before_pb.convert_to_shares(amount);
    
    let actual_shares = execute_deposit(e, from, pool_address, amount);
    
    // Verify that the returned shares match what we calculated
    cvlr_assert!(actual_shares == expected_shares);
}