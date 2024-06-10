use std::cell::RefCell;
use std::rc::Rc;
use crate::parse::{parse_slice, Parsed};

use crate::types::concept::Type;
use crate::types::primitive::Primitive;
use crate::value::concept::{DataValue, ValueCell};

#[derive(Clone, Debug)]
pub enum Bytes {
    Byte(u8),
    Word(u16),
    Quad(u32),
    Long(u64),
    Wide(u128),
    Arch(usize),
    Bytes(Vec<u8>, usize),
}

impl Bytes {
    pub fn parse(input: &[u8], size: usize) -> Parsed<Self> {
        let (Some(raw), rest) = parse_slice(input, size) else { return (None, input); };
        (Some(Self::from(raw)), rest)
    }

    pub fn from(raw: &[u8]) -> Self {
        match raw.len() {
            1 => Self::Byte(u8::from_be_bytes(raw.try_into().unwrap())),
            2 => Self::Word(u16::from_be_bytes(raw.try_into().unwrap())),
            4 => Self::Quad(u32::from_be_bytes(raw.try_into().unwrap())),
            8 => Self::Long(u64::from_be_bytes(raw.try_into().unwrap())),
            16 => Self::Wide(u128::from_be_bytes(raw.try_into().unwrap())),
            l => Self::Bytes(raw.to_vec(), l)
        }
    }

    pub fn try_from_arch(raw: &[u8]) -> Result<Self, ()> {
        if raw.len() == std::mem::size_of::<usize>() {
            Ok(Self::Arch(usize::from_be_bytes(raw.try_into().unwrap())))
        } else {
            Err(())
        }
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}

impl DataValue for Bytes {
    fn data_type(&self) -> Type {
        match self {
            Bytes::Byte(_) => Primitive::Byte,
            Bytes::Arch(_) => Primitive::Bytes(std::mem::size_of::<usize>()),
            Bytes::Word(_) => Primitive::Bytes(2),
            Bytes::Quad(_) => Primitive::Bytes(4),
            Bytes::Long(_) => Primitive::Bytes(8),
            Bytes::Wide(_) => Primitive::Bytes(16),
            Bytes::Bytes(_, l) => Primitive::Bytes(*l),
        }.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        match self {
            Bytes::Byte(b) => b.to_be_bytes().to_vec(),
            Bytes::Arch(a) => a.to_be_bytes().to_vec(),
            Bytes::Word(w) => w.to_be_bytes().to_vec(),
            Bytes::Quad(q) => q.to_be_bytes().to_vec(),
            Bytes::Long(l) => l.to_be_bytes().to_vec(),
            Bytes::Wide(w) => w.to_be_bytes().to_vec(),
            Bytes::Bytes(b, _) => b.to_vec(),
        }
    }

    fn set(&mut self, raw: &[u8]) {
        *self = Self::from(raw);
    }
}