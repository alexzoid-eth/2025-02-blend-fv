use crate::backstop::{self, PoolBackstopData, PoolBalance, UserBalance};
use crate::certora_specs::mocks::storage_ghost as storage;
use crate::certora_specs::state_trans::state_trans_pool_shares_tokens_change_together;
use crate::certora_specs::valid_state::valid_state_pool_user;
use crate::certora_specs::valid_state::{
    valid_state_nonnegative_pb_shares_tokens,
    valid_state_nonnegative_ub_shares,
    valid_state_pool_q4w_leq_total_shares,
    valid_state_q4w_expiration,
    valid_state_q4w_sum,
    valid_state_user_pool_contract_always_zero,
    valid_state_user_share_leq_total_pool_shares,
};
use crate::certora_specs::valid_state_sanity::valid_state_sanity;
use crate::certora_specs::{Call, log_state_details};
use crate::{init_verification, invariant_rule, make_callable, parametric_rule, pass_arg};
use cvlr::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use soroban_sdk::{unwrap::UnwrapOptimized, Address, Env};

// Helpers for external functions
make_callable!(backstop, execute_deposit, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_donate, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_draw, pool_address: Address, amount: i128, to: Address);
make_callable!(backstop, execute_dequeue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_queue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_withdraw, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, load_pool_backstop_data, address: Address);
make_callable!(backstop, require_is_from_pool_factory, address: Address, balance: i128);
make_callable!(backstop, require_pool_above_threshold, no_env, pool_backstop_data: PoolBackstopData);

// Sanity parametric rule
parametric_rule!(valid_state_sanity);

// Valid state parametric rules in invariant style
invariant_rule!(valid_state_q4w_expiration);
invariant_rule!(valid_state_q4w_sum);
invariant_rule!(valid_state_user_share_leq_total_pool_shares);
invariant_rule!(valid_state_pool_q4w_leq_total_shares);
invariant_rule!(valid_state_nonnegative_pb_shares_tokens);
invariant_rule!(valid_state_nonnegative_ub_shares);
invariant_rule!(valid_state_user_pool_contract_always_zero);

// State transition parametric rules
parametric_rule!(state_trans_pool_shares_tokens_change_together);