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
                
                // Assume valid state and bound inputs
                setup_verification_context(&e, &c, pb, ub);                
                
                // Execute the rule's body function
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
                
                // Assume valid state and bound inputs
                setup_verification_context(&e, &c, pb, ub);                

                // Extract pool and user addresses from function arguments
                let pool = c.get_pool_address();
                let user = c.get_user_address();
                
                // Assume invariant holds before call
                cvlr_assume!($inv(&e, pool, user));
                
                // Execute the call
                log_state_details(&e, pool, user, concat!("Before call for invariant: ", stringify!($inv)));
                c.call(&e);
                log_state_details(&e, pool, user, concat!("After call for invariant: ", stringify!($inv)));

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