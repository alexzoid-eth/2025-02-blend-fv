use soroban_sdk::Env;
use cvlr::{cvlr_assume, cvlr_satisfy};
use crate::certora_specs::valid_state::{valid_state_pool_user, bound_amount};
use crate::certora_specs::declarations::{AddressCall, setup_verification_context, log_state_details};
use crate::backstop::{PoolBalance, UserBalance};

pub fn valid_state_sanity<C: AddressCall>(e: Env, c: C) {
                
    // Extract pool and user addresses from function arguments
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    
    // Execute rule function
    log_state_details(&e, pool, user, "Before valid_state_sanity");
    c.call(&e);
    log_state_details(&e, pool, user, "After valid_state_sanity");

    cvlr_satisfy!(true);
}
