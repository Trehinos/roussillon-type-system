use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum EffectKind {
    Definition(String),
    Link(Rc<EffectKind>),
    Product(Vec<Rc<EffectKind>>),
}

impl Display for EffectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|",
               match self {
                   EffectKind::Definition(s) => s.to_string(),
                   EffectKind::Link(l) => l.to_string(),
                   EffectKind::Product(p) => p.iter().map(|k| k.to_string()).collect::<Vec<_>>().join("&"),
               }
        )
    }
}

#[derive(Clone, Debug)]
pub enum EffectLifetime {
    Global,
    WithinScope,
    FollowReturnValue,
}

#[derive(Clone, Debug)]
pub struct EffectType {
    pub lifetime: EffectLifetime,
    pub kind: EffectKind,
}

impl EffectType {
    pub fn name(&self) -> String { format!("<|{}|>", self.kind) }
}

impl Display for EffectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name()) }
}