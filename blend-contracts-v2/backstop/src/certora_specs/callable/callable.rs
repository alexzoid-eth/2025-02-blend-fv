use soroban_sdk::Env;

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

pub trait Call {
    fn call(&self, env: &Env);
}

#[macro_export]
macro_rules! parametric_rule {
    ($f:ident, ($($call:ident),+)) => {
        $(paste::paste!(
            #[rule]
            pub fn [< $f _ $call >](e: Env, c: [< call_ $call >]) {
                $f::<[< call_ $call >]>(e, c);
            }
        );)+
    };
}

/*
#[macro_export]
macro_rules! invariant_rule {
    ($invariant:ident, ($($call:ident),+)) => {
        $(paste::paste!(
            #[rule]
            pub fn [< inv_ $invariant _ $call >](e: Env, c: [< call_ $call >]) {
                let pool = c.get_pool_address();
                let user = c.get_user_address();
                
                // Set up environment
                cvlr_assume!(valid_state_pool(&e, &pool));
                if user != &e.current_contract_address() {
                    cvlr_assume!(valid_state_pool_user(&e, &pool, &user));
                }
                cvlr_assume!(bound_amount(c.get_amount()));
                
                // Check that invariant holds before call
                cvlr_assume!($invariant(&e, &pool, &user));
                
                // Make the call
                c.call(&e);
                
                // Assert that invariant still holds after call
                cvlr_assert!($invariant(&e, &pool, &user));
            }
        );)+
    };
    
    // Variant for invariants that only need pool address
    ($invariant:ident, pool_only, ($($call:ident),+)) => {
        $(paste::paste!(
            #[rule]
            pub fn [< inv_ $invariant _ $call >](e: Env, c: [< call_ $call >]) {
                let pool = c.get_pool_address();
                
                // Set up environment
                cvlr_assume!(valid_state_pool(&e, &pool));
                cvlr_assume!(bound_amount(c.get_amount()));
                
                // Check that invariant holds before call
                cvlr_assume!($invariant(&e, &pool));
                
                // Make the call
                c.call(&e);
                
                // Assert that invariant still holds after call
                cvlr_assert!($invariant(&e, &pool));
            }
        );)+
    };
}
*/