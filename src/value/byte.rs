use std::cell::RefCell;
use std::rc::Rc;
use std::{u64, usize};
use crate::parse::{parse_slice, Parsed};

use crate::types::concept::Type;
use crate::types::primitive::Primitive;
use crate::value::concept::{DataValue, GetDataValue, ValueCell};

#[derive(Copy, Clone, Debug, Default)]
pub struct Byte(u8);
impl Byte {
    pub fn new(from: u8) -> Self { Self(from) }
}

impl GetDataValue<u8> for Byte {
    fn get(&self) -> u8 { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(u8::from_be_bytes(raw.try_into().unwrap())) }
}

impl DataValue for Byte {
    fn data_type(&self) -> Type { Primitive::Byte.to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Word(u16);
impl Word {
    pub fn new(from: u16) -> Self { Self(from) }
}

impl GetDataValue<u16> for Word {
    fn get(&self) -> u16 { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(u16::from_be_bytes(raw.try_into().unwrap())) }
}
impl DataValue for Word {
    fn data_type(&self) -> Type { Primitive::Bytes(2).to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Quad(u32);
impl Quad {
    pub fn new(from: u32) -> Self { Self(from) }
}
impl GetDataValue<u32> for Quad {
    fn get(&self) -> u32 { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(u32::from_be_bytes(raw.try_into().unwrap())) }
}
impl DataValue for Quad {
    fn data_type(&self) -> Type { Primitive::Bytes(4).to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Long(u64);
impl Long {
    pub fn new(from: u64) -> Self { Self(from) }
}
impl GetDataValue<u64> for Long {
    fn get(&self) -> u64 { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(u64::from_be_bytes(raw.try_into().unwrap())) }
}
impl DataValue for Long {
    fn data_type(&self) -> Type { Primitive::Bytes(8).to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Wide(u128);
impl Wide {
    pub fn new(from: u128) -> Self { Self(from) }
}
impl GetDataValue<u128> for Wide {
    fn get(&self) -> u128 { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(u128::from_be_bytes(raw.try_into().unwrap())) }
}
impl DataValue for Wide {
    fn data_type(&self) -> Type { Primitive::Bytes(16).to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Arch(usize);
impl Arch {
    pub fn new(from: usize) -> Self { Self(from) }
    pub const fn size_of() -> usize { std::mem::size_of::<usize>() }
}
impl GetDataValue<usize> for Arch {
    fn get(&self) -> usize { self.0 }
    fn from_raw(raw: &[u8]) -> Self { Self(usize::from_be_bytes(raw.try_into().unwrap())) }
}
impl DataValue for Arch {
    fn data_type(&self) -> Type { Primitive::Bytes(Self::size_of()).to_rc() }
    fn raw(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }
    fn set(&mut self, raw: &[u8]) { *self = Self::from_raw(raw) }
}

#[derive(Clone, Debug)]
pub enum Bytes {
    Byte(Byte),
    Word(Word),
    Quad(Quad),
    Long(Long),
    Wide(Wide),
    Arch(Arch),
    Bytes(Vec<u8>, usize),
}

#[derive(Debug)]
pub struct CannotCreateArchWithGivenSize(pub usize);

impl Bytes {
    pub fn parse(input: &[u8], size: usize) -> Parsed<Self> {
        let (Some(raw), rest) = parse_slice(input, size) else { return (None, input); };
        (Some(Self::from_raw(raw)), rest)
    }

    pub fn try_from_arch(raw: &[u8]) -> Result<Self, CannotCreateArchWithGivenSize> {
        if raw.len() == Arch::size_of() {
            Ok(Self::Arch(Arch::from_raw(raw)))
        } else {
            Err(CannotCreateArchWithGivenSize(raw.len()))
        }
    }

    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}
impl GetDataValue<Vec<u8>> for Bytes {
    fn get(&self) -> Vec<u8> {
        match self {
            Bytes::Byte(v) => vec![v.get()],
            Bytes::Word(v) => v.get().to_be_bytes().to_vec(),
            Bytes::Quad(v) => v.get().to_be_bytes().to_vec(),
            Bytes::Long(v) => v.get().to_be_bytes().to_vec(),
            Bytes::Wide(v) => v.get().to_be_bytes().to_vec(),
            Bytes::Arch(v) => v.get().to_be_bytes().to_vec(),
            Bytes::Bytes(b, _) => b.to_vec(),
        }
    }

    fn from_raw(raw: &[u8]) -> Self {
        if raw.len() == Arch::size_of() {
            return Self::try_from_arch(raw).unwrap();
        }
        match raw.len() {
            1 => Self::Byte(Byte::from_raw(raw)),
            2 => Self::Word(Word::from_raw(raw)),
            4 => Self::Quad(Quad::from_raw(raw)),
            8 => Self::Long(Long::from_raw(raw)),
            16 => Self::Wide(Wide::from_raw(raw)),
            l => Self::Bytes(raw.to_vec(), l)
        }
    }
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
            Bytes::Byte(b) => b.raw(),
            Bytes::Arch(a) => a.raw(),
            Bytes::Word(w) => w.raw(),
            Bytes::Quad(q) => q.raw(),
            Bytes::Long(l) => l.raw(),
            Bytes::Wide(w) => w.raw(),
            Bytes::Bytes(b, _) => b.to_vec(),
        }
    }

    fn set(&mut self, raw: &[u8]) {
        *self = Self::from_raw(raw);
    }
}