//! This module provides ways to manage Algebraic Data Types.
//!
//! - The [SumType] struct describes a type which can have a value of one of its variant.
//! - The [ProductType] struct describes a type which has a value for each one of its composing type.

use std::cmp::max;
use std::ops::{Add, Mul};
use std::rc::Rc;

use crate::types::concept::{DataType, Type};

use crate::types::sequence;
use crate::types::sequence::Tuple;
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;
use crate::value::union::SumValue;

/// This struct describes a type which can have a value of one of its variant.
#[derive(Clone, Debug)]
pub struct SumType(Tuple);

impl SumType {
    pub fn new(types: &[Type]) -> Self { Self(types.to_vec()) }
    pub fn to_tuple(&self) -> Tuple { self.0.to_vec() }
    pub fn variant(&self, tag: usize) -> Option<Type> { self.0.get(tag).cloned() }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for SumType {
    fn size(&self) -> usize {
        sequence::fn_size(&self.0, max)
    }

    fn typename(&self) -> String {
        format!("<{}>", sequence::join(&self.0, "|"))
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        Ok(SumValue::from(self.clone().to_rc(), raw)?.to_cell())
    }
}

impl Add<Type> for SumType {
    type Output = Self;

    fn add(self, rhs: Type) -> Self::Output {
        let mut new = self.clone();
        new.0.push(rhs);
        new
    }
}

/// This struct describes a type which has a value for each one of its composing type.
#[derive(Clone, Debug)]
pub struct ProductType(Tuple);

impl ProductType {
    pub const fn unit_type() -> Self { Self(Vec::new()) }
    pub fn new(types: &[Type]) -> Self { Self(types.to_vec()) }
    pub fn to_tuple(&self) -> Tuple { self.0.to_vec() }
    pub fn is_unit_type(&self) -> bool { self.0.is_empty() }
    pub fn field(&self, index: usize) -> Option<Type> { self.0.get(index).cloned() }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl Mul<Type> for ProductType {
    type Output = Self;

    fn mul(self, rhs: Type) -> Self::Output {
        let mut new = self.clone();
        new.0.push(rhs);
        new
    }
}

impl DataType for ProductType {
    fn size(&self) -> usize { self.0.size() }

    fn typename(&self) -> String { format!("<{}>", sequence::join(&self.0, "&")) }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        self.0.construct_from_raw(raw)
    }
}

/// The `never` type.
pub fn bottom_type() -> Type {
    SumType::new(&[]).to_rc()
}
