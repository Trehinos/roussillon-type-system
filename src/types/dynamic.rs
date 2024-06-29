use crate::types::concept::Type;

pub enum Dynamic {
    Unknown,
    Guessed(String),
    Defined(Type),
}