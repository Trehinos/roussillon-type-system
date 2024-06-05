use crate::identify::Identifier;
use crate::typing::concept::{DataType, Type};
use crate::typing::sequence::{join, SequenceType};

#[derive(Clone, Debug)]
pub struct FunctionType {
    arguments: SequenceType,
    return_type: Type,
}

impl FunctionType {
    pub fn new(arguments: SequenceType, return_type: Type) -> Self {
        FunctionType { arguments, return_type }
    }
}

impl DataType for FunctionType {
    fn size(&self) -> usize { 8 }

    fn typename(&self) -> String {
        format!("fn({}) -> {}", join(&self.arguments, ","), self.return_type)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    identifier: Identifier,
    signature: FunctionType,
    execution: fn(),
}
