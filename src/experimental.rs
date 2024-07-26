use crate::types::concept::Type;

pub type Constructor = fn() -> Type;
pub type Converter = fn(Type) -> Type;
pub type Typer = fn(Kind) -> Type;
pub type Kinder = fn(Type) -> Kind;
pub type Application = fn(Kind) -> Kind;

#[derive(Clone, Debug)]
pub enum Kind {
    Nothing,
    Type(Type),
    Constructor(Constructor),
    Converter(Converter),
    Typer(Typer),
    Kinder(Kinder),
    Application(Application),
}

#[derive(Clone, Debug)]
pub enum KindType {
    Nothing,
    Type(Type),
    Kind(Kind),
}

impl Kind {
    pub fn pattern(&self) -> String {
        match self {
            Kind::Nothing => "•".to_string(),
            Kind::Type(t) => format!("<{}>", t.typename()),
            Kind::Constructor(c) => format!("<()> -> <{}>", c().typename()),
            Kind::Converter(_) => "<T1> -> <T2>".to_string(),
            Kind::Typer(_) => "K -> <T>".to_string(),
            Kind::Kinder(_) => "<T> -> K".to_string(),
            Kind::Application(_) => "K1 -> K2".to_string(),
        }
    }
    pub fn typename(&self, argument: KindType) -> String {
        match self.get_type(argument) {
            KindType::Nothing => "•".to_string(),
            KindType::Type(t) => format!("<{}>", t.typename()),
            KindType::Kind(k) => k.typename(KindType::Nothing),
        }  
    }


    pub fn get_type(&self, argument: KindType) -> KindType {
        match self {
            Kind::Nothing => KindType::Nothing,
            Kind::Type(t) => KindType::Type(t.clone()),
            Kind::Constructor(c) => KindType::Type(c()),
            Kind::Converter(c) => {
                if let KindType::Type(t) = argument { KindType::Type(c(t)) } else { KindType::Nothing }
            }
            Kind::Typer(t) => {
                if let KindType::Kind(k) = argument { KindType::Type(t(k)) } else { KindType::Nothing }
            }
            Kind::Kinder(k) => {
                if let KindType::Type(t) = argument { KindType::Kind(k(t)) } else { KindType::Nothing }
            }
            Kind::Application(a) => {
                if let KindType::Kind(k) = argument { KindType::Kind(a(k)) } else { KindType::Nothing }
            }
        }
    }
}