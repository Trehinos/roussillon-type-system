use std::rc::Rc;
use crate::types::concept::{DataType, Type};
use crate::value::concept::ValueCell;
use crate::value::error::{TypeError, TypeResult};

#[derive(Clone)]
pub enum TypeVariable {
    Label(String),
    Type(Type),
}

impl TypeVariable {
    pub fn typename(&self) -> String {
        match self {
            TypeVariable::Label(l) => format!("dyn<{}>", l),
            TypeVariable::Type(t) => t.typename(),
        }
    }
}

pub struct VoidType;

impl VoidType {
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for VoidType {
    fn size(&self) -> usize { 0 }

    fn typename(&self) -> String { "void".to_string() }

    fn construct_from_raw(&self, _: &[u8]) -> TypeResult<ValueCell> { Err(TypeError::Message("Cannot construct an object from a <void>.".to_string())) }
}

pub struct AnyType;

impl DataType for AnyType {
    fn size(&self) -> usize { 0 }

    fn typename(&self) -> String { "any".to_string() }

    fn construct_from_raw(&self, _: &[u8]) -> TypeResult<ValueCell> { Err(TypeError::Message("Cannot construct an object from an <Any>.".to_string())) }
}

#[derive(Clone)]
pub enum Dynamic {
    Any,
    TypeVar(TypeVariable),
    Defined(Type),
}

impl DataType for Dynamic {
    fn size(&self) -> usize {
        match self {
            Dynamic::Any => 0,
            Dynamic::TypeVar(_) => 0,
            Dynamic::Defined(t) => t.size(),
        }
    }

    fn typename(&self) -> String {
        match self {
            Dynamic::Any => "any".to_string(),
            Dynamic::TypeVar(n) => n.typename(),
            Dynamic::Defined(t) => t.typename(),
        }
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        match self {
            Dynamic::Any => AnyType.construct_from_raw(raw),
            Dynamic::TypeVar(v) => match v {
                TypeVariable::Label(_) => AnyType.construct_from_raw(raw),
                TypeVariable::Type(t) => t.construct_from_raw(raw),
            },
            Dynamic::Defined(t) => t.construct_from_raw(raw),
        }
    }
}

pub fn top_type() -> Type { Rc::new(Dynamic::Any) }