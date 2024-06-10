//! # Roussillon : Type System
//!
//! This crate provides some structs and traits to manage types and typed values.
//!
//! This crate goal **IS NOT** to manage memory allocation.
//!
//! ## Primitive types
//!
//! These are the necessary types to construct any other meaningful type.
//! Usually, these are "machine types" :
//!
//! - [types::primitive::Primitive::Boolean] type of [value::boolean::Boolean],
//! - [types::primitive::Primitive::Byte] type of [value::byte::Bytes::Byte],
//! - [types::primitive::Primitive::Bytes] type of :
//!     - [value::byte::Bytes::Arch],
//!     - [value::byte::Bytes::Word],
//!     - [value::byte::Bytes::Quad],
//!     - [value::byte::Bytes::Long],
//!     - [value::byte::Bytes::Wide], and
//!     - [value::byte::Bytes::Bytes],
//! - [types::primitive::Primitive::Float] type of [value::number::Float],
//! - [types::primitive::Primitive::Integer] type of [value::number::Integer],
//! - [types::primitive::Primitive::Reference] type of [value::reference::Reference],
//! - [types::primitive::Primitive::List] type of [value::list::List],
//!
//! ## The "tuple" type
//!
//! A sequence of heterogeneous typed values.
//!
//! - [types::sequence::Tuple] type of [value::sequence::Sequence],
//!
//! ## Algebraic Data Types
//!
//! - [types::algebraic::SumType] type of [value::union::SumValue],
//! - [types::algebraic::ProductType] type of [value::record::ProductValue],
//!
//! ## Custom types
//!
//! These types are [identity::Identified] ADTs with [identity::Labelled] fields.
//!
//! - [types::typedef::Enumeration] type of [value::union::Union],
//! - [types::typedef::Structure] type of [value::record::Record],
//!
//! ## Functional
//!
//! - [types::functional::FunctionType] type of [types::functional::FunctionDeclaration] composed in [value::function::FunctionDefinition] with [value::function::FunctionBody].


pub mod identity;
mod parse;
pub mod types;
#[cfg(feature = "effects")]
pub mod effect;
pub mod value;

#[cfg(test)]
mod tests {
    use crate::types::algebraic::{ProductType};
    use crate::types::concept::DataType;
    use crate::types::primitive::Primitive;
    use crate::types::typedef::Structure;
    use crate::value::concept::DataValue;
    use crate::value::number::{Float, Integer};
    use crate::value::record::Record;

    #[test]
    fn test() {
        let my_struct = Structure::new("MyStruct", ProductType::new(&[
            Primitive::Integer.to_rc(),
            Primitive::Integer.to_rc(),
            Primitive::Float.to_rc(),
        ]));
        println!("\n{:?}", my_struct);

        let object = Record::new(my_struct.clone().to_rc(), &[
            Integer::new(40).to_cell(),
            Integer::new(96).to_cell(),
            Float::new(40.0).to_cell()
        ]).unwrap();
        println!("\n{:?}", object.clone().to_cell().borrow());
        for i in 0..3 {
            let field = object.get_field(i).unwrap();
            println!("\n{:?}", field.borrow());
        }

        let copy = Structure::construct_from_raw(&my_struct, &object.raw()).unwrap();
        assert_eq!(object.data_type().typename(), copy.borrow().data_type().typename());
    }

}
