
use std::rc::Rc;
use crate::types::concept::Type;

pub struct KindSignature {
    pub from: Option<Kind>,
    pub to: Kind,
}

impl KindSignature {
    pub fn signature(&self) -> String {
        let mut str = String::new();
        if let Some(kind) = &self.from {
            
        }
        todo!()
    }
}

pub trait Effect {
    fn kind(&self) -> KindSignature;
    fn signature(&self) -> String;
}

pub enum EffectRows {
    Diverge,
    Except,
    Effect(Rc<dyn Effect>),
}

pub type Handler = fn(EffectRows, Type) -> Type;

pub enum Kind {
    ValueType(Type),
    Rows(EffectRows),
    Atomic(Rc<dyn Effect>),
    HeapType(Type),
    Handler(Handler),
    Predicate(bool),
    Scoped(Rc<Kind>),
}

impl Kind {
    pub fn signature(&self) -> String {
        match self {
            Kind::ValueType(t) => t.signature(),
            Kind::Rows(rows) => rows.signature(),
            Kind::Atomic(e) => e.signature(),
            Kind::HeapType(t) => t.signature(),
            Kind::Handler(h) => h.signature(),
            Kind::Predicate(b) => b.to_string(),
            Kind::Scoped(s) => s,
        }
    }
}

pub struct EffectTotal;
impl Effect for EffectTotal {
    fn kind(&self) -> KindSignature {
        todo!()
    }

    fn signature(&self) -> String {
        todo!()
    }
}