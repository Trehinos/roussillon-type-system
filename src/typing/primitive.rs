use std::rc::Rc;
use crate::typing::concept::{DataType, Type};

pub type List = (Type, usize);

#[derive(Clone, Debug)]
pub enum Primitive {
    Boolean,
    Byte,
    Bytes(usize),
    Float,
    Integer,
    Reference(Type),
    List(List),
}

impl Primitive {
    pub fn list(t: Type, len: usize) -> Self {
        Primitive::List((t, len))
    }
    pub fn to_rc(self) -> Type {
        Rc::new(self)
    }
}

impl DataType for Primitive {
    fn size(&self) -> usize {
        match self {
            Primitive::Boolean => 1,
            Primitive::Byte => 1,
            Primitive::Bytes(s) => *s,
            Primitive::Float => 8,
            Primitive::Integer => 8,
            Primitive::Reference(_) => 8,
            Primitive::List(l) => l.0.size() * l.1,
        }
    }

    fn typename(&self) -> String {
        match self {
            Primitive::Boolean => "boolean".to_string(),
            Primitive::Byte => "byte".to_string(),
            Primitive::Bytes(s) => match s {
                2 => "word".to_string(),
                4 => "quad".to_string(),
                8 => "long".to_string(),
                16 => "wide".to_string(),
                &_ => format!("bytes<{}>", s)
            },
            Primitive::Float => "float".to_string(),
            Primitive::Integer => "integer".to_string(),
            Primitive::Reference(t) => format!("&{}", t.typename()),
            Primitive::List(l) => format!("[{};{}]", l.0.typename(), l.1),
        }
    }
}