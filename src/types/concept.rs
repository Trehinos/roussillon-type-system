//! Defines the [DataType] trait and its [Display] and [Debug] static implementations.
//!
//! This module also provides the [Type] alias (to `Rc<dyn DataType>`).
//!
//! Any `struct` describing a Roussillon data type *MUST* implement this trait.

use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;

/// A trait for structs that represent a data type.
pub trait DataType {
    /// The allocation size of an element for this [DataType] at compile time.
    fn size(&self) -> usize;

    /// The name of this type.
    fn typename(&self) -> String;
    
    /// Construct a new ValueCell.
    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell>;
}

/// A reference-counted dynamic [DataType].
pub type Type = Rc<dyn DataType>;

impl Display for dyn DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.typename())
    }
}

impl Debug for dyn DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self)
    }
}

impl PartialEq for dyn DataType {
    fn eq(&self, other: &Self) -> bool {
        self.typename() == other.typename()
    }
}

impl Eq for dyn DataType {}
