use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::{cvlr_assume, cvlr_satisfy};
use crate::backstop::{UserBalance, execute_withdraw};
use cvlr_soroban_derive::rule;

#[cfg(feature = "certora_storage_ghost")] 
use crate::certora_specs::mocks::storage_ghost as storage;
#[cfg(not(feature = "certora_storage_ghost"))]
use crate::storage;

