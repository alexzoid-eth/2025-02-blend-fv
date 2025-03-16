use soroban_sdk::{contracttype, Env};
use crate::backstop::Q4W;

// A simplified Vec implementation specifically for Q4W that can hold at most one element
#[derive(Clone)]
#[contracttype]
pub enum VecOneQ4W {
    Empty,
    One(Q4W),
}

impl VecOneQ4W {
    pub fn new(e: &Env) -> Self {
        VecOneQ4W::Empty
    }

    pub fn len(&self) -> u32 {
        match self {
            VecOneQ4W::Empty => 0,
            VecOneQ4W::One(_) => 1,
        }
    }

    pub fn push_back(&mut self, value: Q4W) {
        match self {
            VecOneQ4W::Empty => *self = VecOneQ4W::One(value),
            VecOneQ4W::One(_) => panic!("VecOneQ4W already contains an element"),
        }
    }

    pub fn push_front(&mut self, value: Q4W) {
        self.push_back(value)
    }

    pub fn pop_back_unchecked(&mut self) -> Q4W {
        let result = match self {
            VecOneQ4W::Empty => panic!("VecOneQ4W is empty"),
            VecOneQ4W::One(ref val) => val.clone(),
        };
        *self = VecOneQ4W::Empty;
        result
    }

    pub fn pop_front_unchecked(&mut self) -> Q4W {
        self.pop_back_unchecked()
    }

    pub fn get(&self, index: u32) -> Option<Q4W> {
        match (self, index) {
            (VecOneQ4W::One(val), 0) => Some(val.clone()),
            _ => None,
        }
    }

    pub fn last(&self) -> Option<Q4W> {
        match self {
            VecOneQ4W::Empty => None,
            VecOneQ4W::One(val) => Some(val.clone()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            VecOneQ4W::Empty => true,
            VecOneQ4W::One(_) => false,
        }
    }
    
    // Get the inner value directly
    pub fn inner_value(&self) -> Option<Q4W> {
        match self {
            VecOneQ4W::Empty => None,
            VecOneQ4W::One(q4w) => Some(q4w.clone()),
        }
    }
}

impl cvlr::nondet::Nondet for VecOneQ4W {
    fn nondet() -> Self {
        if cvlr::nondet() {
            VecOneQ4W::Empty
        } else {
            VecOneQ4W::One(cvlr::nondet())
        }
    }
}

#[macro_export]
macro_rules! vec_one {
    ($env:expr $(,)?) => {
        $crate::certora_specs::mocks::vec_one_q4w::VecOneQ4W::new($env)
    };
    ($env:expr, $x:expr $(,)?) => {
        {
            let mut vec = $crate::certora_specs::mocks::vec_one_q4w::VecOneQ4W::new($env);
            vec.push_back($x);
            vec
        }
    };
}

pub fn vec_one_empty(e: &Env) -> VecOneQ4W {
    VecOneQ4W::Empty
}