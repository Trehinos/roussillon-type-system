//! This module provides common constructions to create custom types.
//!
//! It lets the user identify algebraic types :
//! - [Structure] is an [Identified] : [ProductType],
//! - [Enumeration] is an [Identified] : [SumType],

use std::rc::Rc;
use crate::identity::{Identified, Identifier};
use crate::types::algebraic::{ProductType, SumType};
use crate::types::concept::{DataType, Type};
use crate::types::primitive::Primitive;
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;
use crate::value::record::Record;

/// A [Structure] is an identified [ProductType].
#[derive(Clone, Debug)]
pub struct Structure {
    identifier: Identifier,
    pub product_type: ProductType,
}

impl Structure {
    pub fn new(identifier: &str, fields: ProductType) -> Self {
        Structure {
            identifier: Identifier::new(identifier),
            product_type: fields,
        }
    }

    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for Structure {
    fn size(&self) -> usize { self.product_type.size() }

    fn typename(&self) -> String { self.identifier.to_string() }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        Ok(Record::from(self.clone().to_rc(), raw)?.to_cell())
    }
}

impl Identified for Structure {
    fn identifier(&self) -> Identifier {
        self.identifier.clone()
    }
}

/// A [Enumeration] is an identified [SumType].
#[derive(Clone, Debug)]
pub struct Enumeration {
    identifier: Identifier,
    pub sum_type: SumType,
}

impl Enumeration {
    pub fn new(identifier: &str, sum_type: SumType) -> Self {
        Enumeration {
            identifier: Identifier::new(identifier),
            sum_type,
        }
    }

    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for Enumeration {
    fn size(&self) -> usize { self.sum_type.size() }

    fn typename(&self) -> String { self.identifier.to_string() }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        self.sum_type.construct_from_raw(raw)
    }
}

impl Identified for Enumeration {
    fn identifier(&self) -> Identifier {
        self.identifier.clone()
    }
}