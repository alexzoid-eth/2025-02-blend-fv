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
                // Initialize ghost maps with the provided pool and user balances
                storage::initialize_ghost_maps(pool, user, pb, ub);
                // Execute rule function
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
                
                // Initialize ghost maps with the provided pool and user balances
                storage::initialize_ghost_maps(pool, user, pb, ub);
                                
                // Assume valid state and bounded amount
                cvlr_assume!(valid_state_pool_user(&e, pool, user));
                cvlr_assume!(bound_amount(c.get_amount()));
                
                // Check invariant before call
                cvlr_assume!($inv(&e, pool, user));
                
                // Execute the call
                c.call(&e);
                                
                // Assert invariant after call
                cvlr_assert!($inv(&e, pool, user));
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