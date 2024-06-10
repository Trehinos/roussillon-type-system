use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::types::concept::{DataType, Type};
use crate::types::sequence::Tuple;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::{SequenceError, TypeResult};

#[derive(Clone, Debug)]
pub struct Sequence {
    definition: Tuple,
    values: Vec<ValueCell>,
}

impl Sequence {
    
    pub fn empty() -> Self {
        Self {
            definition: vec![],
            values: vec![],
        }
    }
    
    pub fn new(definition: Tuple, values: &[ValueCell]) -> TypeResult<Self> {
        if definition.len() != values.len() {
            return Err(SequenceError::SequenceLengthMismatch { expected: 0, provided: 0 }.promote());
        }
        for (index, expected) in definition.iter().enumerate() {
            values[index].borrow().validate_type(expected)?;
        }
        Ok(Self { definition, values: values.to_vec() })
    }

    pub fn values(&self) -> &[ValueCell] {
        &self.values
    }

    pub fn get(&self) -> Vec<ValueCell> {
        self.values().to_vec()
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
    
    pub fn from(definition: Tuple, raw: &[u8]) -> TypeResult<Self> {
        let mut values = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for t in definition.iter() {
            end += t.size();
            values.push(t.construct_from_raw(&raw[start..end])?);
            start = end;
        }
        Self::new(definition.clone(), &values)
    }
}

pub fn values_to_raw(values: &[ValueCell]) -> Vec<u8> {
    let mut raw = Vec::new();
    for element in values {
        raw.extend(element.borrow().raw());
    }
    raw
}

impl DataValue for Sequence {
    fn data_type(&self) -> Type {
        Rc::new(self.definition.clone())
    }

    fn raw(&self) -> Vec<u8> { values_to_raw(&self.values) }

    fn set(&mut self, raw: &[u8]) {
        let mut pointer: usize = 0;
        for i in self.values.iter_mut() {
            let pointer_end = pointer + i.borrow().data_type().size();
            i.borrow_mut().set(&raw[pointer..pointer_end]);
            pointer = pointer_end;
        }
    }
}
