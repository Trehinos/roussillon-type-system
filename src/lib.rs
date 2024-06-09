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
//! - [typing::primitive::Primitive::Boolean] type of [value::boolean::Boolean],
//! - [typing::primitive::Primitive::Byte] type of [value::byte::Bytes::Byte],
//! - [typing::primitive::Primitive::Bytes] type of :
//!     - [value::byte::Bytes::Word],
//!     - [value::byte::Bytes::Quad],
//!     - [value::byte::Bytes::Long],
//!     - [value::byte::Bytes::Wide], and
//!     - [value::byte::Bytes::Bytes],
//! - [typing::primitive::Primitive::Float] type of [value::number::Float],
//! - [typing::primitive::Primitive::Integer] type of [value::number::Integer],
//! - [typing::primitive::Primitive::Reference] type of [value::reference::Reference],
//! - [typing::primitive::Primitive::List] type of [value::list::List],
//!
//! ## The "tuple" type
//!
//! A sequence of heterogeneous typed values.
//!
//! - [typing::sequence::Tuple] type of [value::sequence::Sequence],
//!
//! ## Algebraic Data Types
//!
//! - [typing::algebraic::SumType] type of [value::union::SumValue],
//! - [typing::algebraic::ProductType] type of [value::record::ProductValue],
//!
//! ## Custom types
//!
//! These types are [identify::Identified] ADTs with [identify::Labelled] fields.
//!
//! - [typing::typedef::Enumeration] type of [value::union::Union],
//! - [typing::typedef::Structure] type of [value::record::Record],
//!
//! ### Functional
//!
//! - [typing::functional::FunctionType] type of [typing::functional::FunctionDeclaration] composed in [value::function::FunctionDefinition] with [value::function::FunctionBody].


pub mod identify;
pub mod typing;
pub mod effect;
pub mod value;

#[cfg(test)]
mod tests {
    use crate::typing::algebraic::{ProductType, SumType};
    use crate::typing::concept::DataType;
    use crate::typing::functional::FunctionType;
    use crate::typing::primitive::Primitive;
    use crate::typing::typedef::{Enumeration, Structure};
    use crate::value::number::{Float, Integer};
    use crate::value::record::Record;
    use crate::value::union::Union;

    #[test]
    fn test() {
        let my_struct = Structure::new("MyStruct", ProductType::new(&[
            Primitive::Integer.to_rc(),
            Primitive::Integer.to_rc(),
            Primitive::Float.to_rc(),
        ])).to_rc();
        println!("\n{:?}", my_struct);

        let object = Record::new(my_struct.clone(), &[
            Integer::new(40).to_cell(),
            Integer::new(96).to_cell(),
            Float::new(40.0).to_cell()
        ]).unwrap();
        println!("\n{:?}", object.clone().to_cell().borrow());


        let x = 3 + 3;
        for i in 0..3 {
            let field = object.get_field(i).unwrap();
            println!("\n{:?}", field.borrow());
        }

        let some = Structure::new("Some", ProductType::new(&[my_struct.clone()])).to_rc();
        let none = Structure::new("None", ProductType::unit_type()).to_rc();
        let option_class = Enumeration::new("Option", SumType::new(&[
            none.clone(),
            some.clone()
        ])).to_rc();

        let mut union_object = Union::new(option_class, 1, Record::new(
            some.clone(),
            &[object.clone().to_cell()],
        ).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());

        union_object.set_cell(0, Record::new(none, &[]).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());

        union_object.set_cell(1, Record::new(some, &[object.to_cell()]).unwrap().to_cell()).unwrap();
        println!("\n{:?}", union_object);
        println!("\n{:?}", union_object.current_value().borrow());

        let constructor = FunctionType::new(my_struct.product_type.to_sequence_type(), my_struct);
        println!("\nConstructor type : {}", constructor.typename())
    }

}
