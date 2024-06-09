use std::cell::RefCell;
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::typing::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};

#[derive(Clone, Debug)]
pub struct Boolean(bool);

impl Boolean {
    pub fn create_true() -> Self { Self(true) }

    pub fn create_false() -> Self { Self(false) }

    pub fn get(&self) -> bool { self.0 }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }

    pub fn from(raw: &[u8]) -> Self {
        if let Some(value) = raw.first() {
            Self(*value != 0)
        } else {
            panic!("Unable to create a Boolean from empty data.")
        }
    }
}

impl DataValue for Boolean {
    fn data_type(&self) -> Type { Primitive::Boolean.to_rc() }

    fn raw(&self) -> Vec<u8> { vec![if self.0 { 1u8 } else { 0u8 }] }

    fn set(&mut self, raw: &[u8]) {
        *self = Self::from(raw)
    }
}