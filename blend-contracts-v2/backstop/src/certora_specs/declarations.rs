use soroban_sdk::{Env, Address, String, unwrap::UnwrapOptimized};
use crate::{pass_arg, make_callable, parametric_rule, invariant_rule};
use crate::certora_specs::callable::Call;
use cvlr_soroban_derive::rule;
use cvlr::{cvlr_assume, cvlr_assert, clog};
use crate::certora_specs::valid_state::valid_state_pool_user;
use crate::backstop::{self, PoolBackstopData};
use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::certora_specs::valid_state_sanity::valid_state_sanity;
use crate::certora_specs::state_trans::state_trans_pool_shares_tokens_change_together;
use crate::certora_specs::valid_state::{
    valid_state_q4w_expiration,
    valid_state_q4w_sum,
    valid_state_user_share_leq_total_pool_shares,
    valid_state_pool_q4w_leq_total_shares,
    valid_state_nonnegative_pb_shares_tokens,
    valid_state_nonnegative_ub_shares,
    valid_state_user_pool_contract_always_zero
};

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

pub fn log_state_details(
    e: Env,
    pool: Address,
    user: Address,
    amount: i128
) {
    let pb = storage::get_pool_balance(&e, &pool);
    let ub = storage::get_user_balance(&e, &pool, &user);
        
    clog!(amount as i64);
    clog!(user == pool);
    clog!(user == e.current_contract_address());

    clog!(pb.shares as i64);
    clog!(pb.tokens as i64);
    clog!(pb.q4w as i64);
    
    clog!(ub.shares as i64);
    clog!(ub.q4w.len());
    
    if ub.q4w.len() == 1 {
        let q4w_entry0 = ub.q4w.get(0).unwrap_optimized();
        clog!(q4w_entry0.amount as i64);
        clog!(q4w_entry0.exp);
    }
}

make_callable!(backstop, execute_deposit, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_donate, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_draw, pool_address: Address, amount: i128, to: Address);
make_callable!(backstop, execute_dequeue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_queue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_withdraw, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, load_pool_backstop_data, address: Address);
make_callable!(backstop, require_is_from_pool_factory, address: Address, balance: i128);
make_callable!(backstop, require_pool_above_threshold, no_env, pool_backstop_data: PoolBackstopData);

// Valid state invariants
invariant_rule!(valid_state_q4w_expiration);
invariant_rule!(valid_state_q4w_sum);
invariant_rule!(valid_state_user_share_leq_total_pool_shares);
invariant_rule!(valid_state_pool_q4w_leq_total_shares);
invariant_rule!(valid_state_nonnegative_pb_shares_tokens);
invariant_rule!(valid_state_nonnegative_ub_shares);
invariant_rule!(valid_state_user_pool_contract_always_zero);

// Sanity parametric rule with valid state assumed
parametric_rule!(valid_state_sanity, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));

// State transition parametric rules
parametric_rule!(state_trans_pool_shares_tokens_change_together, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));