use crate::backstop::PoolBalance;
use crate::certora_specs::mocks::storage_ghost as storage;
use soroban_sdk::{Address, Env};

// q4w (shares queued for withdrawal) should never exceed total shares
pub fn valid_state_pb_q4w_leq_shares (
    e: Env,
    pool: Address,
    _user: Address
) -> bool {
    let pb: PoolBalance = storage::get_pool_balance(&e, &pool);
    
    pb.q4w <= pb.shares
}