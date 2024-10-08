use std::rc::Rc;
use crate::identity::LabelBank;
use crate::types::algebraic::ProductType;
use crate::types::concept::Type;
use crate::types::typedef::Structure;
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;



pub fn create_struct(identifier: &str, labels: LabelBank, members: &[Type]) -> Rc<Structure> {
    Structure::new(identifier, labels, ProductType::new(members)).to_rc()
}

pub fn copy_value(result_type: Type, from: &ValueCell) -> TypeResult<ValueCell> {
    result_type.construct_from_raw(&from.borrow().raw())
}