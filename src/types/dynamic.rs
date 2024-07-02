use std::rc::Rc;
use crate::types::concept::{DataType, Type};
use crate::value::concept::ValueCell;
use crate::value::error::{TypeError, TypeResult};

pub struct VoidType;

impl VoidType {
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for VoidType {
    fn size(&self) -> usize { 0 }

    fn typename(&self) -> String { "void".to_string() }

    fn construct_from_raw(&self, _: &[u8]) -> TypeResult<ValueCell> { Err(TypeError::Message("Cannot construct an object from a <void>.".to_string())) }
}

pub struct UnknownType;

impl DataType for UnknownType {
    fn size(&self) -> usize { 0 }

    fn typename(&self) -> String { "unknown".to_string() }

    fn construct_from_raw(&self, _: &[u8]) -> TypeResult<ValueCell> { Err(TypeError::Message("Cannot construct an object from an <Unknown>.".to_string())) }
}

#[derive(Clone)]
pub enum Dynamic {
    Unknown,
    Named(String),
    Defined(Type),
}

impl DataType for Dynamic {
    fn size(&self) -> usize {
        match self {
            Dynamic::Unknown => 0,
            Dynamic::Named(_) => 0,
            Dynamic::Defined(t) => t.size(),
        }
    }

    fn typename(&self) -> String {
        match self {
            Dynamic::Unknown => "dyn<?>".to_string(),
            Dynamic::Named(n) => format!("dyn<{}>", n),
            Dynamic::Defined(t) => t.typename(),
        }
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        match self {
            Dynamic::Unknown => UnknownType.construct_from_raw(raw),
            Dynamic::Named(_) => UnknownType.construct_from_raw(raw),
            Dynamic::Defined(t) => t.construct_from_raw(raw),
        }
    }
}