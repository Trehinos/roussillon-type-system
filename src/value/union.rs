use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::types::concept::Type;
use crate::types::algebraic::SumType;
use crate::types::typedef::Enumeration;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::{CanTypeError, SumTypeError, TypeResult};

#[derive(Clone, Debug)]
pub struct SumValue {
    pub sum: Rc<SumType>,
    pub value: ValueCell,
    pub tag: usize,
}

impl SumValue {
    pub fn new(sum: Rc<SumType>, tag: usize, value: ValueCell) -> TypeResult<Self> {
        let provided_type = value.borrow().data_type();
        if let Some(expected_type) = sum.variant(tag) {
            if expected_type.typename() == provided_type.typename() {
                Ok(Self { sum, value, tag })
            } else {
                Err(SumTypeError::InvalidTag {
                    expected_type,
                    provided_type,
                    provided_tag: tag,
                }.promote())
            }
        } else {
            Err(SumTypeError::InvalidCase { provided_type }.promote())
        }
    }

    pub fn current_type(&self) -> Type {
        self.sum.to_tuple()[self.tag].clone()
    }

    pub fn current_value(&self) -> &ValueCell { &self.value }

    pub fn tag(&self) -> usize { self.tag }

    pub fn set_cell(&mut self, tag: usize, new_value: ValueCell) -> CanTypeError {
        *self = Self::new(self.sum.clone(), tag, new_value)?;
        Ok(())
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }

    pub fn from(t: Rc<SumType>, raw: &[u8]) -> TypeResult<Self> {
        let (tag_bytes, raw_value) = raw.split_at(8);
        let tag = usize::from_be_bytes(tag_bytes.try_into().unwrap());
        if let Some(variant_type) = t.variant(tag) {
            let value = variant_type.construct_from_raw(&raw_value)?;
            Ok(Self::new(t.clone(), tag, value)?)
        } else {
            Err(SumTypeError::InvalidCase { provided_type: t.clone() }.promote())
        }
    }
}

impl DataValue for SumValue {
    fn data_type(&self) -> Type { self.sum.clone() }

    fn raw(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend(self.tag.to_be_bytes());
        raw.extend(self.value.borrow().raw());
        raw
    }

    fn set(&mut self, raw: &[u8]) {
        self.value.borrow_mut().set(raw)
    }
}


#[derive(Clone)]
pub struct Union {
    of_type: Rc<Enumeration>,
    value: SumValue,
}

impl Union {
    pub fn new(union: Rc<Enumeration>, tag: usize, value: ValueCell) -> TypeResult<Self> {
        Ok(Self { of_type: union.clone(), value: SumValue::new(union.sum_type.clone().to_rc(), tag, value)? })
    }
    pub fn current_type(&self) -> Type { self.value.current_type() }
    pub fn current_value(&self) -> &ValueCell { self.value.current_value() }
    pub fn tag(&self) -> usize {
        self.value.tag
    }
    pub fn set_cell(&mut self, tag: usize, new_value: ValueCell) -> CanTypeError {
        self.value.set_cell(tag, new_value)
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
    
    pub fn from(t: Rc<Enumeration>, raw: &[u8]) -> TypeResult<Self> {
        Ok(Self{ of_type: t.clone(), value: SumValue::from(t.sum_type.clone().to_rc(), raw)? })
    }
}

impl DataValue for Union {
    fn data_type(&self) -> Type { self.of_type.clone() }

    fn raw(&self) -> Vec<u8> { self.value.raw() }

    fn set(&mut self, raw: &[u8]) { self.value.set(raw) }
}

impl Debug for Union {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}:{}>{:x?}", self.data_type(), self.current_type().typename(), self.raw())
    }
}
