use soroban_sdk::Env;
use cvlr::cvlr_satisfy;
use crate::certora_specs::declarations::{AddressCall, log_state_details};

pub fn valid_state_sanity<C: AddressCall>(e: Env, c: C) {
                
    // Extract pool and user addresses from function arguments
    let pool = c.get_pool_address();
    let user = c.get_user_address();
    
    // Execute rule function
    log_state_details(&e, pool, user);
    c.call(&e);
    log_state_details(&e, pool, user);

    cvlr_satisfy!(true);
}
