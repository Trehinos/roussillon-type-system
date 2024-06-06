//! This module defines the [SequenceType] alias to Vec<[Type]>.
//! 
//! This type is used in any "type collection" like [crate::typing::algebraic::SumType] and [crate::typing::algebraic::ProductType].

use crate::typing::concept::{DataType, Type};

pub type SequenceType = Vec<Type>;

/// Returns the sum of all types in the [SequenceType].
pub fn sum_size(record: &SequenceType) -> usize {
    fn_size(record, |a, b| a + b)
}

/// Returns the result of a reduce operation on size of each type in the [SequenceType]
pub fn fn_size(record: &SequenceType, func: fn(acc: usize, item: usize) -> usize) -> usize {
    record.iter()
        .map(|t| t.size())
        .reduce(func).unwrap_or_default()
}

/// Joins the [SequenceType] typenames in a String with the specified separator.
pub fn join(record: &SequenceType, with: &str) -> String {
    record.iter()
        .map(|t| t.typename()).collect::<Vec<_>>()
        .join(with).to_string()
}

impl DataType for SequenceType {
    fn size(&self) -> usize {
        sum_size(self)
    }

    fn typename(&self) -> String {
        join(self, ",")
    }
}
