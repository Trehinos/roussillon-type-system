use crate::types::concept::{DataType, Type};
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;

pub struct HeapType(Type);

impl HeapType {
    pub fn from(t: &Type) -> Self {
        HeapType(t.clone())
    }
}

impl DataType for HeapType {
    fn size(&self) -> usize { self.0.size() }

    fn typename(&self) -> String { format!("box<{}>", self.0.typename()) }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        self.0.construct_from_raw(raw)
    }
}