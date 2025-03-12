use soroban_sdk::{Env, Address};
use crate::{pass_arg, make_callable, parametric_rule, invariant_rule};
use crate::certora_specs::callable::Call;
use cvlr_soroban_derive::rule;
use cvlr::{cvlr_assume, cvlr_assert, clog};
use crate::certora_specs::valid_state::{valid_state_pool_user, bound_amount};
use crate::backstop::{self, PoolBackstopData};
use crate::backstop::{PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;

use crate::certora_specs::valid_state::{
    valid_state_q4w_expiration,
    valid_state_q4w_sum,
    valid_state_user_share_leq_total_pool_shares,
    valid_state_pool_q4w_leq_total_shares,
    valid_state_nonnegative_pb,
    valid_state_nonnegative_ub,
    valid_state_user_pool_contract_always_zero,
    valid_state_test_pb_shares_always_zero
};
use crate::certora_specs::valid_state_sanity::valid_state_sanity;
use crate::certora_specs::state_trans::state_trans_pool_shares_tokens_change_together;

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
invariant_rule!(valid_state_q4w_expiration, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_q4w_sum, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_user_share_leq_total_pool_shares, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_pool_q4w_leq_total_shares, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_nonnegative_pb, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_nonnegative_ub, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_user_pool_contract_always_zero, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
invariant_rule!(valid_state_test_pb_shares_always_zero, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));

// Sanity parametric rule with valid state assumed
parametric_rule!(valid_state_sanity, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));

// State transition parametric rules
parametric_rule!(state_trans_pool_shares_tokens_change_together, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
