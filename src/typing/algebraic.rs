//! This module provides ways to manage Algebraic Data Types.
//! 
//! - The [SumType] struct describes a type which can have a value of one of its variant.
//! - The [ProductType] struct describes a type which has a value for each one of its composing type.

use std::cmp::max;
use std::rc::Rc;

use crate::typing::concept::{DataType, Type};

use crate::typing::sequence;
use crate::typing::sequence::SequenceType;

/// This struct describes a type which can have a value of one of its variant.
#[derive(Clone, Debug)]
pub struct SumType(SequenceType);

impl SumType {
    pub fn new(types: &[Type]) -> Self { Self(types.to_vec()) }
    pub fn to_sequence_type(&self) -> SequenceType { self.0.to_vec() }
    pub fn get_type(&self, tag: usize) -> Option<Type> { self.0.get(tag).cloned() }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for SumType {
    fn size(&self) -> usize {
        sequence::fn_size(&self.0, max)
    }

    fn typename(&self) -> String {
        format!("<{}>", sequence::join(&self.0, "|"))
    }
}

/// This struct describes a type which has a value for each one of its composing type.
#[derive(Clone, Debug)]
pub struct ProductType(SequenceType);

impl ProductType {
    pub const fn unit_type() -> Self { Self(Vec::new()) }
    pub fn new(types: &[Type]) -> Self { Self(types.to_vec()) }
    pub fn to_sequence_type(&self) -> SequenceType { self.0.to_vec() }
    pub fn is_unit_type(&self) -> bool { self.0.is_empty() }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for ProductType {
    fn size(&self) -> usize { self.0.size() }

    fn typename(&self) -> String { format!("<{}>", sequence::join(&self.0, "&")) }
}

pub struct Gadt {
    
}