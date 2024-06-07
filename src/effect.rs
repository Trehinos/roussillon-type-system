
pub enum Effect {
    Name(String),
    Alias(Vec<Effect>),
}