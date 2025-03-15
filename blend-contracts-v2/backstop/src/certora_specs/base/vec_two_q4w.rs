use soroban_sdk::{contracttype, Env};
use crate::backstop::Q4W;

// A simplified Vec implementation for Q4W that can hold up to two elements
#[derive(Clone)]
#[contracttype]
pub enum VecTwoQ4W {
    Empty,
    One(Q4W),
    Two(Q4W, Q4W),
}

impl VecTwoQ4W {
    pub fn new(e: &Env) -> Self {
        VecTwoQ4W::Empty
    }

    pub fn len(&self) -> u32 {
        match self {
            VecTwoQ4W::Empty => 0,
            VecTwoQ4W::One(_) => 1,
            VecTwoQ4W::Two(_, _) => 2,
        }
    }

    pub fn push_back(&mut self, value: Q4W) {
        match self {
            VecTwoQ4W::Empty => *self = VecTwoQ4W::One(value),
            VecTwoQ4W::One(v1) => *self = VecTwoQ4W::Two(v1.clone(), value),
            VecTwoQ4W::Two(_, _) => panic!("VecTwoQ4W already contains two elements"),
        }
    }

    pub fn push_front(&mut self, value: Q4W) {
        match self {
            VecTwoQ4W::Empty => *self = VecTwoQ4W::One(value),
            VecTwoQ4W::One(v1) => *self = VecTwoQ4W::Two(value, v1.clone()),
            VecTwoQ4W::Two(_, _) => panic!("VecTwoQ4W already contains two elements"),
        }
    }

    pub fn pop_back_unchecked(&mut self) -> Q4W {
        match self {
            VecTwoQ4W::Empty => panic!("VecTwoQ4W is empty"),
            VecTwoQ4W::One(v1) => {
                let result = v1.clone();
                *self = VecTwoQ4W::Empty;
                result
            },
            VecTwoQ4W::Two(v1, v2) => {
                let result = v2.clone();
                *self = VecTwoQ4W::One(v1.clone());
                result
            }
        }
    }

    pub fn pop_front_unchecked(&mut self) -> Q4W {
        match self {
            VecTwoQ4W::Empty => panic!("VecTwoQ4W is empty"),
            VecTwoQ4W::One(v1) => {
                let result = v1.clone();
                *self = VecTwoQ4W::Empty;
                result
            },
            VecTwoQ4W::Two(v1, v2) => {
                let result = v1.clone();
                *self = VecTwoQ4W::One(v2.clone());
                result
            }
        }
    }

    pub fn get(&self, index: u32) -> Option<Q4W> {
        match (self, index) {
            (VecTwoQ4W::One(v1), 0) => Some(v1.clone()),
            (VecTwoQ4W::Two(v1, _), 0) => Some(v1.clone()),
            (VecTwoQ4W::Two(_, v2), 1) => Some(v2.clone()),
            _ => None,
        }
    }

    pub fn last(&self) -> Option<Q4W> {
        match self {
            VecTwoQ4W::Empty => None,
            VecTwoQ4W::One(v1) => Some(v1.clone()),
            VecTwoQ4W::Two(_, v2) => Some(v2.clone()),
        }
    }

    pub fn first(&self) -> Option<Q4W> {
        match self {
            VecTwoQ4W::Empty => None,
            VecTwoQ4W::One(v1) => Some(v1.clone()),
            VecTwoQ4W::Two(v1, _) => Some(v1.clone()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            VecTwoQ4W::Empty => true,
            _ => false,
        }
    }
    
    pub fn is_full(&self) -> bool {
        match self {
            VecTwoQ4W::Two(_, _) => true,
            _ => false,
        }
    }
        
    // Get a specific inner value (0 or 1) directly
    pub fn inner_value(&self, index: u32) -> Option<Q4W> {
        self.get(index)
    }
}

impl cvlr::nondet::Nondet for VecTwoQ4W {
    fn nondet() -> Self {
        // Randomly return one of the three states
        let variant: u8 = cvlr::nondet();
        match variant % 3 {
            0 => VecTwoQ4W::Empty,
            1 => VecTwoQ4W::One(cvlr::nondet()),
            _ => VecTwoQ4W::Two(cvlr::nondet(), cvlr::nondet()),
        }
    }
}

#[macro_export]
macro_rules! vec_two {
    ($env:expr $(,)?) => {
        $crate::certora_specs::base::vec_two_q4w::VecTwoQ4W::new($env)
    };
    ($env:expr, $x:expr $(,)?) => {
        {
            let mut vec = $crate::certora_specs::base::vec_two_q4w::VecTwoQ4W::new($env);
            vec.push_back($x);
            vec
        }
    };
    ($env:expr, $x:expr, $y:expr $(,)?) => {
        {
            let mut vec = $crate::certora_specs::base::vec_two_q4w::VecTwoQ4W::new($env);
            vec.push_back($x);
            vec.push_back($y);
            vec
        }
    };
}

pub fn vec_two_empty(e: &Env) -> VecTwoQ4W {
    VecTwoQ4W::Empty
}