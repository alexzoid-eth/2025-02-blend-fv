use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::cvlr_assert;
use crate::backstop::{PoolBalance, UserBalance};

#[cfg(feature = "certora_storage_ghost")] 
use crate::certora_specs::mocks::storage_ghost as storage;
#[cfg(not(feature = "certora_storage_ghost"))]
use crate::storage;

