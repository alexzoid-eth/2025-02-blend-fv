pub(crate) mod summaries;
pub(crate) mod mocks;
pub(crate) mod callable;
pub(crate) mod declarations;

pub(crate) mod valid_state;
pub(crate) mod valid_state_sanity;
pub(crate) mod state_trans;

// https://github.com/Certora/reflector-subscription-contract/blob/51944577dc4536e9cf9711db6e125fe1e2254054/src/certora_specs/mod.rs
use cvlr::nondet::*;
pub enum GhostMap<K, V> {
    UnInit,
    Init { k: K, v: V }
}

impl<K: Clone + Eq, V: Nondet + Clone> GhostMap<K, V> {
    #[inline(never)]
    pub fn init(&mut self, k: &K, v: V) {
        *self = Self::Init { k: k.clone(), v };
    }

    #[inline(never)]
    pub fn set(&mut self, k: &K, v: V) {
        match self {
            Self::Init { k: my_k, v: my_v } => {
                if k == my_k {
                    *my_v = v;
                }
            },
            Self::UnInit => {
                *self = Self::Init { k: k.clone(), v };
            }
        }
    }

    #[inline(never)]
    pub fn get(&self, k: &K) -> V {
        match self {
            Self::UnInit => V::nondet(),
            Self::Init { k: my_k, v: my_v } => {
                if k == my_k {
                    my_v.clone()
                } else {
                    V::nondet()
                }
            }
        }
    }
}