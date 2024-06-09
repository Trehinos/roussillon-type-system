//! A namespaced [Identifier] for named types like [crate::typing::typedef::Enumeration] and [crate::typing::typedef::Structure].
//!
//! The [Identified] trait helps to get information about the [Identifier] of a type. 

use std::fmt::{Display, Formatter};

/// An identifier with a namespace.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Identifier {
    pub space: String,
    pub name: String,
}

impl Identifier {
    pub fn new(from: &str) -> Self {
        from.split_once('/')
            .map(|(space, name)| Identifier { space: space.to_string(), name: name.to_string() })
            .unwrap_or_else(|| Identifier { space: String::new(), name: from.to_string() })
    }
    pub fn core(name: &str) -> Self {
        Self {
            space: "Core".to_string(),
            name: name.to_string(),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               if self.space.is_empty() {
                   self.name.to_string()
               } else { format!("{}/{}", self.space, self.name) },
        )
    }
}

pub trait Identified {
    fn identifier(&self) -> Identifier;
    fn name(&self) -> String { self.identifier().name }
    fn space(&self) -> String { self.identifier().space }
    fn fully_qualified_name(&self, current_namespace: String) -> String {
        if self.space().is_empty() {
            format!("{}/{}", current_namespace, self.identifier())
        } else {
            self.identifier().to_string()
        }
    }
}

/// A label to identify fields or members.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Label(String);

impl Label {
    pub fn new(name: &str) -> Self {
        Label(name.to_string())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

pub trait Labelled {
    fn label(&self) -> Label;
}