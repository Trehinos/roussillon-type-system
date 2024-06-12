use crate::types::concept::Type;
use crate::value::concept::ValueCell;
use crate::value::error::TypeResult;

pub fn copy(result_type: Type, from: &ValueCell) -> TypeResult<ValueCell> {
    result_type.construct_from_raw(&from.borrow().raw())
}