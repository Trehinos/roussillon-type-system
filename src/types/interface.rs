use std::rc::Rc;
use crate::value::function::FunctionDefinition;

#[derive(Clone, Debug)]
pub struct Interface(Vec<FunctionDefinition>);

impl Interface {
    pub fn new(functions: &[FunctionDefinition]) -> Self { Interface(functions.to_vec()) }

    pub fn functions(&self) -> &[FunctionDefinition] { &self.0 }

    pub fn function(&self, index: usize) -> Option<&FunctionDefinition> { self.0.get(index) }
    
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}