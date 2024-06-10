use crate::types::concept::Type;

#[derive(Clone, Debug)]
pub enum SumTypeError {
    InvalidTag {
        expected_type: Type,
        provided_type: Type,
        provided_tag: usize,
    },
    InvalidCase {
        provided_type: Type,
    },
}

#[derive(Clone, Debug)]
pub enum SequenceError {
    SequenceLengthMismatch {
        expected: usize,
        provided: usize,
    },
}

#[derive(Clone, Debug)]
pub enum TypeError {
    SumTypeError(SumTypeError),
    ProductTypeError(SequenceError),
    InvalidType {
        expected: Type,
        provided: Type
    },
    Message(String),
}

pub type TypeResult<T> = Result<T, TypeError>;
pub type CanTypeError = TypeResult<()>;

impl SumTypeError {
    pub fn promote(self) -> TypeError {
        TypeError::SumTypeError(self)
    }
}
impl SequenceError {
    pub fn promote(self) -> TypeError {
        TypeError::ProductTypeError(self)
    }
}