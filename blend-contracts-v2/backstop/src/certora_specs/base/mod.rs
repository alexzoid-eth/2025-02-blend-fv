pub(crate) mod log_state;
pub(crate) mod vec_one_q4w;

use soroban_sdk::Env;

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
        
        // Initialize ghost storage from rule parameters
        storage::initialize_ghost_maps($pb.clone(), $ub.clone());

        // Explicitly assume that reading from the ghost maps returns the expected values
        let read_pb = storage::get_pool_balance(&$e, &$pool);
        let read_ub = storage::get_user_balance(&$e, &$pool, &$user);
        cvlr_assume!(read_pb.shares == $pb.shares);
        cvlr_assume!(read_pb.tokens == $pb.tokens);
        cvlr_assume!(read_pb.q4w == $pb.q4w);
        cvlr_assume!(read_ub.shares == $ub.shares);
        cvlr_assume!(read_ub.q4w.len() == $ub.q4w.len());

        // Bound inputs
        cvlr_assume!($amount >= i64::MIN as i128 && $amount <= i64::MAX as i128);
        cvlr_assume!(read_pb.shares >= i64::MIN as i128 && read_pb.shares <= i64::MAX as i128);
        cvlr_assume!(read_pb.tokens >= i64::MIN as i128 && read_pb.tokens <= i64::MAX as i128);
        cvlr_assume!(read_pb.q4w >= i64::MIN as i128 && read_pb.q4w <= i64::MAX as i128);
        cvlr_assume!(read_ub.shares >= i64::MIN as i128 && read_ub.shares <= i64::MAX as i128);
        cvlr_assume!(read_ub.q4w.len() <= 1);

        // Assume valid state
        cvlr_assume!($e.ledger().timestamp() != 0);
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