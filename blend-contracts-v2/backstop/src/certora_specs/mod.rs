pub(crate) mod summaries;
pub(crate) mod mocks;
pub(crate) mod callable;
pub(crate) mod declarations;

pub(crate) mod valid_state;
pub(crate) mod valid_state_sanity;
pub(crate) mod state_trans;

// https://github.com/Certora/reflector-subscription-contract/blob/51944577dc4536e9cf9711db6e125fe1e2254054/src/certora_specs/mod.rs
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