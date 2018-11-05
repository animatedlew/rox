#[derive(Debug, Clone)]
pub enum LiteralType {
    Custom(String),
    Number(f64),
    Text(&'static str),
    Eof,
}
