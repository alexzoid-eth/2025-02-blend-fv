use soroban_sdk::Env;
use cvlr::cvlr_satisfy;
use crate::certora_specs::callable::Call;

// @todo check with valid state and preserved block

pub fn sanity<C: Call>(e: Env, c: C) {
    c.call(&e);
    cvlr_satisfy!(true);
}