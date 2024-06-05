use std::cell::RefCell;
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::typing::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};

#[derive(Copy, Clone, Debug)]
pub struct Float(f64);

impl Float {
    pub const fn new(value: f64) -> Self { Self(value) }

    pub fn value(&self) -> f64 {
        self.0
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}

impl DataValue for Float {
    fn data_type(&self) -> Type {
        Primitive::Float.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn set(&mut self, raw: &[u8]) {
        self.0 = f64::from_be_bytes(raw.try_into().unwrap())
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Integer(u64);

impl Integer {
    pub const fn new(value: u64) -> Self { Self(value) }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}

impl DataValue for Integer {
    fn data_type(&self) -> Type {
        Primitive::Integer.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn set(&mut self, raw: &[u8]) {
        self.0 = u64::from_be_bytes(raw.try_into().unwrap())
    }
}
