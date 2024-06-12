//! All values corresponding types in the module [crate::types].
//! 
//! - [byte::Bytes::Byte]
//! - [byte::Bytes::Arch]
//! - [byte::Bytes::Word]
//! - [byte::Bytes::Quad]
//! - [byte::Bytes::Long]
//! - [byte::Bytes::Wide]
//! - [byte::Bytes::Bytes]
//! - [number::Integer]
//! - [number::Float]
//! - [list::List]
//! - [sequence::Sequence]
//! - [record::ProductValue]
//! - [record::Record]
//! - [union::SumValue]
//! - [union::Union]


pub mod concept;
pub mod error;
pub mod sequence;
pub mod boolean;
pub mod byte;
pub mod number;
pub mod reference;
pub mod list;
pub mod union;
pub mod record;
pub mod function;
pub mod operations;