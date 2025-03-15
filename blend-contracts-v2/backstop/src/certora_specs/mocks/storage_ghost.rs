use crate::backstop::{PoolBalance, UserBalance};
use cvlr_soroban::nondet_address;
use soroban_sdk::{Address, Env};

// GhostMap for storage_ghost, inspired by https://github.com/Certora/reflector-subscription-contract/blob/51944577dc4536e9cf9711db6e125fe1e2254054/src/certora_specs/mod.rs
use cvlr::nondet::*;

pub enum GhostMap<V> {
    UnInit,
    Init { v: V }
}

impl<V: Nondet + Clone> GhostMap<V> {
    #[inline(never)]
    pub fn init(&mut self, v: V) {
        *self = Self::Init { v };
    }

    #[inline(never)]
    pub fn set(&mut self, v: V) {
        match self {
            Self::Init { v: my_v } => {
                *my_v = v;
            },
            Self::UnInit => {
                *self = Self::Init { v };
            }
        }
    }

    #[inline(never)]
    pub fn get(&self) -> V {
        match self {
            Self::UnInit => V::nondet(),
            Self::Init { v: my_v } => my_v.clone()
        }
    }
}

pub(crate) static mut GHOST_POOL_BALANCE: GhostMap<PoolBalance> = GhostMap::UnInit;
pub(crate) static mut GHOST_USER_BALANCE: GhostMap<UserBalance> = GhostMap::UnInit;

pub fn get_user_balance(_e: &Env, _pool: &Address, _user: &Address) -> UserBalance {
    unsafe {
        GHOST_USER_BALANCE.get()
    }
}

pub fn set_user_balance(_e: &Env, _pool: &Address, _user: &Address, balance: &UserBalance) {
    unsafe {
        GHOST_USER_BALANCE.set(balance.clone());
    }
}

pub fn get_pool_balance(_e: &Env, _pool: &Address) -> PoolBalance {
    unsafe {
        GHOST_POOL_BALANCE.get()
    }
}

pub fn set_pool_balance(_e: &Env, _pool: &Address, balance: &PoolBalance) {
    unsafe {
        GHOST_POOL_BALANCE.set(balance.clone());
    }
}

pub fn initialize_ghost_maps(
    pool_balance: PoolBalance, 
    user_balance: UserBalance
) {
    unsafe {
        GHOST_POOL_BALANCE.init(pool_balance);
        GHOST_USER_BALANCE.init(user_balance);
    }
}

// Add additional required functions for the file-specific mocks
#[allow(dead_code)]
pub fn get_backstop_token(_e: &Env) -> Address {
    nondet_address()
}

#[allow(dead_code)]
pub fn get_blnd_token(_e: &Env) -> Address {
    nondet_address()
}

#[allow(dead_code)]
pub fn get_usdc_token(_e: &Env) -> Address {
    nondet_address()
}

#[allow(dead_code)]
pub fn get_pool_factory(_e: &Env) -> Address {
    nondet_address()
}

#[allow(dead_code)]
pub fn get_emitter(_e: &Env) -> Address {
    nondet_address()
}