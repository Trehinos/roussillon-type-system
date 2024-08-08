//! This module provides the machines types :
//! - [Primitive::Boolean]
//! - [Primitive::Byte]
//! - [Primitive::Bytes]
//! - [Primitive::Float]
//! - [Primitive::Integer]
//! - [Primitive::Reference]
//! - [Primitive::List]

use std::rc::Rc;
use crate::types::concept::{DataType, Type};
use crate::value::boolean::Boolean;
use crate::value::byte::Bytes;
use crate::value::concept::{GetDataValue, ValueCell};
use crate::value::error::{TypeError, TypeResult};
use crate::value::number::{Float, Integer};
use crate::value::reference::Reference;
use crate::value::list::List as ListValue;

pub type List = (Type, usize);

/// This enum provides all primitive types.
#[derive(Clone, Debug)]
pub enum Primitive {
    /// A type to hold `true` or `false` values.
    ///
    /// Value is [Boolean]
    Boolean,
    /// A type to manage byte at low level.
    ///
    /// Value is [Bytes::Byte]
    Byte,
    /// A type to manage words composed of bytes at low level.
    ///
    /// Value can be :
    /// * [Bytes::Arch] for size = `size_of::<usize>()`,
    /// * [Bytes::Word] for size = 2,
    /// * [Bytes::Quad] for size = 4,
    /// * [Bytes::Long] for size = 8,
    /// * [Bytes::Wide] for size = 16, or,
    /// * [Bytes::Bytes] for any other size.
    Bytes(usize),
    /// A type to perform floating-point arithmetic.
    ///
    /// Value is [Float]
    Float,
    /// A type to perform integral arithmetic.
    ///
    /// Value is [Integer]
    Integer,
    /// A type intended to retain an address to a value of a specific type.
    ///
    /// Value is [Reference]
    Reference(Type),
    /// A collection type which contains many elements of a specific type.
    ///
    /// Value is [list::List]
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

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        match self {
            Primitive::Boolean => Ok(Boolean::from(raw).to_cell()),
            Primitive::Byte => if raw.len() == 1 { Ok(Bytes::from_raw(raw).to_cell()) } else {
                Err(TypeError::InvalidType {
                    expected: self.clone().to_rc(),
                    provided: Primitive::Bytes(raw.len()).to_rc(),
                })
            },
            Primitive::Bytes(s) => if raw.len() == *s { Ok(Bytes::from_raw(raw).to_cell()) } else {
                Err(TypeError::InvalidType {
                    expected: self.clone().to_rc(),
                    provided: Primitive::Bytes(raw.len()).to_rc(),
                })
            },
            Primitive::Float => Ok(Float::from(raw).to_cell()),
            Primitive::Integer => Ok(Integer::from(raw).to_cell()),
            Primitive::Reference(t) => Ok(Reference::from(t.clone(), raw).to_cell()),
            Primitive::List(t) => Ok(ListValue::from(t.0.clone(), t.1, raw).to_cell())
        }
    }
}