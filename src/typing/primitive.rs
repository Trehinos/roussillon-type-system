//! This module provides the machines types :
//! - [Primitive::Boolean]
//! - [Primitive::Byte]
//! - [Primitive::Bytes]
//! - [Primitive::Float]
//! - [Primitive::Integer]
//! - [Primitive::Reference]
//! - [Primitive::List]

use std::rc::Rc;
use crate::typing::concept::{DataType, Type};

pub type List = (Type, usize);

/// This enum provides all primitive types.
#[derive(Clone, Debug)]
pub enum Primitive {
    /// A type to hold `true` or `false` values.
    ///
    /// Value is [crate::value::boolean::Boolean]
    Boolean,
    /// A type to manage byte at low level.
    ///
    /// Value is [crate::value::byte::Bytes::Byte]
    Byte,
    /// A type to manage words composed of bytes at low level.
    ///
    /// Value can be :
    /// * [crate::value::byte::Bytes::Word] for size = 2,
    /// * [crate::value::byte::Bytes::Quad] for size = 4,
    /// * [crate::value::byte::Bytes::Long] for size = 8,
    /// * [crate::value::byte::Bytes::Wide] for size = 16, or,
    /// * [crate::value::byte::Bytes::Bytes] for any other size.
    Bytes(usize),
    /// A type to perform floating-point arithmetic.
    /// 
    /// Value is [crate::value::number::Float]
    Float,
    /// A type to perform integral arithmetic.
    ///
    /// Value is [crate::value::number::Integer]
    Integer,
    /// A type intended to retain an address to a value of a specific type.
    ///
    /// Value is [crate::value::reference::Reference]
    Reference(Type),
    /// A collection type which contains many elements of a specific type.
    ///
    /// Value is [crate::value::list::List]
    List(List),
}

impl Primitive {
    /// Creates a new List type. The resulting type will be `[t; len]`.
    pub fn list(t: Type, len: usize) -> Self {
        Primitive::List((t, len))
    }

    /// Transforms the current instance to a new Rc.
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