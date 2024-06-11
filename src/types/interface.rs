use crate::types::concept::Type;
use crate::types::functional::FunctionType;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::sequence::Sequence;

pub type Method = fn(&Type, Sequence) -> ValueCell;

#[derive(Clone, Debug)]
pub struct Trait {
    pub implements: Type,
    interface: Vec<FunctionType>,
    vtable: Vec<Method>,
}

impl Trait {
    pub fn raw_constructor(&self) -> Method {
        (|ty, arguments| {
            ty.construct_from_raw(&arguments.raw()).unwrap()
        })
    }
}