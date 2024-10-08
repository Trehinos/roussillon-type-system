use std::cell::RefCell;
use std::rc::Rc;
use crate::parse::{parse_slice, Parsed};

use crate::types::concept::Type;
use crate::types::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};

#[derive(Copy, Clone, Debug)]
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

    pub fn parse(input: &[u8]) -> Parsed<Self> {
        let (Some(raw), rest) = parse_slice(input, 1) else { return (None, input); };
        (Some(Self::from(raw)), rest)
    }
}

impl DataValue for Boolean {
    fn data_type(&self) -> Type { Primitive::Boolean.to_rc() }

    fn raw(&self) -> Vec<u8> { vec![if self.0 { 1u8 } else { 0u8 }] }

    fn set(&mut self, raw: &[u8]) {
        *self = Self::from(raw)
    }
}