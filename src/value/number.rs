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
    
    pub fn from(raw: &[u8]) -> Self { Self::new(f64::from_be_bytes(raw.try_into().unwrap_or_default())) }
}

impl DataValue for Float {
    fn data_type(&self) -> Type {
        Primitive::Float.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn set(&mut self, raw: &[u8]) {
        *self = Self::from(raw)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Integer(i64);

impl Integer {
    pub const fn new(value: i64) -> Self { Self(value) }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
    pub fn from(raw: &[u8]) -> Self { Self::new(i64::from_be_bytes(raw.try_into().unwrap_or_default())) }
}

impl DataValue for Integer {
    fn data_type(&self) -> Type {
        Primitive::Integer.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn set(&mut self, raw: &[u8]) { *self = Self::from(raw) }
}
