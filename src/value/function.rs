use crate::typing::functional::FunctionDeclaration;
use crate::value::concept::ValueCell;
use crate::value::sequence::Sequence;

pub type FunctionBody = fn(&Sequence) -> ValueCell;

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub declaration: FunctionDeclaration,
    execution: FunctionBody,
}

impl FunctionDefinition {
    pub fn new(declaration: FunctionDeclaration, execution: fn(&Sequence) -> ValueCell) -> Self {
        Self { declaration, execution }
    }
    
    pub fn call(&self, arguments: &Sequence) -> ValueCell {
        (self.execution)(arguments)
    }
}