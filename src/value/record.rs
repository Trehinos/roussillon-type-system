use std::cell::RefCell;
use std::rc::Rc;

use crate::typing::concept::Type;
use crate::typing::algebraic::ProductType;
use crate::typing::typedef::Structure;

use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::TypeResult;
use crate::value::sequence::Sequence;

#[derive(Clone, Debug)]
pub struct Product {
    product: Rc<ProductType>,
    value: Sequence,
}

impl Product {
    pub fn new(product: Rc<ProductType>, values: &[ValueCell]) -> TypeResult<Self> {
        let sequence = product.to_sequence_type();
        Ok(Product { product, value: Sequence::new(sequence, values)? })
    }

    pub fn to_sequence(self) -> Sequence { self.value }

    pub fn clone_sequence(&self) -> Sequence { self.value.clone() }

    pub fn as_sequence(&self) -> &Sequence { &self.value }

    pub fn get_element(&self, nth: usize) -> Option<ValueCell> {
        let fields = self.value.get();
        if nth > fields.len() {
            None
        } else {
            Some(fields[nth].clone())
        }
    }
}

impl DataValue for Product {
    fn data_type(&self) -> Type { self.product.clone() }

    fn raw(&self) -> Vec<u8> { self.value.raw() }

    fn set(&mut self, raw: &[u8]) {
        self.value.set(raw);
    }
}

#[derive(Clone, Debug)]
pub struct Record {
    of_type: Rc<Structure>,
    value: Product,
}

impl Record {
    pub fn new(structure_type: Rc<Structure>, values: &[ValueCell]) -> TypeResult<Self> {
        Ok(Record {
            of_type: structure_type.clone(),
            value: Product::new(Rc::new(structure_type.product_type.clone()), values)?,
        })
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }

    pub fn to_sequence(self) -> Sequence { self.value.to_sequence() }

    pub fn clone_sequence(&self) -> Sequence { self.value.clone().to_sequence() }

    pub fn as_sequence(&self) -> &Sequence { self.value.as_sequence() }

    pub fn get_field(&self, field: usize) -> Option<ValueCell> { self.value.get_element(field) }
}

impl DataValue for Record {
    fn data_type(&self) -> Type { self.of_type.clone() }

    fn raw(&self) -> Vec<u8> { self.value.raw() }

    fn set(&mut self, raw: &[u8]) {
        self.value.set(raw);
    }
}