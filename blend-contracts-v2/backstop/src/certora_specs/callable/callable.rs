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
macro_rules! parametric_rule {
    ($f:ident, ($($call:ident),+)) => {
        $(paste::paste!(
            #[rule]
            pub fn [< $f _ $call >](e: Env, c: [< call_ $call >]) {   
                // @todo
                $f::<[< call_ $call >]>(e, c);
            }
        );)+
    };
}

#[macro_export]
macro_rules! invariant_rule {
    ($inv:ident) => {
        paste::paste! {
            // Helper function to handle common verification logic
            fn [< $inv _helper >](
                e: Env,
                pb: PoolBalance,
                ub: UserBalance,
                pool: Address,
                user: Address,
                amount: i128,
                call_fn: impl FnOnce()
            ) {
                // Bound inputs and ghost storage
                cvlr_assume!(amount >= i32::MIN as i128 && amount <= i32::MAX as i128);
                cvlr_assume!(pb.shares >= i32::MIN as i128 && pb.shares <= i32::MAX as i128 
                    && pb.tokens >= i32::MIN as i128 && pb.tokens <= i32::MAX as i128 
                    && pb.q4w >= i32::MIN as i128 && pb.q4w <= i32::MAX as i128
                );
                cvlr_assume!(ub.shares >= i32::MIN as i128 && ub.shares <= i32::MAX as i128 
                    && ub.q4w.len() <= 1
                );
                if(ub.q4w.len() == 1) {
                    let e = ub.q4w.get(0).unwrap_optimized();
                    cvlr_assume!(e.amount >= i32::MIN as i128 && e.amount <= i32::MAX as i128);
                }

                // Initialize ghost maps with the provided pool and user balances
                storage::initialize_ghost_maps(pb.clone(), ub.clone());

                // Explicitly assume that reading from the ghost maps returns the expected values
                let read_pb = storage::get_pool_balance(&e, &pool);
                let read_ub = storage::get_user_balance(&e, &pool, &user);
                cvlr_assume!(read_pb.shares == pb.shares);
                cvlr_assume!(read_pb.tokens == pb.tokens);
                cvlr_assume!(read_pb.q4w == pb.q4w);
                cvlr_assume!(read_ub.shares == ub.shares);
                cvlr_assume!(read_ub.q4w.len() == ub.q4w.len());
                if read_ub.q4w.len() == 1 && ub.q4w.len() == 1 {
                    let r_e = read_ub.q4w.get(0).unwrap_optimized();
                    let e = ub.q4w.get(0).unwrap_optimized();
                    cvlr_assume!(r_e.amount == e.amount);
                    cvlr_assume!(r_e.exp == e.exp);
                }

                // Assume valid state
                cvlr_assume!(valid_state_pool_user(e.clone(), pool.clone(), user.clone()));

                // Assume invariant holds before call
                cvlr_assume!($inv(e.clone(), pool.clone(), user.clone()));
                
                // Execute the call and log state before and after
                log_state_details(e.clone(), pool.clone(), user.clone(), amount);
                call_fn();
                log_state_details(e.clone(), pool.clone(), user.clone(), amount);

                // Assert invariant after call
                cvlr_assert!($inv(e.clone(), pool.clone(), user.clone()));
            }
            
            // Rule for execute_deposit
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
            
            // Rule for execute_donate
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
            
            // Rule for execute_draw
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
            
            // Rule for execute_dequeue_withdrawal
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
            
            // Rule for execute_queue_withdrawal
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
            
            // Rule for execute_withdraw
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