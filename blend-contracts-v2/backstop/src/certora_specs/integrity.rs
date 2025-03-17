use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::{cvlr_assume, cvlr_assert};
use crate::backstop::{PoolBalance, UserBalance, 
    execute_deposit,
    execute_withdraw,
    execute_queue_withdrawal,
    execute_dequeue_withdrawal,
    execute_donate,
    execute_draw
};
use cvlr_soroban_derive::rule;
use crate::init_verification;
use crate::certora_specs::valid_state::valid_state_pool_user;
use crate::certora_specs::summaries::emissions::GHOST_EMISSION_POOL_BALANCE;
use crate::certora_specs::summaries::emissions::GHOST_EMISSION_USER_BALANCE;
use crate::certora_specs::base::clear_upper_bits;
use crate::certora_specs::FV_MAX_Q4W_VEC_LEN;

#[cfg(feature = "certora_storage_ghost")] 
use crate::certora_specs::mocks::storage_ghost as storage;
#[cfg(not(feature = "certora_storage_ghost"))]
use crate::storage;

#[rule]
pub fn integrity_execute_deposit(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    let ret = execute_deposit(e, from, pool_address, amount);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Get emission state after execution
    let emission_pb_after = unsafe { GHOST_EMISSION_POOL_BALANCE.get() };
    let emission_ub_after = unsafe { GHOST_EMISSION_USER_BALANCE.get() };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    
    cvlr_assert!(ub_shares_change == ret);              // User shares should increase by returned amount (minted shares)
    cvlr_assert!(pb_shares_change == ub_shares_change); // Pool shares should increase by same amount
    cvlr_assert!(pb_tokens_change > 0);                 // Pool tokens should increase as tokens are deposited
    cvlr_assert!(pb_q4w_change == 0);                   // Pool Q4W should remain unchanged
    cvlr_assert!(ub_q4w_amount_change == 0);            // User's queued withdrawal amount should remain unchanged
    
    // Check that emission state is updated correctly - compare each field individually
    cvlr_assert!(emission_pb_after.shares == before_pb.shares); 
    cvlr_assert!(emission_pb_after.tokens == before_pb.tokens);
    cvlr_assert!(emission_pb_after.q4w == before_pb.q4w);     
    cvlr_assert!(emission_ub_after.shares == before_ub.shares); 
}

#[rule]
pub fn integrity_execute_withdraw(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Assume emission state is uninitialized before execution
    cvlr_assume!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assume!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });

    let ret = execute_withdraw(e, from, pool_address, amount);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    
    cvlr_assert!(ub_q4w_amount_change == -amount);  // User Q4W amount should decrease by amount
    cvlr_assert!(pb_q4w_change == -amount);         // Pool Q4W should decrease by amount
    cvlr_assert!(pb_shares_change == -amount);      // Pool shares should decrease by amount
    cvlr_assert!(pb_tokens_change == -ret);         // Pool tokens should decrease by returned amount
    cvlr_assert!(ub_shares_change == 0);            // User shares should not change
    
    // Check emission state is still uninitialized after execution
    cvlr_assert!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assert!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });
}

#[rule]
pub fn integrity_execute_queue_withdrawal(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    let ret = execute_queue_withdrawal(e, from, pool_address, amount);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Get emission state after execution
    let emission_pb_after = unsafe { GHOST_EMISSION_POOL_BALANCE.get() };
    let emission_ub_after = unsafe { GHOST_EMISSION_USER_BALANCE.get() };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    
    cvlr_assert!(ub_shares_change == -amount);      // User shares should decrease by amount
    cvlr_assert!(ub_q4w_amount_change == amount);   // User Q4W amount should increase by amount
    cvlr_assert!(pb_q4w_change == amount);          // Pool Q4W should increase by amount
    cvlr_assert!(pb_shares_change == 0);            // Pool shares should not change
    cvlr_assert!(pb_tokens_change == 0);            // Pool tokens should not change
    cvlr_assert!(ret.amount == amount);             // Returned Q4W should have correct amount
    cvlr_assert!(ret.exp > e.ledger().timestamp()); // Expiration should be in the future
    
    // Check that emission state is updated correctly - compare each field individually
    cvlr_assert!(emission_pb_after.shares == before_pb.shares);  
    cvlr_assert!(emission_pb_after.tokens == before_pb.tokens); 
    cvlr_assert!(emission_pb_after.q4w == before_pb.q4w);  
    cvlr_assert!(emission_ub_after.shares == before_ub.shares); 
}

#[rule]
pub fn integrity_execute_dequeue_withdrawal(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    execute_dequeue_withdrawal(e, from, pool_address, amount);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Get emission state after execution
    let emission_pb_after = unsafe { GHOST_EMISSION_POOL_BALANCE.get() };
    let emission_ub_after = unsafe { GHOST_EMISSION_USER_BALANCE.get() };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    
    cvlr_assert!(ub_shares_change == amount);       // User shares should increase by amount
    cvlr_assert!(ub_q4w_amount_change == -amount);  // User Q4W amount should decrease by amount
    cvlr_assert!(pb_q4w_change == -amount);         // Pool Q4W should decrease by amount
    cvlr_assert!(pb_shares_change == 0);            // Pool shares should not change
    cvlr_assert!(pb_tokens_change == 0);            // Pool tokens should not change
    
    // Check that emission state is updated correctly
    cvlr_assert!(emission_pb_after.shares == before_pb.shares);  
    cvlr_assert!(emission_pb_after.tokens == before_pb.tokens); 
    cvlr_assert!(emission_pb_after.q4w == before_pb.q4w);  
    cvlr_assert!(emission_ub_after.shares == before_ub.shares); 
}

#[rule]
pub fn integrity_execute_donate(
    e: &Env, 
    from: &Address, 
    pool_address: &Address, 
    amount: i128,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, from, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Assume emission state is uninitialized before execution
    cvlr_assume!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assume!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });

    execute_donate(e, from, pool_address, amount);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, from);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;

    cvlr_assert!(pb_tokens_change == amount);   // Pool tokens should increase by amount
    cvlr_assert!(pb_shares_change == 0);        // Pool shares should not change
    cvlr_assert!(pb_q4w_change == 0);           // Pool Q4W should not change
    cvlr_assert!(ub_shares_change == 0);        // User shares should not change
    cvlr_assert!(ub_q4w_amount_change == 0);    // User Q4W should not change
    
    // Check emission state is still uninitialized after execution
    cvlr_assert!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assert!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });
}

#[rule]
pub fn integrity_execute_draw(
    e: &Env, 
    pool_address: &Address, 
    amount: i128,
    to: &Address,
    pb: PoolBalance,
    ub: UserBalance
) {
    init_verification!(e, pb, ub, pool_address, to, amount, FV_MAX_Q4W_VEC_LEN);

    let before_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let before_ub: UserBalance = storage::get_user_balance(&e, pool_address, to);
    let before_ub_q4w_amount = if before_ub.q4w.len() == 1 {
        let entry = before_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    // Assume emission state is uninitialized before execution
    cvlr_assume!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assume!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });

    execute_draw(e, pool_address, amount, to);

    let after_pb: PoolBalance = storage::get_pool_balance(&e, pool_address);
    let after_ub: UserBalance = storage::get_user_balance(&e, pool_address, to);
    let after_ub_q4w_amount = if after_ub.q4w.len() == 1 {
        let entry = after_ub.q4w.get(0).unwrap_optimized();
        entry.amount
    } else {
        0
    };

    let ub_shares_change = after_ub.shares - before_ub.shares;
    let pb_shares_change = after_pb.shares - before_pb.shares;
    let pb_tokens_change = after_pb.tokens - before_pb.tokens;
    let pb_q4w_change = after_pb.q4w - before_pb.q4w;
    let ub_q4w_amount_change = after_ub_q4w_amount - before_ub_q4w_amount;
    
    cvlr_assert!(pb_tokens_change == -amount);  // Pool tokens should decrease by amount
    cvlr_assert!(pb_shares_change == 0);        // Pool shares should not change
    cvlr_assert!(pb_q4w_change == 0);           // Pool Q4W should not change
    cvlr_assert!(ub_shares_change == 0);        // User shares should not change
    cvlr_assert!(ub_q4w_amount_change == 0);    // User Q4W should not change
    
    // Check emission state is still uninitialized after execution
    cvlr_assert!(unsafe { GHOST_EMISSION_POOL_BALANCE.is_uninit() });
    cvlr_assert!(unsafe { GHOST_EMISSION_USER_BALANCE.is_uninit() });
}
