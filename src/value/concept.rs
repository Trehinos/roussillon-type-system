use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::value::error::{CanTypeError, TypeError};

pub trait DataValue {
    fn data_type(&self) -> Type;

    fn raw(&self) -> Vec<u8>;

    fn set(&mut self, raw: &[u8]);

    fn validate_type(&self, expected_type: &Type) -> CanTypeError {
        if self.data_type().typename() == expected_type.typename() {
            Ok(())
        } else {
            Err(TypeError::InvalidType { expected: expected_type.clone(), provided: self.data_type() })
        }
    }
}

impl Debug for dyn DataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:x?}", self.data_type(), self.raw())
    }
}

pub type ValueCell = Rc<RefCell<dyn DataValue>>;