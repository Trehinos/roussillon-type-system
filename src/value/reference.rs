use std::cell::RefCell;
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::typing::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};

#[derive(Clone, Debug)]
pub struct Reference {
    to_type: Type,
    address: usize,
}

impl Reference {
    pub fn new(to_type: Type, address: usize) -> Self {
        Self { to_type, address }
    }
    pub fn get_address(&self) -> usize {
        self.address
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
    pub fn referenced(&self) -> &Type { &self.to_type }
    pub fn from(t: Type, raw: &[u8]) -> Self {
        Self::new(t, usize::from_be_bytes(raw.try_into().unwrap_or_default()))
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.to_type.typename() == other.to_type.typename()
            && self.address == other.address
    }
}

impl Eq for Reference {}

impl DataValue for Reference {
    fn data_type(&self) -> Type {
        Primitive::Reference(self.to_type.clone()).to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        self.address.to_be_bytes().to_vec()
    }

    fn set(&mut self, raw: &[u8]) {
        self.address = usize::from_be_bytes(raw.try_into().unwrap())
    }
}