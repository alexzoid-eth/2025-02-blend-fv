pub(crate) mod mocks;
pub(crate) mod declarations;

pub(crate) mod valid_state;
pub(crate) mod valid_state_sanity;
pub(crate) mod valid_state_nonnegative;
pub(crate) mod valid_state_ub;
pub(crate) mod valid_state_pb;
pub(crate) mod state_trans;

use soroban_sdk::{Env, Address, unwrap::UnwrapOptimized};
use cvlr::clog;
use crate::certora_specs::mocks::storage_ghost as storage;

// Log an i128 value by splitting it into parts that fit within i64 range

pub fn log_i128(value: i128) {
    let i128_lower_bits = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let i128_upper_bits = (value.abs() >> 63) as i64;
    let i128_is_negative = value.is_negative();
    
    clog!(i128_lower_bits);
    clog!(i128_upper_bits);
    clog!(i128_is_negative);
}

pub fn log_input_amount(value: i128) {
    let input_amount_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let input_amount_upper = (value.abs() >> 63) as i64;
    let input_amount_neg = value.is_negative();
    
    clog!(input_amount_lower);
    clog!(input_amount_upper);
    clog!(input_amount_neg);
}

pub fn log_pb_shares(value: i128) {
    let pb_shares_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let pb_shares_upper = (value.abs() >> 63) as i64;
    let pb_shares_neg = value.is_negative();
    
    clog!(pb_shares_lower);
    clog!(pb_shares_upper);
    clog!(pb_shares_neg);
}

pub fn log_pb_tokens(value: i128) {
    let pb_tokens_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let pb_tokens_upper = (value.abs() >> 63) as i64;
    let pb_tokens_neg = value.is_negative();
    
    clog!(pb_tokens_lower);
    clog!(pb_tokens_upper);
    clog!(pb_tokens_neg);
}

pub fn log_pb_q4w(value: i128) {
    let pb_q4w_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let pb_q4w_upper = (value.abs() >> 63) as i64;
    let pb_q4w_neg = value.is_negative();
    
    clog!(pb_q4w_lower);
    clog!(pb_q4w_upper);
    clog!(pb_q4w_neg);
}

pub fn log_ub_shares(value: i128) {
    let ub_shares_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let ub_shares_upper = (value.abs() >> 63) as i64;
    let ub_shares_neg= value.is_negative();
    
    clog!(ub_shares_lower);
    clog!(ub_shares_upper);
    clog!(ub_shares_neg);
}

pub fn log_pb_q4w_amount(value: i128) {
    let pb_q4w_amount_lower = (value.abs() & 0x7FFF_FFFF_FFFF_FFFF) as i64;
    let pb_q4w_amount_upper = (value.abs() >> 63) as i64;
    let pb_q4w_amount_neg = value.is_negative();
    
    clog!(pb_q4w_amount_lower);
    clog!(pb_q4w_amount_upper);
    clog!(pb_q4w_amount_neg);
}

pub fn log_state_details(
    e: Env,
    pool: Address,
    user: Address,
    amount: i128
) {
    let pb = storage::get_pool_balance(&e, &pool);
    let ub = storage::get_user_balance(&e, &pool, &user);

    log_input_amount(amount);
    clog!(user == pool);
    clog!(user == e.current_contract_address());

    log_pb_shares(pb.shares);
    log_pb_tokens(pb.tokens);
    log_pb_q4w(pb.q4w);
    
    log_ub_shares(ub.shares);
    clog!(ub.q4w.len());
    
    if ub.q4w.len() == 1 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();
        log_pb_q4w_amount(entry0.amount);
        clog!(entry0.exp);
    }
}

pub trait Call {
    fn call(&self, env: &Env);
}

#[macro_export]
macro_rules! pass_arg {
    ($arg:expr, Address) => { &$arg };
    ($arg:expr, PoolBackstopData) => { &$arg };
    ($arg:expr, $ty:ty) => { $arg };
}

#[macro_export]
macro_rules! make_callable {
    ($contract:ident, $call:ident, $($arg:ident : $ty:ty),*) => {
        paste::paste!(
            #[allow(non_camel_case_types)]
            pub struct [< call_ $call >] { $(pub $arg: $ty,)* } 
            impl Call for [< call_ $call >] {
                fn call(&self, env: &Env) {
                    $contract::$call(env, $(pass_arg!(self.$arg, $ty)),*);
                }
            }
        );
    };

    ($contract:ident, $call:ident, no_env, $($arg:ident : $ty:ty),*) => {
        paste::paste!(
            #[allow(non_camel_case_types)]
            pub struct [< call_ $call >] { $(pub $arg: $ty,)* }  
            impl Call for [< call_ $call >] {
                fn call(&self, _env: &Env) {
                    $contract::$call($(pass_arg!(self.$arg, $ty)),*);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! init_verification {
    ($e:expr, $pb:expr, $ub:expr, $pool:expr, $user:expr, $amount:expr) => {

        storage::initialize_ghost_maps($pb.clone(), $ub.clone());

        // Explicitly assume that reading from the ghost maps returns the expected values
        let read_pb = storage::get_pool_balance(&$e, &$pool);
        let read_ub = unsafe { storage::GHOST_USER_BALANCE.get() };
        cvlr_assume!(read_pb.shares == $pb.shares);
        cvlr_assume!(read_pb.tokens == $pb.tokens);
        cvlr_assume!(read_pb.q4w == $pb.q4w);
        cvlr_assume!(read_ub.shares == $ub.shares);
        cvlr_assume!(read_ub.q4w.len() == $ub.q4w.len());
        cvlr_assume!(read_ub.q4w.len() <= 1);
        cvlr_assume!($e.ledger().timestamp() != 0);
        if read_ub.q4w.len() == 1 {
            let r_entry0 = read_ub.q4w.get(0).unwrap_optimized();
            let entry0 = $ub.q4w.get(0).unwrap_optimized();

            cvlr_assume!(r_entry0.amount == entry0.amount);
            cvlr_assume!(r_entry0.exp == entry0.exp);
        }
        
        // Assume valid state
        cvlr_assume!(valid_state_pool_user($e.clone(), $pool.clone(), $user.clone()));
    };
}

#[macro_export]
macro_rules! parametric_rule {
    ($f:ident) => {
        paste::paste! {
            #[rule]
            pub fn [< $f _execute_deposit >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                init_verification!(e, pb, ub, pool_address, from, amount);
                
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_deposit(&e_clone, &from_clone, &pool_clone, amount); };
                
                $f(&e, &pool_address, &from, amount, call_fn);
            }
            
            #[rule]
            pub fn [< $f _execute_donate >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                init_verification!(e, pb, ub, pool_address, from, amount);
                
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_donate(&e_clone, &from_clone, &pool_clone, amount); };
                
                $f(&e, &pool_address, &from, amount, call_fn);
            }
            
            #[rule]
            pub fn [< $f _execute_draw >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                pool_address: Address, 
                amount: i128,
                to: Address
            ) {
                init_verification!(e, pb, ub, pool_address, to, amount);
                
                let e_clone = e.clone();
                let to_clone = to.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_draw(&e_clone, &pool_clone, amount, &to_clone); };
                
                $f(&e, &pool_address, &to, amount, call_fn);
            }
            
            #[rule]
            pub fn [< $f _execute_dequeue_withdrawal >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                init_verification!(e, pb, ub, pool_address, from, amount);
                
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_dequeue_withdrawal(&e_clone, &from_clone, &pool_clone, amount); };
                
                $f(&e, &pool_address, &from, amount, call_fn);
            }
            
            #[rule]
            pub fn [< $f _execute_queue_withdrawal >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                init_verification!(e, pb, ub, pool_address, from, amount);
                
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_queue_withdrawal(&e_clone, &from_clone, &pool_clone, amount); };
                
                $f(&e, &pool_address, &from, amount, call_fn);
            }
            
            #[rule]
            pub fn [< $f _execute_withdraw >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                init_verification!(e, pb, ub, pool_address, from, amount);
                
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                let call_fn = || { backstop::execute_withdraw(&e_clone, &from_clone, &pool_clone, amount); };
                
                $f(&e, &pool_address, &from, amount, call_fn);
            }
        }
    };
}

#[macro_export]
macro_rules! invariant_rule {
    ($inv:ident) => {
        paste::paste! {
            fn [< $inv _helper >](
                e: Env,
                pb: PoolBalance,
                ub: UserBalance,
                pool: Address,
                user: Address,
                amount: i128,
                call_fn: impl FnOnce()
            ) {
                init_verification!(e, pb, ub, pool, user, amount);

                cvlr_assume!($inv(e.clone(), pool.clone(), user.clone()));
                
                log_state_details(e.clone(), pool.clone(), user.clone(), amount);
                call_fn();
                log_state_details(e.clone(), pool.clone(), user.clone(), amount);

                cvlr_assert!($inv(e.clone(), pool.clone(), user.clone()));
            }
            
            #[rule]
            pub fn [< $inv _execute_deposit >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, from, amount, 
                    || { backstop::execute_deposit(&e_clone, &from_clone, &pool_clone, amount); }
                );
            }
            
            #[rule]
            pub fn [< $inv _execute_donate >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, from, amount, 
                    || { backstop::execute_donate(&e_clone, &from_clone, &pool_clone, amount); }
                );
            }
            
            #[rule]
            pub fn [< $inv _execute_draw >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                pool_address: Address, 
                amount: i128,
                to: Address
            ) {
                let e_clone = e.clone();
                let to_clone = to.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, to, amount, 
                    || { backstop::execute_draw(&e_clone, &pool_clone, amount, &to_clone); }
                );
            }
            
            #[rule]
            pub fn [< $inv _execute_dequeue_withdrawal >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, from, amount, 
                    || { backstop::execute_dequeue_withdrawal(&e_clone, &from_clone, &pool_clone, amount); }
                );
            }
            
            #[rule]
            pub fn [< $inv _execute_queue_withdrawal >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, from, amount, 
                    || { backstop::execute_queue_withdrawal(&e_clone, &from_clone, &pool_clone, amount); }
                );
            }
            
            #[rule]
            pub fn [< $inv _execute_withdraw >](
                e: Env, 
                pb: PoolBalance, 
                ub: UserBalance, 
                from: Address, 
                pool_address: Address, 
                amount: i128
            ) {
                let e_clone = e.clone();
                let from_clone = from.clone();
                let pool_clone = pool_address.clone();
                
                [< $inv _helper >](
                    e, pb, ub, pool_address, from, amount, 
                    || { backstop::execute_withdraw(&e_clone, &from_clone, &pool_clone, amount); }
                );
            }
        }
    };
}

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
