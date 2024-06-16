use std::cell::RefCell;
use std::rc::Rc;
use crate::identity::{Label, Labelled};

use crate::types::concept::Type;
use crate::types::algebraic::ProductType;
use crate::types::typedef::Structure;

use crate::value::concept::{DataValue, ValueCell};
use crate::value::error::TypeResult;
use crate::value::sequence::Sequence;
use crate::value::union::Union;

#[derive(Clone, Debug)]
pub struct ProductValue {
    product: Rc<ProductType>,
    value: Sequence,
}

impl ProductValue {
    pub fn new(product: Rc<ProductType>, values: &[ValueCell]) -> TypeResult<Self> {
        let sequence = product.to_tuple();
        Ok(ProductValue { product, value: Sequence::new(sequence, values)? })
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

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }

    pub fn from(product: Rc<ProductType>, raw: &[u8]) -> TypeResult<Self> {
        Ok(ProductValue { product: product.clone(), value: Sequence::from(product.to_tuple(), raw)? })
    }
}

impl DataValue for ProductValue {
    fn data_type(&self) -> Type { self.product.clone() }

    fn raw(&self) -> Vec<u8> { self.value.raw() }

    fn set(&mut self, raw: &[u8]) {
        self.value.set(raw);
    }
}

#[derive(Clone, Debug)]
pub struct Record {
    of_type: Rc<Structure>,
    value: ProductValue,
}

impl Record {
    pub fn new(structure_type: Rc<Structure>, values: &[ValueCell]) -> TypeResult<Self> {
        Ok(Record {
            of_type: structure_type.clone(),
            value: ProductValue::new(Rc::new(structure_type.product_type.clone()), values)?,
        })
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }

    pub fn to_sequence(self) -> Sequence { self.value.to_sequence() }

    pub fn clone_sequence(&self) -> Sequence { self.value.clone().to_sequence() }

    pub fn as_sequence(&self) -> &Sequence { self.value.as_sequence() }

    pub fn get_field(&self, field: usize) -> Option<ValueCell> { self.value.get_element(field) }

    pub fn field_from_name(&self, field: &str) -> Option<ValueCell> { self.labelled(&Label::new(field)) }

    pub fn from(structure_type: Rc<Structure>, raw: &[u8]) -> TypeResult<Self> {
        let product = structure_type.product_type.clone().to_rc();
        Ok(
            Self {
                of_type: structure_type,
                value: ProductValue::from(product, raw)?,
            }
        )
    }
}

impl DataValue for Record {
    fn data_type(&self) -> Type { self.of_type.clone() }

    fn raw(&self) -> Vec<u8> { self.value.raw() }

    fn set(&mut self, raw: &[u8]) {
        self.value.set(raw);
    }
}

impl Labelled<ValueCell> for Record {
    fn labelled(&self, label: &Label) -> Option<ValueCell> {
        self.get_field(self.of_type.labels.labelled(label)?)
    }
}
