use soroban_sdk::Env;
use crate::backstop::{self, PoolBackstopData};
use soroban_sdk::Address;
use crate::{pass_arg, make_callable};
use crate::certora_specs::callable::Call;
use crate::parametric_rule;
use cvlr_soroban_derive::rule;

use crate::certora_specs::valid_state::{
    valid_state_q4w_expiration,
    valid_state_q4w_sum,
    valid_state_user_share_leq_total_pool_shares,
    valid_state_pool_q4w_leq_total_shares,
    valid_state_nonnegative,
    valid_state_user_pool_contract_always_zero
};
use crate::certora_specs::state_trans::state_trans_pool_shares_tokens_change_together;

make_callable!(backstop, execute_deposit, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_donate, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_draw, pool_address: Address, amount: i128, to: Address);
make_callable!(backstop, execute_dequeue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_queue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_withdraw, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, load_pool_backstop_data, address: Address);
make_callable!(backstop, require_is_from_pool_factory, address: Address, balance: i128);
make_callable!(backstop, require_pool_above_threshold, no_env, pool_backstop_data: PoolBackstopData);

// valid state
parametric_rule!(valid_state_q4w_expiration, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
parametric_rule!(valid_state_q4w_sum, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
parametric_rule!(valid_state_user_share_leq_total_pool_shares, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
parametric_rule!(valid_state_pool_q4w_leq_total_shares, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
parametric_rule!(valid_state_nonnegative, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
parametric_rule!(valid_state_user_pool_contract_always_zero, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));

// state transition
parametric_rule!(state_trans_pool_shares_tokens_change_together, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw));
