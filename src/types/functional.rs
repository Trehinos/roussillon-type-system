use std::rc::Rc;

use crate::identity::Identifier;
use crate::types::concept::{DataType, Type};
use crate::types::sequence::{join, Tuple};
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub arguments: Tuple,
    pub return_type: Type,
}

impl FunctionType {
    pub fn new(arguments: Tuple, return_type: Type) -> Self { Self { arguments, return_type } }
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for FunctionType {
    fn size(&self) -> usize { 8 }

    fn typename(&self) -> String {
        format!("fn<({}), {}>", join(&self.arguments, ","), self.return_type)
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub identifier: Identifier,
    pub signature: FunctionType,
}

impl FunctionDeclaration {
    pub fn new(identifier: Identifier, signature: FunctionType) -> Self {
        Self { identifier, signature }
    }
}
