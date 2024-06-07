use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::typing::sequence::SequenceType;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::{SequenceError, TypeResult};

#[derive(Clone, Debug)]
pub struct Sequence {
    definition: SequenceType,
    values: Vec<ValueCell>,
}

impl Sequence {
    
    pub fn empty() -> Self {
        Self {
            definition: vec![],
            values: vec![],
        }
    }
    
    pub fn new(definition: SequenceType, values: &[ValueCell]) -> TypeResult<Self> {
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
