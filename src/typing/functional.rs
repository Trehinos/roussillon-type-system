use std::rc::Rc;

use crate::identify::Identifier;
use crate::typing::concept::{DataType, Type};
use crate::typing::sequence::{join, SequenceType};
use crate::value::concept::ValueCell;
use crate::value::sequence::Sequence;

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub arguments: SequenceType,
    pub return_type: Type,
}

impl FunctionType {
    pub fn new(arguments: SequenceType, return_type: Type) -> Self { Self { arguments, return_type } }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for FunctionType {
    fn size(&self) -> usize { 8 }

    fn typename(&self) -> String {
        format!("fn({}) -> {}", join(&self.arguments, ","), self.return_type)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub identifier: Identifier,
    pub signature: FunctionType,
    execution: fn(&Sequence) -> ValueCell,
}

impl FunctionDefinition {
    pub fn new(identifier: Identifier, signature: FunctionType, execution: fn(&Sequence) -> ValueCell) -> Self {
        Self { identifier, signature, execution }
    }
    pub fn call(&self, arguments: &Sequence) -> ValueCell {
        (self.execution)(arguments)
    }
}
