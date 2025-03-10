use soroban_sdk::{Env, Address};
use cvlr::{cvlr_assume, cvlr_satisfy};
use crate::certora_specs::valid_state::{
    valid_state_pool_address,
    valid_state_pool_address_user_address
};
use crate::backstop::{
    execute_deposit,
    execute_donate,
    execute_draw,
    execute_dequeue_withdrawal,
    execute_queue_withdrawal,
    load_pool_backstop_data,
    require_is_from_pool_factory,
    require_pool_above_threshold,
    PoolBackstopData
};

pub fn valid_state_sanity_execute_deposit(e: Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(valid_state_pool_address_user_address(&e, &from, &pool_address));
    execute_deposit(&e, &from, &pool_address, amount);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_execute_donate(e: Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(valid_state_pool_address_user_address(&e, &from, &pool_address));
    execute_donate(&e, &from, &pool_address, amount);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_execute_draw(e: Env, pool_address: Address, amount: i128, to: Address) {
    cvlr_assume!(valid_state_pool_address_user_address(&e, &to, &pool_address));
    execute_draw(&e, &pool_address, amount, &to);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_execute_dequeue_withdrawal(e: Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(valid_state_pool_address_user_address(&e, &from, &pool_address));
    execute_dequeue_withdrawal(&e, &from, &pool_address, amount);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_execute_queue_withdrawal(e: Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(valid_state_pool_address_user_address(&e, &from, &pool_address));
    execute_queue_withdrawal(&e, &from, &pool_address, amount);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_load_pool_backstop_data(e: Env, address: Address) {
    cvlr_assume!(valid_state_pool_address(&e, &address));
    load_pool_backstop_data(&e, &address);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_require_is_from_pool_factory(e: Env, address: Address, balance: i128) {
    cvlr_assume!(valid_state_pool_address(&e, &address));
    require_is_from_pool_factory(&e, &address, balance);
    cvlr_satisfy!(true);
}

pub fn valid_state_sanity_require_pool_above_threshold(pool_backstop_data: PoolBackstopData) {
    require_pool_above_threshold(&pool_backstop_data);
    cvlr_satisfy!(true);
}