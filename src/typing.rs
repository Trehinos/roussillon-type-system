//! This module provides a typing feature.
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
//! - [sequence::SequenceType]
//! - [algebraic::ProductType]
//! - [algebraic::SumType]
//! - [typedef::Structure]
//! - [typedef::Enumeration]

pub mod concept;
pub mod sequence;
pub mod algebraic;
pub mod primitive;
pub mod typedef;
pub mod functional;