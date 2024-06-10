use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;

use crate::types::concept::Type;
use crate::types::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::CanTypeError;
use crate::value::sequence::values_to_raw;

#[derive(Clone, Debug)]
pub struct List {
    of_type: Type,
    elements: Vec<ValueCell>,
}

impl List {
    pub fn empty(of_type: Type) -> Self {
        Self { of_type, elements: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    pub fn push(&mut self, value: ValueCell) -> CanTypeError {
        value.borrow().validate_type(&self.of_type)?;
        self.elements.push(value);
        Ok(())
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
    
    pub fn item(&self, index: usize) -> Option<ValueCell> {
        self.elements.get(index).cloned()
    }

    pub fn from(of_type: Type, size: usize, raw: &[u8]) -> Self {
        let mut elements = Vec::new();
        for i in 0..size {
            let raw_element = &raw[i * of_type.size()..(i + 1) * of_type.size()];
            let element = of_type.construct_from_raw(raw_element).unwrap();
            elements.push(element);
        }
        List { of_type, elements }
    }
}

impl Index<usize> for List {
    type Output = ValueCell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl DataValue for List {
    fn data_type(&self) -> Type {
        Primitive::list(self.of_type.clone(), self.elements.len()).to_rc()
    }

    fn raw(&self) -> Vec<u8> { values_to_raw(&self.elements) }

    fn set(&mut self, raw: &[u8]) {
        let mut pointer: usize = 0;
        let size = self.data_type().size();
        for i in self.elements.iter_mut() {
            let pointer_end = pointer + size;
            i.borrow_mut().set(&raw[pointer..pointer_end]);
            pointer = pointer_end;
        }
    }
}