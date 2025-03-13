use soroban_sdk::Env;
use cvlr::cvlr_satisfy;
use crate::certora_specs::declarations::AddressCall;

pub fn valid_state_sanity<C: AddressCall>(e: Env, c: C) {        
    c.call(&e);
    cvlr_satisfy!(true);
}
