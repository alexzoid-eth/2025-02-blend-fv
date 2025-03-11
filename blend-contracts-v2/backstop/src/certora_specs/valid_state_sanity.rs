use soroban_sdk::Env;
use cvlr::{cvlr_assume, cvlr_satisfy};
use crate::certora_specs::valid_state::{valid_state_pool_user, bound_amount};
use crate::certora_specs::parametric_rules::AddressCall;

pub fn valid_state_sanity<C: AddressCall>(e: Env, c: C) {
    cvlr_assume!(valid_state_pool_user(&e, &c.get_pool_address(), &c.get_user_address()));
    cvlr_assume!(bound_amount(c.get_amount()));
    c.call(&e);
    cvlr_satisfy!(true);
}
