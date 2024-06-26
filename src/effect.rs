use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub trait ToRc where Self: Sized {
    fn to_rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}

#[derive(Clone, Debug)]
pub enum EffectKind {
    Definition(String),
    Link(Rc<EffectKind>),
    Product(Vec<Rc<EffectKind>>),
}

impl Display for EffectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   EffectKind::Definition(s) => s.to_string(),
                   EffectKind::Link(l) => l.to_string(),
                   EffectKind::Product(p) => p.iter().map(|k| k.to_string()).collect::<Vec<_>>().join("*"),
               }
        )
    }
}

impl ToRc for EffectKind {}

impl EffectKind {
    pub fn to_link(self) -> Rc<Self> { self.to_rc() }

    pub fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            (EffectKind::Definition(s1), EffectKind::Definition(s2)) => {
                if s1 == s2 {
                    self.clone()
                } else {
                    EffectKind::Product(vec![Rc::new(self.clone()), Rc::new(other.clone())])
                }
            }
            _ => EffectKind::Product(vec![Rc::new(self.clone()), Rc::new(other.clone())]),
        }
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
    pub fn new(lifetime: EffectLifetime, from: &str) -> Self { Self { lifetime, kind: EffectKind::Definition(from.to_string()) } }
    pub fn link(lifetime: EffectLifetime, from: Rc<EffectKind>) -> Self { Self { lifetime, kind: EffectKind::Link(from) } }
    pub fn product(lifetime: EffectLifetime, from: &[Rc<EffectKind>]) -> Self { Self { lifetime, kind: EffectKind::Product(from.to_vec()) } }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            lifetime: self.lifetime.clone(),
            kind: self.kind.merge(&other.kind),
        }
    }
}

impl Display for EffectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name()) }
}