use std::rc::Rc;
use crate::identity::Label;
use crate::types::concept::{DataType, Type};
use crate::types::dynamic::Dynamic;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::TypeResult;

pub struct ValueType {
    pub label: Label,
    pub value: Dynamic,
}

impl DataType for ValueType {
    fn size(&self) -> usize { self.value.size() }

    fn typename(&self) -> String { format!("type<{}>", self.value.typename()) }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        self.value.construct_from_raw(raw)
    }
}

impl DataValue for ValueType {
    fn data_type(&self) -> Type { Rc::new(self.value.clone()) }
    fn raw(&self) -> Vec<u8> { vec![] }
    fn set(&mut self, raw: &[u8]) {}
}

impl ValueType {
    pub fn type_of(label: Label, cell: ValueCell) -> Self {
        let value = cell.borrow();
        ValueType {
            label,
            value: Dynamic::Defined(value.data_type()),
        }
    }
}