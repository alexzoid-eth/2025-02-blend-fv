use soroban_sdk::{Env, Address, String};
use soroban_sdk::unwrap::UnwrapOptimized;
use cvlr::{cvlr_assume, cvlr_assert, cvlr_satisfy};
use crate::certora_specs::callable::Call;
use crate::certora_specs::parametric_rules::{
    call_execute_deposit,
    call_execute_donate,
    call_execute_draw,
    call_execute_dequeue_withdrawal,
    call_execute_queue_withdrawal,
    call_execute_withdraw
};
use crate::backstop::{PoolBalance, UserBalance};
use crate::storage;
use crate::constants::Q4W_LOCK_TIME;

pub trait AddressCall: Call {
    fn get_pool_address(&self) -> &Address;
    fn get_user_address(&self) -> &Address;
    fn get_amount(&self) -> i128;
}

impl AddressCall for call_execute_deposit {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.from
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

impl AddressCall for call_execute_donate {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.from
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

impl AddressCall for call_execute_draw {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.to 
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

impl AddressCall for call_execute_dequeue_withdrawal {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.from
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

impl AddressCall for call_execute_queue_withdrawal {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.from
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

impl AddressCall for call_execute_withdraw {
    fn get_pool_address(&self) -> &Address {
        &self.pool_address
    }
    
    fn get_user_address(&self) -> &Address {
        &self.from
    }

    fn get_amount(&self) -> i128 {
        self.amount
    }
}

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

pub fn valid_state_pool(
    e: &Env,
    pool: &Address
) -> bool {
    bound_pool(e, pool)
    && _valid_state_pool_q4w_leq_total_shares(e, pool)
}

pub fn valid_state_pool_user(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    bound_pool(e, pool)
    && bound_user(e, user)
    && _valid_state_user_share_leq_total_pool_shares(e, pool, user)
    && _valid_state_q4w_sum(e, pool, user)
    && _valid_state_q4w_expiration(e, pool, user)
    && _valid_state_nonnegative(e, pool, user)
    && _valid_state_user_pool_contract_always_zero(e, pool, user)
}

// Sanity with valid state

pub fn valid_state_sanity<C: AddressCall>(e: Env, c: C) {
    cvlr_assume!(valid_state_pool_user(&e, &c.get_pool_address(), &c.get_user_address()));
    cvlr_assume!(bound_amount(c.get_amount()));
    c.call(&e);
    cvlr_satisfy!(true);
}

// The expiration time (exp) in any Q4W entry must not exceed timestamp + Q4W_LOCK_TIME

fn _valid_state_q4w_expiration(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
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

pub fn valid_state_q4w_expiration<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_q4w_expiration(&e, &pool, &user));
    c.call(&e);
    cvlr_assert!(_valid_state_q4w_expiration(&e, &pool, &user));
}

// The sum of all amounts in the q4w vector must be less than or equal to the user's shares

fn _valid_state_q4w_sum(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    
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

pub fn valid_state_q4w_sum<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_q4w_sum(&e, &pool, &user));
    c.call(&e);
    cvlr_assert!(_valid_state_q4w_sum(&e, &pool, &user));
}

// q4w (shares queued for withdrawal) should never exceed total shares

fn _valid_state_pool_q4w_leq_total_shares (
    e: &Env,
    pool: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    if pb.q4w as i64 > pb.shares as i64 { return false; }
    true
}

pub fn valid_state_pool_q4w_leq_total_shares<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    cvlr_assume!(valid_state_pool(&e, &pool));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_pool_q4w_leq_total_shares(&e, &pool));
    c.call(&e);
    cvlr_assert!(_valid_state_pool_q4w_leq_total_shares(&e, &pool));
}

// A user's shares cannot exceed the total pool shares

fn _valid_state_user_share_leq_total_pool_shares(
    e: &Env,
    pool: &Address,
    user: &Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(e, pool);
    let ub: UserBalance = storage::get_user_balance(e, pool, user);
    
    if ub.shares > pb.shares { return false; }
    true
}

pub fn valid_state_user_share_leq_total_pool_shares<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_user_share_leq_total_pool_shares(&e, &pool, &user));
    c.call(&e);
    cvlr_assert!(_valid_state_user_share_leq_total_pool_shares(&e, &pool, &user));
}

// All important storage variables are non-negative

fn _valid_state_nonnegative(
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

pub fn valid_state_nonnegative<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_nonnegative(&e, &pool, &user));
    c.call(&e);
    cvlr_assert!(_valid_state_nonnegative(&e, &pool, &user));
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

pub fn valid_state_user_pool_contract_always_zero<C: AddressCall>(e: Env, c: C) {
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
    cvlr_assume!(bound_amount(c.get_amount()));
    cvlr_assume!(_valid_state_user_pool_contract_always_zero(&e, &pool, &user));
    c.call(&e);
    cvlr_assert!(_valid_state_user_pool_contract_always_zero(&e, &pool, &user));
}