use crate::typing::concept::{DataType, Type};

pub type SequenceType = Vec<Type>;

pub fn sum_size(record: &SequenceType) -> usize {
    fn_size(record, |a, b| a + b)
}

pub fn fn_size(record: &SequenceType, func: fn(acc: usize, item: usize) -> usize) -> usize {
    record.iter()
        .map(|t| t.size())
        .reduce(func).unwrap_or_default()
}

pub fn join(record: &SequenceType, with: &str) -> String {
    record.iter()
        .map(|t| t.typename()).collect::<Vec<_>>()
        .join(with).to_string()
}

impl DataType for SequenceType {
    fn size(&self) -> usize {
        sum_size(self)
    }

    fn typename(&self) -> String {
        join(self, ",")
    }
}
