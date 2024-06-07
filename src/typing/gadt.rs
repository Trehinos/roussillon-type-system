use crate::identify::{Identified, Identifier};
use crate::typing::concept::DataType;
use crate::typing::functional::FunctionType;
use crate::typing::sequence::SequenceType;
use crate::value::concept::ValueCell;
use crate::value::sequence::Sequence;

pub type GadtEvaluation = fn(gadt: &Gadt, tag: usize, arguments: &Sequence) -> Option<ValueCell>;

/// A Generalised Abstract DataType
#[derive(Clone, Debug)]
pub struct Gadt {
    identifier: Identifier,
    constructors: Vec<FunctionType>,
    evaluation: GadtEvaluation,
}

impl Gadt {
    pub fn new(identifier: Identifier, constructors: &[FunctionType], eval: GadtEvaluation) -> Self {
        Gadt { identifier, constructors: constructors.to_vec(), evaluation: eval }
    }

    pub fn to_sequence_type(&self) -> SequenceType {
        let mut sequence_type = SequenceType::new();
        for constructor in &self.constructors {
            sequence_type.push(constructor.clone().to_rc());
        }
        sequence_type
    }

    pub fn constructor_from_tag(&self, tag: usize) -> Option<&FunctionType> {
        self.constructors.get(tag)
    }

    pub fn checked_constructor(&self, tag: usize, arguments: &SequenceType) -> Option<FunctionType> {
        let constructor = self.constructor_from_tag(tag)?;
        let expected_type = constructor.arguments.typename();
        if arguments.typename() == expected_type {
            Some(constructor.clone())
        } else {
            None
        }
    }

    pub fn eval(&self, tag: usize, arguments: &Sequence) {
        (self.evaluation)(self, tag, arguments);
    }
}

impl DataType for Gadt {
    fn size(&self) -> usize { self.to_sequence_type().size() }

    fn typename(&self) -> String { format!("<{}>", self.identifier) }
}

impl Identified for Gadt {
    fn identifier(&self) -> Identifier { self.identifier.clone() }
}