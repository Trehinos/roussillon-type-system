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
//! use roussillon_type_system::factory::create_struct;
//! use roussillon_type_system::identity::LabelBank;
//! use roussillon_type_system::types::primitive::Primitive;
//!
//! let my_struct = create_struct("MyStruct", LabelBank::from(&[
//!     "field_a",
//!     "field_b",
//!     "field_c",
//! ]), &[
//!     Primitive::Integer.to_rc(),
//!     Primitive::Integer.to_rc(),
//!     Primitive::Float.to_rc(),
//! ]);
//! ```
//!
//! This program creates the following my_struct object : 
//! ```-
//! Structure {
//!     identifier: Identifier {
//!         space: "",
//!         name: "MyStruct"
//!     }, 
//!     labels: LabelBank({
//!         Label("field_a"): 0,
//!         Label("field_b"): 1,
//!         Label("field_c"): 2}
//!     ),
//!     product_type: ProductType([<integer>, <integer>, <float>])
//! }
//! ```
//!

pub mod concept;
pub mod primitive;
pub mod sequence;
pub mod algebraic;
pub mod typedef;
pub mod functional;
#[cfg(feature = "experiments")]
pub mod dynamic;
#[cfg(feature = "experiments")]
pub mod interface;
#[cfg(feature = "experiments")]
pub mod heap;