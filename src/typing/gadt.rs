use crate::identify::Identifier;
use crate::typing::concept::DataType;
use crate::typing::functional::FunctionDefinition;
use crate::typing::sequence::SequenceType;
use crate::value::concept::{DataValue, ValueCell};
use crate::value::sequence::Sequence;

/// A Generalised Abstract DataType
#[derive(Clone, Debug)]
pub struct Gadt {
    identifier: Identifier,
    constructors: Vec<FunctionDefinition>,
}

impl Gadt {
    pub fn new(identifier: Identifier, constructors: &[FunctionDefinition]) -> Self {
        Gadt { identifier, constructors: constructors.to_vec() }
    }

    pub fn to_sequence_type(&self) -> SequenceType {
        let mut sequence_type = SequenceType::new();
        for constructor in &self.constructors {
            let constructor_type = constructor.declaration.signature.clone();
            sequence_type.push(constructor_type.to_rc());
        }
        sequence_type
    }

    pub fn get_constructor(&self, tag: usize) -> Option<&FunctionDefinition> {
        self.constructors.get(tag)
    }

    // TODO : move to another location (Gadt "const value")
    pub fn apply(&self, tag: usize, arguments: &Sequence) -> Option<ValueCell> {
        let constructor = self.get_constructor(tag)?;
        let expected_type = constructor.declaration.signature.arguments.typename();
        if arguments.data_type().typename() == expected_type {
            Some(constructor.call(arguments))
        } else {
            None
        }
    }
}

impl DataType for Gadt {
    fn size(&self) -> usize { self.to_sequence_type().size() }

    fn typename(&self) -> String { self.identifier.to_string() }
}