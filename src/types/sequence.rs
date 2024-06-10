//! This module defines the [Tuple] alias to Vec<[Type]>.
//!
//! This type is used in any "type collection" like [crate::types::algebraic::SumType] and [crate::types::algebraic::ProductType].

use crate::types::concept::{DataType, Type};
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;
use crate::value::sequence::Sequence;

pub type Tuple = Vec<Type>;

/// Returns the sum of all types in the [Tuple].
pub fn sum_size(record: &Tuple) -> usize {
    fn_size(record, |a, b| a + b)
}

/// Returns the result of a reduce operation on size of each type in the [Tuple]
pub fn fn_size(record: &Tuple, func: fn(acc: usize, item: usize) -> usize) -> usize {
    record.iter()
        .map(|t| t.size())
        .reduce(func).unwrap_or_default()
}

/// Joins the [Tuple] typenames in a String with the specified separator.
pub fn join(record: &Tuple, with: &str) -> String {
    record.iter()
        .map(|t| t.typename()).collect::<Vec<_>>()
        .join(with).to_string()
}

impl DataType for Tuple {
    fn size(&self) -> usize {
        sum_size(self)
    }

    fn typename(&self) -> String {
        join(self, ",")
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        Ok(Sequence::from(self.clone(), raw)?.to_cell())
    }
}
