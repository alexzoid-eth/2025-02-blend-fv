use soroban_sdk::{Env, Address};
use cvlr::cvlr_satisfy;

pub fn valid_state_sanity(
    e: &Env, 
    pool: &Address, 
    user: &Address, 
    amount: i128,
    call_fn: impl FnOnce()
) {        
    call_fn();
    cvlr_satisfy!(true);
}
