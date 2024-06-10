//! This module provides a types feature.
//!
//! All types implement the [concept::DataType] trait.
//!
//! This module provides the following types :
//! - [primitive::Primitive::Boolean]
//! - [primitive::Primitive::Byte]
//! - [primitive::Primitive::Bytes]
//! - [primitive::Primitive::Integer]
//! - [primitive::Primitive::Float]
//! - [primitive::Primitive::List]
//! - [sequence::Tuple]
//! - [algebraic::ProductType]
//! - [algebraic::SumType]
//! - [typedef::Structure]
//! - [typedef::Enumeration]
//! - [functional::FunctionType]
//! 
//! ## Example
//! 
//! Create a "MyStruct" [typedef::Structure] with 3 fields :
//! ```
//! use roussillon_type_system::types::primitive::Primitive;
//! use roussillon_type_system::types::algebraic::ProductType;
//! use roussillon_type_system::types::typedef::Structure;
//!
//! let my_struct = Structure::new("MyStruct", ProductType::new(&[
//!     Primitive::Integer.to_rc(),
//!     Primitive::Integer.to_rc(),
//!     Primitive::Float.to_rc(),
//! ])).to_rc();
//! ```
//! 
//! This program creates the following my_struct object : 
//! ```-
//! Structure {
//!     identifier: Identifier {
//!         space: "",
//!         name: "MyStruct"
//!     },
//!     product_type: ProductType([<integer>, <integer>, <float>])
//! }
//! ```
//! 
//! 
//! 

pub mod concept;
pub mod primitive;
pub mod sequence;
pub mod algebraic;
pub mod typedef;
pub mod functional;