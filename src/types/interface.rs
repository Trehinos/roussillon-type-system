use crate::types::concept::Type;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::function::FunctionDefinition;
use crate::value::reference::Reference;
use crate::value::sequence::Sequence;

pub type Method = fn(this: Reference, Sequence) -> ValueCell;
pub type Constructor = fn(&Type, Sequence) -> ValueCell;

#[derive(Clone, Debug)]
pub struct Interface(Vec<FunctionDefinition>);

impl Interface {
    pub fn new(functions: &[FunctionDefinition]) -> Self { Interface(functions.to_vec()) }

    pub fn functions(&self) -> &[FunctionDefinition] { &self.0 }

    pub fn function(&self, index: usize) -> Option<&FunctionDefinition> { self.0.get(index) }
}


#[derive(Clone, Debug)]
pub struct Trait {
    pub ty: Type,
    pub with: Interface,
    vtable: Vec<Method>,
}

impl Trait {
    pub fn new(implements: Type, interface: Interface, vtable: &[Method]) -> Self {
        Self { ty: implements, with: interface, vtable: vtable.to_vec() }
    }

    pub fn create_raw_constructor(&self) -> Constructor {
        |ty, arguments| ty.construct_from_raw(&arguments.raw()).unwrap()
    }

    pub fn vtable(&self) -> &[Method] { &self.vtable }

    pub fn add_method(&mut self, method: Method) {
        self.vtable.push(method);
    }

    pub fn method(&self, index: usize) -> Option<&Method> { self.vtable.get(index) }
}