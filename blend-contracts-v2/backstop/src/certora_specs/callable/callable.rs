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
            pub fn [< $f _ $call >](e: Env, pb: PoolBalance, ub: UserBalance, c: [< call_ $call >]) {   

                // Extract pool and user addresses from function arguments
                let pool = c.get_pool_address();
                let user = c.get_user_address();
                let amount = c.get_amount();
                
                // Bound inputs and storage
                cvlr_assume!(amount >= i64::MIN as i128 && amount <= i64::MAX as i128);
                cvlr_assume!(pb.shares >= i64::MIN as i128 && pb.shares <= i64::MAX as i128 
                    && pb.tokens >= i64::MIN as i128 && pb.tokens <= i64::MAX as i128 
                    && pb.q4w >= i64::MIN as i128 && pb.q4w <= i64::MAX as i128
                );
                cvlr_assume!(ub.shares >= i64::MIN as i128 && ub.shares <= i64::MAX as i128 
                    && ub.q4w.len() <= 1
                );
                if(ub.q4w.len() == 1) {
                    let e = ub.q4w.get(0).unwrap_optimized();
                    cvlr_assume!(e.amount >= i64::MIN as i128 && e.amount <= i64::MAX as i128);
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
                
                $f::<[< call_ $call >]>(e, c);
            }
        );)+
    };
}

#[macro_export]
macro_rules! invariant_rule {
    ($inv:ident, ($($call:ident),+)) => {
        paste::paste! {
            pub fn [< inv_ $inv >]<C: AddressCall>(e: Env, pb: PoolBalance, ub: UserBalance, c: C) {
                
                // Extract pool and user addresses from function arguments
                let pool = c.get_pool_address();
                let user = c.get_user_address();
                let amount = c.get_amount();
                
                // Bound inputs and storage
                cvlr_assume!(amount >= i64::MIN as i128 && amount <= i64::MAX as i128);
                cvlr_assume!(pb.shares >= i64::MIN as i128 && pb.shares <= i64::MAX as i128 
                    && pb.tokens >= i64::MIN as i128 && pb.tokens <= i64::MAX as i128 
                    && pb.q4w >= i64::MIN as i128 && pb.q4w <= i64::MAX as i128
                );
                cvlr_assume!(ub.shares >= i64::MIN as i128 && ub.shares <= i64::MAX as i128 
                    && ub.q4w.len() <= 1
                );
                if(ub.q4w.len() == 1) {
                    let e = ub.q4w.get(0).unwrap_optimized();
                    cvlr_assume!(e.amount >= i64::MIN as i128 && e.amount <= i64::MAX as i128);
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
                cvlr_assume!(valid_state_pool_user(&e, &pool, &user));

                // Assume invariant holds before call
                cvlr_assume!($inv(&e, &pool, &user));
                
                // Execute the call
                log_state_details(&e, &pool, &user, amount);
                c.call(&e);
                log_state_details(&e, &pool, &user, amount);

                // Assert invariant after call
                cvlr_assert!($inv(&e, &pool, &user));
            }
            
            // Generate specific rule functions for each call type
            $(
                #[rule]
                pub fn [< $inv _ $call >](e: Env, pb: PoolBalance, ub: UserBalance, c: [< call_ $call >]) {
                    [< inv_ $inv >]::<[< call_ $call >]>(e, pb, ub, c);
                }
            )+
        }
    };
}