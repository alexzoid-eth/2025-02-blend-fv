use soroban_sdk::Env;
use crate::backstop::{self, PoolBackstopData};
use soroban_sdk::Address;
use crate::{pass_arg, make_callable};
use crate::certora_specs::callable::Call;
use crate::parametric_rule;
use cvlr_soroban_derive::rule;

use crate::certora_specs::sanity::sanity;
use crate::certora_specs::valid_state::inv_pool_shares_backed;
use crate::certora_specs::state_transition::shares_tokens_sync;

make_callable!(backstop, execute_deposit, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_donate, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_draw, pool_address: Address, amount: i128, to: Address);
make_callable!(backstop, execute_dequeue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_queue_withdrawal, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, execute_withdraw, from: Address, pool_address: Address, amount: i128);
make_callable!(backstop, load_pool_backstop_data, address: Address);
make_callable!(backstop, require_is_from_pool_factory, address: Address, balance: i128);
make_callable!(backstop, require_pool_above_threshold, no_env, pool_backstop_data: PoolBackstopData);

// sanity
parametric_rule!(sanity, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw, load_pool_backstop_data, require_is_from_pool_factory, require_pool_above_threshold));

// valid state
parametric_rule!(inv_pool_shares_backed, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw, load_pool_backstop_data, require_is_from_pool_factory, require_pool_above_threshold));

// state transition
parametric_rule!(shares_tokens_sync, (execute_deposit, execute_donate, execute_draw, execute_dequeue_withdrawal, execute_queue_withdrawal, execute_withdraw, load_pool_backstop_data, require_is_from_pool_factory, require_pool_above_threshold));
